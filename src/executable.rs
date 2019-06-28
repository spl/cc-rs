use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::fmt;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A minimal representation of an executable such as a compiler, assembler, or other build tool.
///
/// In the simplest case, `exe: Executable` contains only a path to the executable. In some cases,
/// `exe` contains an executable path with arguments (e.g. when invoking a script with `sh` or
/// `cmd.exe`). In other cases, `exe` contains an executable path and environment variables (e.g.
/// for `cl.exe` in MSVC.).
///
/// This representation is minimal in that it does not contain any context (arguments or
/// environment variables) that would modify the behavior beyond its default operation. By keeping
/// this invariant, we can run `Executable::to_command` in multiple different contexts.
#[derive(Clone)]
pub struct Executable {
    /// Requested name of the executable.
    ///
    /// This can be an absolute path, a relative path, or an executable found in a directory listed
    /// in the `PATH` environment variable.
    name: OsString,

    /// A note with extra information about the source of the `name`.
    ///
    /// This is included in the `Debug` string. It can be useful for identifying problems with the
    /// `Executable`. It does not need to include any other fields of the `Executable` since they
    /// are already included in the `Debug` string.
    note: String,

    /// Path of an executable.
    ///
    /// This may be the path of the executable of interest, or it may be something like `sh` or
    /// `cmd.exe` if this involves invoking a script.
    ///
    /// Invariant: The construction of an `Executable` implies two things about `path`:
    ///   * It exists. By checking the path, we catch errors early.
    ///   * It is the canonical path (with symbolic links traversed). A canonical path helps
    ///     debug problems related to which executable is called.
    path: PathBuf,

    /// Arguments passed to the executable.
    args: Vec<String>,

    /// Environment variables used by the executable.
    envs: HashMap<String, String>,
}

impl fmt::Debug for Executable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Name
        write!(f, "{:?} (", self.name)?;
        // Note (possibly empty)
        if !self.note.is_empty() {
            write!(f, "{}, ", self.note)?;
        }
        // Path
        write!(f, "path: {:?}", self.path)?;
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
    /// Create an `Executable` with a context of arguments and environment variables.
    pub fn with_context<N: Into<OsString>, S: Into<String>>(
        name: N,
        note: S,
        args: Vec<String>,
        envs: HashMap<String, String>,
    ) -> io::Result<Self> {
        let name = name.into();
        let note = note.into();

        // Find the path of the executable. `name` can be an absolute path, a relative path, or an
        // executable found in one of the directories of the `PATH` environment variable.
        let path = which::which(&name)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{} ({:?})", e, name)))?;

        // Traverse symbolic links to find the canonical path.
        let path = path.canonicalize()?.into();

        // Build the `Executable` and test that it can be run.
        let exe = Executable {
            name,
            note,
            path,
            args,
            envs,
        };
        exe.to_command().spawn()?;

        Ok(exe)
    }

    /// Create an `Executable` with no arguments or environment variables.
    pub fn new<N: Into<OsString>, S: Into<String>>(name: N, note: S) -> io::Result<Self> {
        Executable::with_context(name, note, Vec::new(), HashMap::new())
    }

    /// Create an `Executable` for Emscripten.
    pub fn emscripten(cpp: bool) -> io::Result<Self> {
        let (name, exe) = if cpp {
            ("Emscripten C++", "em++")
        } else {
            ("Emscripten C", "emcc")
        };

        if cfg!(windows) {
            // Emscripten on Windows uses a batch file.
            Executable::with_context(
                "cmd",
                name,
                vec!["/c".to_string(), format!("{}.bat", exe)],
                HashMap::new(),
            )
        } else {
            Executable::new(exe, name)
        }
    }

    /// Returns a `Command` to run the `Executable`.
    pub fn to_command(&self) -> Command {
        let mut cmd = Command::new(&self.path);
        cmd.envs(&self.envs);
        cmd.args(&self.args);
        cmd
    }

    /// Returns the name used to build the `Executable`.
    pub fn name(&self) -> &OsStr {
        &self.name
    }

    /// Returns the path of the `Executable`.
    ///
    /// WARNING! You can't always assume that this is the actual path of a compiler or other tool.
    /// In some cases, it may be something like `sh` or `cmd.exe`. It depends on how the
    /// `Executable` was constructed.
    pub fn path(&self) -> &Path {
        &self.path
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
