#![cfg_attr(test, allow(dead_code))]

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var_os("GCCTEST_OUT_DIR").unwrap());
    for i in 0.. {
        let fname = format!("out{}", i);
        let candidate = out_dir.join(&fname);
        if candidate.exists() {
            continue;
        }
        let mut f = File::create(&candidate).unwrap();
        for arg in env::args().skip(1) {
            writeln!(f, "{}", &arg).unwrap();
            writeln!(std::io::stdout(), "{}: {}", &fname, &arg).unwrap();
        }

        File::create(out_dir.join("libfoo.a")).unwrap();
        break;
    }
}
