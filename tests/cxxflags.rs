extern crate cc;
extern crate tempdir;

mod support;

use std::env;
use support::Test;

#[test]
fn gnu_no_warnings_if_cxxflags() {
    env::set_var("CXXFLAGS", "-Wflag-does-not-exist");
    let test = Test::gnu();
    test.gcc().file("foo.c").cpp(true).compile("foo");

    test.cmd(0).must_not_have("-Wall").must_not_have("-Wextra");
}
