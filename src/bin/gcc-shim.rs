#![cfg_attr(test, allow(dead_code))]

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

fn main() {
    let mut args = env::args();
    let _program = args.next().unwrap();
    let args: Vec<_> = args.collect();

    // Handle the following special cases:
    match &args[..] {
        // No arguments. Assume this is a spawn test and don't do anything.
        [] => return,
        // The arguments indicate a `ToolFamily::detect`. If a `ToolFamily` type is provided by the
        // env var, print it.
        [_, a2] if a2.contains("cc_rs_tool_family") => {
            if let Ok(family) = env::var("CC_RS_TOOL_FAMILY") {
                let _ = std::io::stdout().write_all(family.as_ref());
            }
            return;
        },
        _ => {}
    }

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR not found"));

    for i in 0.. {
        let candidate = &out_dir.join(format!("out{}", i));

        // If the file exists, this command has already been called: try the `i+1`th time.
        if candidate.exists() {
            //println!("shim={} candidate={:?} exists:", program, candidate);
            std::io::copy(&mut File::open(candidate).unwrap(), &mut std::io::stdout()).unwrap();
            continue;
        }

        // Create a file and record the args passed to the command.
        let mut f = File::create(candidate).unwrap();
        //println!("shim={} candidate={:?} created", program, candidate);
        for arg in args {
            //println!("shim={} candidate={:?} i={} arg={}", program, candidate, i, arg);
            writeln!(f, "{}", arg).unwrap();
        }

        File::create(out_dir.join("libfoo.a")).unwrap();
        break;
    }
}
