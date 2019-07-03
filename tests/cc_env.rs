extern crate cc;
extern crate tempdir;
extern crate which;

use std::env;
use std::ffi::OsString;

mod support;
use support::Test;

#[test]
fn main() {
    ccache();
    distcc();
    ccache_spaces();
    ccache_env_flags();
    leading_spaces();
    extra_flags();
    path_to_ccache();
    more_spaces();
}

fn ccache() {
    let test = Test::gnu();

    env::set_var("CC", "ccache cc");
    let compiler = test.gcc().file("foo.c").get_compiler();

    assert_eq!(compiler.path(), test.which("cc"));
}

fn ccache_spaces() {
    let test = Test::gnu();
    test.shim("ccache");

    env::set_var("CC", "ccache        cc");
    let compiler = test.gcc().file("foo.c").get_compiler();
    assert_eq!(compiler.path(), test.which("cc"));
}

fn distcc() {
    let test = Test::gnu();
    test.shim("distcc");

    env::set_var("CC", "distcc cc");
    let compiler = test.gcc().file("foo.c").get_compiler();
    assert_eq!(compiler.path(), test.which("cc"));
}

fn ccache_env_flags() {
    let test = Test::gnu();
    test.shim("ccache");

    env::set_var("CC", "ccache lol-this-is-not-a-compiler");
    let compiler = test.gcc().file("foo.c").get_compiler();
    assert_eq!(compiler.path(), test.which("ccache"));
    assert_eq!(compiler.cc_env(), OsString::from(""));
    assert!(
        compiler
            .cflags_env()
            .into_string()
            .unwrap()
            .contains("ccache")
            == false
    );
    assert!(
        compiler
            .cflags_env()
            .into_string()
            .unwrap()
            .contains("lol-this-is-not-a-compiler")
            == false
    );

    env::set_var("CC", "");
}

fn leading_spaces() {
    let test = Test::gnu();
    test.shim("test");

    env::set_var("CC", " test ");
    let compiler = test.gcc().file("foo.c").get_compiler();
    assert_eq!(compiler.path(), test.which("test"));

    env::set_var("CC", "");
}

fn extra_flags() {
    let test = Test::gnu();
    test.shim("ccache");

    env::set_var("CC", "ccache cc -m32");
    let compiler = test.gcc().file("foo.c").get_compiler();
    assert_eq!(compiler.path(), test.which("cc"));
}

fn path_to_ccache() {
    let test = Test::gnu();
    test.shim("ccache");

    env::set_var("CC", "/path/to/ccache.exe cc -m32");
    let compiler = test.gcc().file("foo.c").get_compiler();
    assert_eq!(compiler.path(), test.which("cc"));
    assert_eq!(compiler.cc_env(), OsString::from(""));
}

fn more_spaces() {
    let test = Test::gnu();
    test.shim("ccache");

    env::set_var("CC", "cc -m32");
    let compiler = test.gcc().file("foo.c").get_compiler();
    assert_eq!(compiler.path(), test.which("cc"));
}
