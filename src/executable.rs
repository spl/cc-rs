use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::fmt;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A minimal representation of an executable such as a compiler, assembler, or other build tool.
///
/// In the simplest case, `exe: Executable` contains only a path to the executable. In some cases,
/// `exe` contains an executable path with arguments (e.g. when invoking a script with `sh` or
/// `cmd.exe`). In other cases, `exe` contains an executable path and environment variables (e.g.
/// for `cl.exe` in MSVC.).
///
/// This representation is minimal in that it should not contain any context (arguments or
/// environment variables) that would modify the behavior beyond its default operation. By keeping
/// this invariant, we can run `Executable::to_command` in multiple different contexts.
#[derive(Clone)]
pub struct Executable {
    /// Verified canonical path of the executable.
    path: ExecutablePath,

    /// Note with extra information about the source of the `path`.
    ///
    /// This is included in the `Debug` string. It can be useful for identifying problems with the
    /// `Executable`. It does not need to include any other fields of the `Executable` since they
    /// are already included in the `Debug` string.
    note: String,

    /// Arguments passed to the executable.
    args: Vec<String>,

    /// Environment variables used by the executable.
    envs: HashMap<String, String>,
}

impl fmt::Debug for Executable {
    /// Format a debug string for an `Executable`. Note that, unlike `Command`, this includes all
    /// information, including environment variables.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Name
        write!(f, "{:?} (", self.path.name())?;
        // Note (possibly empty)
        if !self.note.is_empty() {
            write!(f, "{}, ", self.note)?;
        }
        // Path
        write!(f, "path: {:?}", &self.path)?;
        // Arguments (possibly empty)
        if !self.args.is_empty() {
            write!(f, ", args:")?;
            for arg in &self.args {
                write!(f, " \"{}\"", arg)?;
            }
        }
        // Environment variables (possibly empty)
        if !self.envs.is_empty() {
            write!(f, ", envs:")?;
            for (name, val) in &self.envs {
                write!(f, " {}=\"{}\"", name, val)?;
            }
        }
        write!(f, ")")
    }
}

impl Executable {
    /// Create an `Executable` from an `ExecutablePath`.
    pub fn new<S>(path: ExecutablePath, note: S) -> Result<Self, Error>
    where
        S: Into<String>,
    {
        Executable::new_with(path, note, Default::default(), Default::default())
    }

    /// Create an `Executable` from an `ExecutablePath` with context (arguments and environment
    /// variables).
    pub fn new_with<S: Into<String>>(
        path: ExecutablePath,
        note: S,
        args: Vec<String>,
        envs: HashMap<String, String>,
    ) -> Result<Self, Error> {
        // Build the `Executable` and test that it can be run.
        let exe = Executable {
            path,
            note: note.into(),
            args,
            envs,
        };
        exe.to_command().spawn().map_err(|_| Error::SpawnError {
            exe: format!("{:?}", exe),
        })?;
        Ok(exe)
    }

    /// Creates a new `Command` to run the `Executable`.
    pub fn to_command(&self) -> Command {
        let mut cmd = self.path.to_command();
        cmd.envs(&self.envs);
        cmd.args(&self.args);
        cmd
    }

    /// Returns the name used to build the `Executable`.
    pub fn name(&self) -> &OsStr {
        &self.path.name()
    }

    /// Returns the path of the `Executable`.
    ///
    /// WARNING! This is not necessarily the actual path of a compiler or other tool. In some
    /// cases, it may be something like `sh` or `cmd.exe`. It depends on how the `Executable` was
    /// constructed.
    pub fn path(&self) -> &Path {
        self.path.as_ref()
    }

    /// Returns the arguments to the `Executable`.
    pub fn args(&self) -> &Vec<String> {
        &self.args
    }

    /// Returns the environment variables for the `Executable`.
    pub fn envs(&self) -> &HashMap<String, String> {
        &self.envs
    }
}

