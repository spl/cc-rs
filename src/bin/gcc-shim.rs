#![cfg_attr(test, allow(dead_code))]

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

fn main() {
    let mut args = env::args();
    let program = args.next().unwrap();
    let args: Vec<_> = args.collect();

    // Handle special cases:
    match &args[..] {
        // No arguments. Assume this is a spawn test and don't do anything.
        [] => return,
        // The arguments indicate a `ToolFamily::detect`. Print the expected `ToolFamily` string.
        [_, a2] if a2.contains("cc_rs_tool_family") => {
            let program = Path::new(&program)
                .file_stem()
                .expect(&format!("Program `{}` has no file stem", program));
            let _ = if program == "cl" {
                std::io::stdout().write_all(b"msvc")
            } else {
                std::io::stdout().write_all(b"gnu")
            };
            return;
        }
        _ => {}
    }

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect(&format!(
        "Program `{}`: OUT_DIR not found",
        program
    )));

    for i in 0.. {
        let candidate = &out_dir.join(format!("out{}", i));

        // If the file exists, commands have already run. Try again.
        if candidate.exists() {
            continue;
        }

        // Create a file and record the args passed to the command.
        let mut f = File::create(candidate).unwrap();
        for arg in args {
            writeln!(f, "{}", arg).unwrap();
        }

        File::create(out_dir.join("libfoo.a")).unwrap();
        break;
    }
}
