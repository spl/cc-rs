extern crate cc;
extern crate tempdir;
extern crate which;

use std::env;

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
    let result = test.gcc().file("foo.c").try_get_compiler();
    assert!(result.is_err());
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
    let ccache = test.which("ccache").into_os_string().into_string().unwrap();
    let cc = test.which("cc");

    env::set_var("CC", format!("{} cc -m32", ccache));
    let compiler = test.gcc().file("foo.c").get_compiler();
    assert_eq!(compiler.path(), cc);
    let cc_env = compiler.cc_env().into_string().unwrap();
    let cc = cc.into_os_string().into_string().unwrap();
    assert_eq!(cc_env, format!("{} {} -m32", ccache, cc));
}

fn more_spaces() {
    let test = Test::gnu();
    test.shim("ccache");

    env::set_var("CC", "cc -m32");
    let compiler = test.gcc().file("foo.c").get_compiler();
    assert_eq!(compiler.path(), test.which("cc"));
}
