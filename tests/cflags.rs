extern crate cc;
extern crate tempdir;

mod support;

use std::env;
use support::Test;

#[test]
fn gnu_no_warnings_if_cflags() {
    env::set_var("CFLAGS", "-Wflag-does-not-exist");
    let test = Test::gnu();
    test.gcc().file("foo.c").compile("foo");

    test.cmd(0).must_not_have("-Wall").must_not_have("-Wextra");
}
