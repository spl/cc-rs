#![allow(dead_code)]

use super::Error;
use super::ErrorKind::ToolNotFound;
use std::collections::HashMap;
use std::env;
use std::ffi::{OsStr, OsString};
use std::fmt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use which;

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
    /// Requested name or path of the executable.
    ///
    /// This can be an absolute path, a relative path, or the name of an executable that should be
    /// found in a directory listed in the `PATH` environment variable.
    ///
    /// We keep this around to report meaningful error messages to the user. For example, the
    /// canonical `path` can be so different that it's unrecognizable. In that case, the user
    /// should recognize the requested name or path.
    requested: OsString,

    /// Verified canonical path of the executable.
    path: PathBuf,

    /// Note with extra information about the source of the `path`.
    ///
    /// This is included in the `Debug` string. It can be useful for identifying problems with the
    /// `Executable`. It does not need to include any other fields of the `Executable` since they
    /// are already included in the `Debug` string.
    note: String,

    /// Arguments passed to the executable.
    args: Vec<OsString>,

    /// Environment variables used by the executable.
    envs: HashMap<OsString, OsString>,
}

impl fmt::Debug for Executable {
    /// Format a debug string for an `Executable`. Note that, unlike `Command`, this includes all
    /// information, including environment variables.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Name
        write!(f, "{:?} (", self.requested())?;
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
                write!(f, " \"{:?}\"", arg)?;
            }
        }
        // Environment variables (possibly empty)
        if !self.envs.is_empty() {
            write!(f, ", envs:")?;
            for (name, val) in &self.envs {
                write!(f, " {:?}=\"{:?}\"", name, val)?;
            }
        }
        write!(f, ")")
    }
}

impl Executable {
    /// Creates a new `Command` to run the `Executable`.
    pub fn to_command(&self) -> Command {
        let mut cmd = Command::new(&self.path);
        cmd.envs(&self.envs);
        cmd.args(&self.args);
        cmd
    }

    /// Returns a reference to the requested name or path.
    pub fn requested(&self) -> &OsStr {
        &self.requested
    }

    /// Returns a reference to the verified path.
    ///
    /// WARNING! This is not necessarily the path of a compiler or other tool. In some cases, it
    /// may be something like `sh` or `cmd.exe`. It depends on how the `Executable` was
    /// constructed.
    pub fn path(&self) -> &Path {
        self.path.as_ref()
    }

    /// Returns the arguments to the `Executable`.
    pub fn args(&self) -> &Vec<OsString> {
        &self.args
    }

    /// Returns the environment variables for the `Executable`.
    pub fn envs(&self) -> &HashMap<OsString, OsString> {
        &self.envs
    }
}

/// A builder for either an `Executable` using `Build::exe` or a `PathBuf` of an executable using
/// `Build::path`.
#[derive(Clone, Debug)]
pub struct Build {
    requested: OsString,
    note: String,
    args: Vec<OsString>,
    envs: HashMap<OsString, OsString>,
    current_dir: Option<PathBuf>,
}

impl Build {
    pub fn new<T, U>(requested: T, note: U) -> Self
    where
        T: Into<OsString>,
        U: Into<String>,
    {
        Build {
            requested: requested.into(),
            note: note.into(),
            args: Vec::new(),
            envs: HashMap::new(),
            current_dir: None,
        }
    }

    pub fn arg<T: AsRef<OsStr>>(&mut self, arg: T) -> &mut Self {
        self.args.push(arg.as_ref().into());
        self
    }

