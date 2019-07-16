#![cfg_attr(test, allow(dead_code))]

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

fn main() {
    let mut args = env::args();

    // Get the program file stem.
    let program = args.next().expect("Unexpected empty args");
    let program = &Path::new(&program)
        .file_stem()
        .expect(&format!("{}: no file stem", program))
        .to_string_lossy()
        .into_owned();

    let args: Vec<_> = args.collect();

    // Handle special cases:
    match &args[..] {
        // No arguments. Assume this is a spawn test and don't do anything.
        [] => return,
        // The arguments indicate a `ToolFamily::detect`. Print the expected `ToolFamily` string.
        [_, a2] if a2.ends_with("cc_rs_tool_family.h") => {
            let _ = if program == "cl" {
                std::io::stdout().write_all(b"msvc")
            } else {
                std::io::stdout().write_all(b"gnu")
            };
            return;
        }
        _ => {}
    }

    let out_dir = PathBuf::from(
        env::var_os("GCCTEST_OUT_DIR").expect(&format!("{}: GCCTEST_OUT_DIR not found", program)),
    );

    // Find the first nonexistent candidate file to which the program's args can be written.
    for i in 0.. {
        let candidate = &out_dir.join(format!("out{}", i));

        // If the file exists, commands have already run. Try again.
        if candidate.exists() {
            continue;
        }

        // Create a file and record the args passed to the command.
        let mut f = File::create(candidate).expect(&format!(
            "{}: can't create candidate: {}",
            program,
            candidate.to_string_lossy()
        ));
        for arg in args {
            // Tests for flag_if_supported
            if !program.ends_with("++") {
                exit_if_eq(program, &arg, "-std=c++11");
            }
            exit_if_eq(program, &arg, "-Wflag-does-not-exist");

            writeln!(f, "{}", arg).expect(&format!(
                "{}: can't write to candidate: {}",
                program,
                candidate.to_string_lossy()
            ));
        }
        break;
    }

    // Create a file used by some tests.
    let path = &out_dir.join("libfoo.a");
    File::create(path).expect(&format!(
        "{}: can't create libfoo.a: {}",
        program,
        path.to_string_lossy()
    ));
}

/// Report the flag to `stderr` and exit.
fn exit_if_eq<S, T, U>(program: S, arg: T, flag: U)
where
    S: AsRef<str>,
    T: AsRef<str>,
    U: AsRef<str>,
{
    if arg.as_ref() == flag.as_ref() {
        eprintln!(
            "{}: flag '{}' not supported",
            program.as_ref(),
            flag.as_ref()
        );
        std::process::exit(0);
    }
}
