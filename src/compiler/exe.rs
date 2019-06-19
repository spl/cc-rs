use super::super::ToolFamily;
use std::ffi::OsStr;
use std::fmt;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use which::which;

/// A representation of the minumum components needed to execute a compiler.
///
/// In many cases, this representation contains only a path to the executable, thus the name `Exe`.
/// It does not contain flags that modify the behavior of the executable. However, in some cases,
/// an `Exe` may represent the invocation of a script with arguments.
#[derive(Clone)]
pub struct Exe {
    /// Familiar name for the compiler.
    ///
    /// This is used for printing messages.
    name: String,

    /// Path of an executable.
    ///
    /// This may be the actual compiler or something like `sh` or `cmd.exe` if this involves
    /// invoking a script.
    ///
    /// Invariant: This path must be exist, and it should be the canonical path. A canonical path
    /// helps with identifying problems related to the executable called.
    path: PathBuf,

    /// Arguments passed to the executable.
    args: Vec<String>,

    /// Family of tools to which this compiler belongs.
    ///
    /// Invariant: This should be determined from the compiler itself.
    family: ToolFamily,
}

impl fmt::Debug for Exe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({:?})", self.name, self.to_command())
    }
}

impl Exe {
    /// Create an `Exe` from a path and arguments.
    ///
    /// The canonical path and `ToolFamily` are determined automatically.
    fn from_path_with_args<S, P>(name: S, path: P, args: Vec<String>) -> io::Result<Self>
    where
        S: Into<String>,
        P: AsRef<Path>,
    {
        let path = path.as_ref().canonicalize()?;
        let family = ToolFamily::of_command(Command::new(&path).args(&args))?;
        Ok(Exe {
            name: name.into(),
            path,
            args,
            family,
        })
    }

    /// Create an `Exe` from a path.
    ///
    /// The canonical path and `ToolFamily` are determined automatically.
    pub fn from_path<S, P>(name: S, path: P) -> io::Result<Self>
    where
        S: Into<String>,
        P: AsRef<Path>,
    {
        Exe::from_path_with_args(name, path, Vec::new())
    }

    /// Create an `Exe` from the name of executable and arguments.
    ///
    /// The executable must be found in one of the directories in the `PATH` environment.
    ///
    /// The canonical path of the executable and the `ToolFamily` are determined automatically.
    fn from_name_with_args<S, E>(name: S, exe: E, args: Vec<String>) -> io::Result<Self>
    where
        S: Into<String>,
        E: AsRef<OsStr>,
    {
        Exe::from_path_with_args(name, &which(exe).unwrap(), args)
    }

    /// Create an `Exe` from the name of executable.
    ///
    /// The executable must be found in one of the directories in the `PATH` environment.
    ///
    /// The canonical path of the executable and the `ToolFamily` are determined automatically.
    pub fn from_name<S, E>(name: S, exe: E) -> io::Result<Self>
    where
        S: Into<String>,
        E: AsRef<OsStr>,
    {
        Exe::from_name_with_args(name, exe, Vec::new())
    }

    /// Create an `Exe` for Emscripten.
    pub fn emscripten(cpp: bool) -> io::Result<Self> {
        let (name, exe) = if cpp {
            ("Emscripten C++".to_string(), "em++")
        } else {
            ("Emscripten C".to_string(), "emcc")
        };

        if cfg!(windows) {
            // Emscripten on Windows uses a batch file.
            Exe::from_name_with_args(name, "cmd", vec!["/c".to_string(), format!("{}.bat", exe)])
        } else {
            Exe::from_name(name, exe)
        }
    }

    /// Create a `Command` from an `Exe`.
    fn to_command(&self) -> Command {
        let mut cmd = Command::new(&self.path);
        cmd.args(&self.args);
        cmd
    }

    /// Returns the `Path` of the `Exe`.
    ///
    /// FIXME: We should not need this because the path may be something like `sh` or `cmd.exe` and
    /// used to invoke a script.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Returns the `ToolFamily` of the `Exe`.
    pub fn family(&self) -> ToolFamily {
        self.family
    }
}