    pub fn args<I, T>(&mut self, args: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: AsRef<OsStr>,
    {
        for arg in args {
            self.arg(arg);
        }
        self
    }

    pub fn env<T, U>(&mut self, var: T, val: U) -> &mut Self
    where
        T: AsRef<OsStr>,
        U: AsRef<OsStr>,
    {
        self.envs.insert(var.as_ref().into(), val.as_ref().into());
        self
    }

    pub fn envs<'a, I, K, V>(&mut self, envs: I) -> &mut Self
    where
        I: IntoIterator<Item = &'a (K, V)>,
        K: 'a + AsRef<OsStr>,
        V: 'a + AsRef<OsStr>,
    {
        for (var, val) in envs {
            self.env(var, val);
        }
        self
    }

    pub fn current_dir<T: AsRef<Path>>(&mut self, current_dir: T) -> &mut Self {
        self.current_dir.replace(current_dir.as_ref().into());
        self
    }

    fn take_path(&mut self) -> Result<PathBuf, Error> {
        // Get the search paths by first checking the environment variables for an optional
        // explicit use of `PATH` and then checking the environment itself.
        let paths: Option<OsString> = self
            .envs
            .get(OsStr::new("PATH"))
            .cloned()
            .or_else(|| env::var_os("PATH"));

        // Get the current working directory by first checking for the optional explicit use and
        // then checking the environment itself.
        let current_dir: PathBuf = self
            .current_dir
            .take()
            .ok_or_else(|| 0)
            .or_else(|_error_not_used| env::current_dir())
            .map_err(|e| {
                Error::new(
                    ToolNotFound,
                    &format!("Can't access current directory: {}", e),
                )
            })?;

        // Find the path of the requested executable.
        //
        // This confirms that we're working with an actual executable in the search path.
        let path = which::which_in(&self.requested, paths.as_ref(), &current_dir).map_err(|e| {
            Error::new(
                ToolNotFound,
                &format!(
                    "{:?} (paths: {:?}, current_dir: {:?}): Can't find path of executable: {}",
                    self.requested, paths, current_dir, e
                ),
            )
        })?;

        // Find the canonical path of the executable.
        //
        // This gives the absolute path with all symbolic links and relative paths resolved. It
        // is useful for identifying the executable used when debugging problems with compilation.
        let path = path.canonicalize().map_err(|e| {
            Error::new(
                ToolNotFound,
                &format!(
                    "{:?} (path: {:?}): Can't find canonical path: {}",
                    self.requested, path, e
                ),
            )
        })?;

        Ok(if cfg!(windows) {
            // This is an attempt to remove the leading "\\?\" from file paths with the Windows NT
            // universal naming convention (a.k.a. UNC or extended-length paths) produced by
            // `std::fs::canonicalize`. Such paths are not accepted by many Windows programs
            // (including MinGW `gcc` and some versions of MSVC `cl.exe`).
            //
            // Reference: <https://docs.microsoft.com/en-us/windows/win32/fileio/naming-a-file>
            //
            // Note that this does not fail. If the string cannot be converted to Unicode or the
            // prefix is not found, the original path is used.
            //
            // If better conversion is desired, consider <https://crates.io/crates/dunce>.

            /*
            match path.clone().to_str() {
                Some(s) if s.starts_with(r"\\?\") => match s.get(4..) {
                    Some(s) => PathBuf::from(s),
                    None => path,
                },
                _ => path,
            }
            */

            /*
            let mut path = path;
            if let Some(s) = path.to_str() {
                if s.starts_with(r"\\?\") {
                    if let Some(s) = s.get(4..) {
                        path = PathBuf::from(s.to_string());
                    }
                }
            }
            */

            path.to_str().map(|s| s.get(4..)).and_then(|o| o).map(PathBuf::from).unwrap_or(path)
        } else {
            path
        })
    }

    /// Convert the builder into a canonical path without performing a spawn test.
    pub fn path(mut self) -> Result<PathBuf, Error> {
        self.take_path()
    }

    /// Convert the builder into an executable after performing a spawn test.
    pub fn exe(mut self) -> Result<Executable, Error> {
        let path = self.take_path()?;

        // Construct and verify that it can be spawned.
        let exe = Executable {
            requested: self.requested,
            path,
            note: self.note,
            args: self.args,
            envs: self.envs,
        };
        exe.to_command()
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| {
                Error::new(
                    ToolNotFound,
                    &format!("{:?}: Can't spawn executable: {}", exe, e),
                )
            })?;
        Ok(exe)
    }
}