/// A wrapper around `PathBuf` that contains a canonical path (with symbolic links resolved) that
/// has been verified to exist.
///
/// The goal of this wrapper is to catch problems early (at construction) related to which
/// executable is called.
///
/// The wrapper also contains the requested path name to provide a known string for debugging
/// because the canonical path may appear strange without that context.
///
/// Note that construction of an `ExecutablePath` does not imply the executable can be run. That
/// may require additional context (env vars or args). See `Executable`.
#[derive(Clone)]
pub struct ExecutablePath {
    /// Path of the executable requested at construction.
    ///
    /// This can be an absolute path, a relative path, or the name of an executable that should be
    /// found in a directory listed in the `PATH` environment variable.
    name: OsString,

    /// Verified, canonical, owned path of the executable.
    path: PathBuf,

    /// Current working directory of the executable.
    ///
    /// We keep this to ensure the `Command` from `ExecutablePath::to_command` has the expected
    /// environent.
    current_dir: Option<PathBuf>,

    /// Search paths in which the executable was found.
    ///
    /// We keep this to ensure the `Command` from `ExecutablePath::to_command` has the expected
    /// environent.
    paths: Option<OsString>,
}

impl ExecutablePath {
    /// Create an `ExecutablePath` from the name of the executable.
    ///
    /// `name` can be an absolute path, a path relative to the current directory, or the name of an
    /// executable that found in a directory listed in the `PATH` environment variable.
    pub fn new<N>(name: N) -> Result<Self, Error>
    where
        N: Into<OsString>,
    {
        let name = name.into();
        let result = which::which(&name);
        ExecutablePath::from_which(name, result, None, None)
    }

    /// Create an `ExecutablePath` given the name of an executable, a current directory, and an
    /// optional list of search paths (as would be given by the `PATH` env var).
    ///
    /// `name` can be an absolute path, a path relative to `current_dir`, or the name of an
    /// executable that found in a directory listed in the `paths` (which should have the same
    /// format as the `PATH` environment variable).
    pub fn new_in<N, P, S>(name: N, current_dir: P, paths: Option<S>) -> Result<Self, Error>
    where
        N: Into<OsString>,
        P: AsRef<Path>,
        S: AsRef<OsStr>,
    {
        let name = name.into();
        let current_dir = current_dir.as_ref().to_path_buf();
        let paths = paths.map(|s| s.as_ref().to_os_string());
        let result = which::which_in(&name, paths.as_ref(), &current_dir);
        ExecutablePath::from_which(name, result, Some(current_dir), paths)
    }

    /// Create an `ExecutablePath` from the result of `which::which` or `which::which_in`.
    fn from_which(
        name: OsString,
        result: which::Result<PathBuf>,
        current_dir: Option<PathBuf>,
        paths: Option<OsString>,
    ) -> Result<Self, Error> {
        // Find the path of the executable in the search paths.
        let path = result.map_err(|_| Error::WhichError { name: name.clone() })?;

        // Traverse symbolic links to find the canonical path.
        let path = path
            .canonicalize()
            .map_err(|_| Error::CanonicalizeError {
                name: name.clone(),
                path: path.clone(),
            })?
            .into();

        Ok(ExecutablePath {
            name,
            path,
            current_dir,
            paths,
        })
    }

    /// Creates a new `Command` with the `ExecutablePath`.
    pub fn to_command(&self) -> Command {
        let mut cmd = Command::new(&self.path);

        // Preserve the expected current directory and paths if they were passed to the
        // `ExecutablePath` at construction.
        for current_dir in &self.current_dir {
            cmd.current_dir(current_dir);
        }
        for paths in &self.paths {
            cmd.env("PATH", paths);
        }

        cmd
    }

    pub fn name(&self) -> &OsStr {
        &self.name
    }
}

impl fmt::Debug for ExecutablePath {
    /// Format a debug string for an `ExecutablePath`.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} (path: {:?})", self.name, self.path)
    }
}

impl AsRef<Path> for ExecutablePath {
    fn as_ref(&self) -> &Path {
        self.path.as_ref()
    }
}

impl AsRef<OsStr> for ExecutablePath {
    fn as_ref(&self) -> &OsStr {
        self.path.as_ref()
    }
}

/// An error that can occur while construction an `ExecutablePath` or `Executable`.
#[derive(Clone, Debug)]
pub enum Error {
    /// `which::which` error.
    WhichError { name: OsString },
    /// `fs::canonicalize` error.
    CanonicalizeError { name: OsString, path: PathBuf },
    /// `Command::spawn` error.
    SpawnError { exe: String },
}
