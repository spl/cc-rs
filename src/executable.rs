use std::fmt;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use which::which;
use std::collections::HashMap;

/// A minimal representation of an executable such as a compiler, assembler, or other build tool.
///
/// In the simplest case, `exe: Executable` contains only a path to the executable. In some cases,
/// `exe` contains an executable path with arguments (e.g. when invoking a script with `sh` or
/// `cmd.exe`). In other cases, `exe` contains an executable path and environment variables (e.g.
/// for `cl.exe` in MSVC.).
///
/// This representation is minimal in that it does not contain any context (arguments or
/// environment variables) that would modify the behavior beyond its default operation. By keeping
/// this invariant, we can re-use an `Executable` as a command in different contexts.
#[derive(Clone)]
pub struct Executable {
    /// Information about the name and construction of the executable.
    ///
    /// This is shown to the user in status and error messages.
    info: Vec<String>,

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
        let mut info = self.info.iter();
        if let Some(name) = info.next() {
            write!(f, "{} (", name)?;
            for s in info {
                write!(f, "{}, ", s)?;
            }
            write!(f, "command: {:?})", self.to_command())?;
        }
        Ok(())
    }
}

impl Executable {
    /// Create an `Executable` from a path, arguments, and environment variables.
    fn from_path_with_context<P>(info: Vec<String>, path: P, args: Vec<String>, envs: HashMap<String, String>) -> io::Result<Self>
    where
        P: AsRef<Path>,
    {
        let requested_path = path.as_ref();

        // Check if the request path exists and traverse symbolic links to find the canonical path.
        let canonical_path = requested_path.canonicalize()?.into();

        // Build the debug information, adding the requested path if it's different from the
        // canonical path.
        let mut info = info;
        if requested_path != canonical_path {
            info.push(format!("requested path: {}", requested_path.to_string_lossy()));
        }

        Ok(Executable {
            info,
            path: canonical_path,
            args,
            envs,
        })
    }

    /// Create an `Executable` from a path.
    pub fn from_path<P>(info: Vec<String>, path: P) -> io::Result<Self>
    where
        P: AsRef<Path>,
    {
        Executable::from_path_with_context(info, path, Vec::new(), HashMap::new())
    }

    /// Create an `Executable` from the name (not the path) of an executable, arguments, and
    /// environment variables.
    ///
    /// The directories of the `PATH` environment variable are searched to find the executable
    /// name.
    fn from_name_with_context<S>(info: Vec<String>, exe: S, args: Vec<String>, envs: HashMap<String, String>) -> io::Result<Self>
    where
        S: Into<String>,
    {
        let path = which(exe.into()).map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{}", e)))?;
        Executable::from_path_with_context(info, path, args, envs)
    }

    /// Create an `Executable` from the name of executable.
    ///
    /// The executable must be found in one of the directories in the `PATH` environment.
    pub fn from_name<S>(info: Vec<String>, exe: S) -> io::Result<Self>
    where
        S: Into<String>,
    {
        Executable::from_name_with_context(info, exe, Vec::new(), HashMap::new())
    }

    /// Create an `Executable` for Emscripten.
    pub fn emscripten(cpp: bool) -> io::Result<Self> {
        let (name, exe) = if cpp {
            ("Emscripten C++".to_string(), "em++")
        } else {
            ("Emscripten C".to_string(), "emcc")
        };

        // Build the info vector with added capacity for the requested path.
        let mut info = Vec::with_capacity(2);
        info.push(name);

        if cfg!(windows) {
            // Emscripten on Windows uses a batch file.
            Executable::from_name_with_context(info, "cmd", vec!["/c".to_string(), format!("{}.bat", exe)], HashMap::new())
        } else {
            Executable::from_name(info, exe)
        }
    }

    /// Returns a `Command` to run the `Executable`.
    fn to_command(&self) -> Command {
        let mut cmd = Command::new(&self.path);
        cmd.envs(&self.envs);
        cmd.args(&self.args);
        cmd
    }

    /// Returns the `Path` of the `Executable`.
    ///
    /// FIXME: We should not need this because the path may be something like `sh` or `cmd.exe` and
    /// used to invoke a script.
    pub fn path(&self) -> &Path {
        &self.path
    }
}
