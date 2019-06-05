#![allow(dead_code)]

use std::env;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use cc;
use tempdir::TempDir;

/// This struct contains all that is required for running tests with a set of executables in a
/// temporary directory.
pub struct Test {
    /// This temporary directory is created when the `Test` is created and is dropped when the
    /// `Test` is dropped.
    dir: TempDir,
    /// The path to a stub executable that doesn't do anything else other than write its arguments
    /// to a file.
    stub: PathBuf,
    msvc: bool,
}

pub struct Execution {
    args: Vec<String>,
}

impl Test {
    fn new() -> Test {
        let mut stub = PathBuf::from(env::current_exe().unwrap());
        stub.pop();
        if stub.ends_with("deps") {
            stub.pop();
        }
        // TODO: Rename gcc-shim to stub
        stub.push(format!("gcc-shim{}", env::consts::EXE_SUFFIX));
        Test {
            dir: TempDir::new("gcc-test").unwrap(),
            stub: stub,
            msvc: false,
        }
    }

    /// Use the "traditional" toolchain usually available on Unix systems.
    pub fn trad() -> Test {
        let t = Test::new();
        t.stub("cc").stub("c++").stub("ar");
        t
    }

    /// Use the GNU toolchain.
    pub fn gnu() -> Test {
        let t = Test::new();
        t.stub("gcc").stub("g++").stub("ar");
        t
    }

    /// Use the Clang/LLVM toolchain.
    pub fn clang() -> Test {
        let t = Test::new();
        t.stub("clang").stub("clang++").stub("ar");
        t
    }

    /// Use the Visual Studio toolchain.
    pub fn msvc() -> Test {
        let mut t = Test::new();
        t.stub("cl").stub("lib.exe");
        t.msvc = true;
        t
    }

    /// The path of the test directory.
    pub fn dir(&self) -> &Path {
        self.dir.path()
    }

    /// Create a stub executable.
    pub fn stub(&self, name: &str) -> &Test {
        let fname = format!("{}{}", name, env::consts::EXE_SUFFIX);
        fs::hard_link(&self.stub, self.dir().join(&fname))
            .or_else(|_| fs::copy(&self.stub, self.dir().join(&fname)).map(|_| ()))
            .unwrap();
        self
    }

    /// Initialize a `cc::Build` with the default host and target of `x86_64-unknown-linux-gnu` as
    /// well as other default options.
    pub fn default(&self) -> cc::Build {
        self.target("x86_64-unknown-linux-gnu")
    }

    /// Initialize a `cc::Build` with the same host and target as well as other default options.
    pub fn target(&self, target: &str) -> cc::Build {
        self.host_target(target, target)
    }

    /// Initialize a `cc::Build` with the given host and target as well as other default options.
    pub fn host_target(&self, host: &str, target: &str) -> cc::Build {
        let mut cfg = cc::Build::new();
        cfg.host(host)
            .target(target)
            .opt_level(2)
            .debug(false)
            .out_dir(self.dir())
            .__set_env("PATH", self.dir())
            .__set_env("GCCTEST_OUT_DIR", self.dir());
        if target.contains("msvc") {
            cfg.compiler(self.dir().join("cl"));
            cfg.archiver(self.dir().join("lib.exe"));
        }
        cfg
    }

    pub fn gcc(&self) -> cc::Build {
        let mut cfg = cc::Build::new();
        let target = if self.msvc {
            "x86_64-pc-windows-msvc"
        } else {
            "x86_64-unknown-linux-gnu"
        };

        cfg.target(target)
            .host(target)
            .opt_level(2)
            .debug(false)
            .out_dir(self.dir())
            .__set_env("PATH", self.dir())
            .__set_env("GCCTEST_OUT_DIR", self.dir());
        if self.msvc {
            cfg.compiler(self.dir().join("cl"));
            cfg.archiver(self.dir().join("lib.exe"));
        }
        cfg
    }

    pub fn cmd(&self, i: u32) -> Execution {
        let mut s = String::new();
        File::open(self.dir().join(format!("out{}", i)))
            .unwrap()
            .read_to_string(&mut s)
            .unwrap();
        Execution {
            args: s.lines().map(|s| s.to_string()).collect(),
        }
    }
}

impl Execution {
    pub fn must_have<P: AsRef<OsStr>>(&self, p: P) -> &Execution {
        if !self.has(p.as_ref()) {
            panic!("didn't find {:?} in {:?}", p.as_ref(), self.args);
        } else {
            self
        }
    }

    pub fn must_not_have<P: AsRef<OsStr>>(&self, p: P) -> &Execution {
        if self.has(p.as_ref()) {
            panic!("found {:?}", p.as_ref());
        } else {
            self
        }
    }

    pub fn has(&self, p: &OsStr) -> bool {
        self.args.iter().any(|arg| OsStr::new(arg) == p)
    }

    pub fn must_have_in_order(&self, before: &str, after: &str) -> &Execution {
        let before_position = self
            .args
            .iter()
            .rposition(|x| OsStr::new(x) == OsStr::new(before));
        let after_position = self
            .args
            .iter()
            .rposition(|x| OsStr::new(x) == OsStr::new(after));
        match (before_position, after_position) {
            (Some(b), Some(a)) if b < a => {}
            (b, a) => panic!(
                "{:?} (last position: {:?}) did not appear before {:?} (last position: {:?})",
                before, b, after, a
            ),
        };
        self
    }
}
