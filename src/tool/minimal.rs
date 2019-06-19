use std::fmt;
use super::super::ToolFamily;
use std::process::Command;
use std::path::{Path, PathBuf};
use std::io;
use std::ffi::OsStr;
use which::which;

/// A minimal representation of the components necessary to invoke a compiler without any wrapper
/// or additional flags that modify its behavior.
///
/// This may represent the invocation of a script, so it includes both the path of an executable
/// and arguments passed to the executable.
#[derive(Clone, Debug)]
pub struct Minimal {
    /// Familiar name for the compiler. This is used for printing messages.
    name: String,
    /// Path of the executable. If this involves invoking a script, the executable may be something
    /// like `sh` or `cmd.exe`.
    path: PathBuf,
    /// Arguments passed to the executable.
    args: Vec<String>,
    /// Family of tools to which this compiler belongs.
    family: ToolFamily,
}

impl fmt::Display for Minimal {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{} ({:?})", self.name, self.to_command())
    }
}

impl Minimal {

    fn new<P: AsRef<Path>>(name: String, path: P, args: Vec<String>, family: ToolFamily) -> Result<Self, io::Error> {
        let path = path.as_ref().canonicalize()?;
        Ok(Minimal { name, path, args, family })
    }

    fn from_path_with_args<P: AsRef<Path>>(name: String, path: P, args: Vec<String>) -> Result<Self, io::Error> {
        let family = ToolFamily::of_command(Command::new(path.as_ref()).args(&args))?;
        Minimal::new(name, path, args, family)
    }

    pub fn from_path<P: AsRef<Path>>(name: String, path: P) -> Result<Self, io::Error> {
        Minimal::from_path_with_args(name, path, Vec::new())
    }

    fn from_exe_with_args<E: AsRef<OsStr>>(name: String, exe: E, args: Vec<String>) -> Result<Self, io::Error> {
        let path = which(exe).unwrap();
        Minimal::from_path_with_args(name, &path, args)
    }

    pub fn from_exe<E: AsRef<OsStr>>(name: String, exe: E) -> Result<Self, io::Error> {
        Minimal::from_exe_with_args(name, exe, Vec::new())
    }

    fn to_command(&self) -> Command {
        let mut cmd = Command::new(&self.path);
        cmd.args(&self.args);
        cmd
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn family(&self) -> ToolFamily {
        self.family
    }

    pub fn emscripten(cpp: bool) -> Result<Self, io::Error> {
        let (name, exe) = if cpp { ("Emscripten C++".to_string(), "em++") } else { ("Emscripten C".to_string(), "emcc") };

        if cfg!(windows) {
            // Emscripten on Windows uses a batch file.
            Minimal::from_exe_with_args(name, "cmd", vec!["/c".to_string(), format!("{}.bat", exe)])
        } else {
            Minimal::from_exe(name, exe)
        }
    }
}
