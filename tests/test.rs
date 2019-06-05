extern crate cc;
extern crate tempdir;

use std::env;
use support::Test;

mod support;

#[test]
fn gnu_smoke() {
    let test = Test::trad();
    test.default().file("foo.c").compile("foo");

    test.cmd(0)
        .must_have("-O2")
        .must_have("foo.c")
        .must_not_have("-g")
        .must_have("-c")
        .must_have("-ffunction-sections")
        .must_have("-fdata-sections");
    test.cmd(1).must_have(test.dir().join("foo.o"));
}

#[test]
fn gnu_opt_level_1() {
    let test = Test::trad();
    test.default().opt_level(1).file("foo.c").compile("foo");

    test.cmd(0).must_have("-O1").must_not_have("-O2");
}

#[test]
fn gnu_opt_level_s() {
    let test = Test::trad();
    test.default()
        .opt_level_str("s")
        .file("foo.c")
        .compile("foo");

    test.cmd(0)
        .must_have("-Os")
        .must_not_have("-O1")
        .must_not_have("-O2")
        .must_not_have("-O3")
        .must_not_have("-Oz");
}

#[test]
fn gnu_debug() {
    let test = Test::trad();
    test.default().debug(true).file("foo.c").compile("foo");
    test.cmd(0).must_have("-g");
}

#[test]
fn gnu_warnings_into_errors() {
    let test = Test::trad();
    test.default()
        .warnings_into_errors(true)
        .file("foo.c")
        .compile("foo");

    test.cmd(0).must_have("-Werror");
}

#[test]
fn gnu_warnings() {
    let test = Test::trad();
    test.default()
        .warnings(true)
        .flag("-Wno-missing-field-initializers")
        .file("foo.c")
        .compile("foo");

    test.cmd(0).must_have("-Wall").must_have("-Wextra");
}

#[test]
fn gnu_extra_warnings0() {
    let test = Test::trad();
    test.default()
        .warnings(true)
        .extra_warnings(false)
        .flag("-Wno-missing-field-initializers")
        .file("foo.c")
        .compile("foo");

    test.cmd(0).must_have("-Wall").must_not_have("-Wextra");
}

#[test]
fn gnu_extra_warnings1() {
    let test = Test::trad();
    test.default()
        .warnings(false)
        .extra_warnings(true)
        .flag("-Wno-missing-field-initializers")
        .file("foo.c")
        .compile("foo");

    test.cmd(0).must_not_have("-Wall").must_have("-Wextra");
}

#[test]
fn gnu_warnings_overridable() {
    let test = Test::trad();
    test.default()
        .warnings(true)
        .flag("-Wno-missing-field-initializers")
        .file("foo.c")
        .compile("foo");

    test.cmd(0)
        .must_have_in_order("-Wall", "-Wno-missing-field-initializers");
}

#[test]
fn gnu_no_warnings_if_cflags() {
    env::set_var("CFLAGS", "-Wflag-does-not-exist");
    let test = Test::trad();
    test.default().file("foo.c").compile("foo");

    test.cmd(0).must_not_have("-Wall").must_not_have("-Wextra");
    env::set_var("CFLAGS", "");
}

#[test]
fn gnu_no_warnings_if_cxxflags() {
    env::set_var("CXXFLAGS", "-Wflag-does-not-exist");
    let test = Test::trad();
    test.default().file("foo.c").compile("foo");

    test.cmd(0).must_not_have("-Wall").must_not_have("-Wextra");
    env::set_var("CXXFLAGS", "");
}

#[test]
fn trad_x86_64_defaults() {
    let test = Test::trad();
    test.target("x86_64-unknown-linux-gnu")
        .file("foo.c")
        .compile("foo");
    test.cmd(0).must_have("-m64").must_have("-fPIC");
}

#[test]
fn clang_x86_64_defaults() {
    let test = Test::clang();
    test.target("x86_64-apple-darwin")
        .file("foo.c")
        .compile("foo");
    test.cmd(0).must_not_have("-m64").must_have("-fPIC");
}

#[test]
fn no_pic() {
    fn run(test: Test, arch: &str, vendor_os: &str) {
        test.target(&format!("{}-{}", arch, vendor_os))
            .pic(false)
            .file("foo.c")
            .compile("foo");
        test.cmd(0).must_not_have("-fPIC");
    }
    for arch in &["x86_64", "i686"] {
        run(Test::trad(), arch, "unknown-linux-gnu");
        run(Test::clang(), arch, "apple-darwin");
    }
}

#[test]
fn trad_i686_defaults() {
    let test = Test::trad();
    test.target("i686-unknown-linux-gnu")
        .file("foo.c")
        .compile("foo");
    test.cmd(0).must_have("-m32").must_have("-fPIC");
}

#[test]
fn clang_i686_defaults() {
    let test = Test::clang();
    test.target("i686-apple-darwin")
        .file("foo.c")
        .compile("foo");
    test.cmd(0).must_not_have("-m32").must_have("-fPIC");
}

#[test]
fn trad_x86_64_no_plt() {
    let test = Test::trad();
    test.target("x86_64-unknown-linux-gnu")
        .use_plt(false)
        .file("foo.c")
        .compile("foo");
    test.cmd(0).must_have("-fno-plt");
}

#[test]
fn trad_cpp_set_stdlib() {
    let test = Test::trad();
    test.target("x86_64-unknown-linux-gnu")
        .cpp(true)
        .cpp_set_stdlib(Some("foo"))
        .file("foo.c")
        .compile("foo");
    test.cmd(0).must_have("-stdlib=libfoo");
}

#[test]
fn trad_cpp_set_stdlib_no_cpp() {
    let test = Test::trad();
    test.target("x86_64-unknown-linux-gnu")
        .cpp_set_stdlib(Some("foo"))
        .file("foo.c")
        .compile("foo");
    test.cmd(0).must_not_have("-stdlib=libfoo");
}

#[test]
fn trad_include() {
    let test = Test::trad();
    test.target("x86_64-unknown-linux-gnu")
        .include("foo/bar")
        .file("foo.c")
        .compile("foo");
    test.cmd(0).must_have("-I").must_have("foo/bar");
}

#[test]
fn trad_define() {
    let test = Test::trad();
    test.target("x86_64-unknown-linux-gnu")
        .define("FOO", "bar")
        .define("BAR", None)
        .file("foo.c")
        .compile("foo");
    test.cmd(0).must_have("-DFOO=bar").must_have("-DBAR");
}

#[test]
fn trad_compile_assembly() {
    let test = Test::trad();
    test.target("x86_64-unknown-linux-gnu")
        .file("foo.S")
        .compile("foo");
    test.cmd(0).must_have("foo.S");
}

#[test]
fn trad_shared() {
    let test = Test::trad();
    test.target("x86_64-unknown-linux-gnu")
        .shared_flag(true)
        .static_flag(false)
        .file("foo.c")
        .compile("foo");
    test.cmd(0).must_have("-shared").must_not_have("-static");
}

#[test]
fn trad_flag_if_supported() {
    if cfg!(windows) {
        return;
    }
    let test = Test::trad();
    test.target("x86_64-unknown-linux-gnu")
        .file("foo.c")
        .flag("-v")
        .flag_if_supported("-Wall")
        .flag_if_supported("-Wflag-does-not-exist")
        .flag_if_supported("-std=c++11")
        .compile("foo");
    test.cmd(0)
        .must_have("-v")
        .must_have("-Wall")
        .must_not_have("-Wflag-does-not-exist")
        .must_not_have("-std=c++11");
}

#[test]
fn gnu_flag_if_supported_cpp() {
    if cfg!(windows) {
        return;
    }
    let test = Test::trad();
    test.target("x86_64-unknown-linux-gnu")
        .cpp(true)
        .file("foo.cpp")
        .flag_if_supported("-std=c++11")
        .compile("foo");

    test.cmd(0).must_have("-std=c++11");
}

#[test]
fn gnu_static() {
    let test = Test::trad();
    test.target("x86_64-unknown-linux-gnu")
        .file("foo.c")
        .shared_flag(false)
        .static_flag(true)
        .compile("foo");

    test.cmd(0).must_have("-static").must_not_have("-shared");
}

#[test]
fn msvc_smoke() {
    let test = Test::msvc();
    test.gcc().file("foo.c").compile("foo");

    test.cmd(0)
        .must_have("/O2")
        .must_have("foo.c")
        .must_not_have("/Z7")
        .must_have("/c")
        .must_have("/MD");
    test.cmd(1).must_have(test.dir().join("foo.o"));
}

#[test]
fn msvc_opt_level_0() {
    let test = Test::msvc();
    test.gcc().opt_level(0).file("foo.c").compile("foo");

    test.cmd(0).must_not_have("/O2");
}

#[test]
fn msvc_debug() {
    let test = Test::msvc();
    test.gcc().debug(true).file("foo.c").compile("foo");
    test.cmd(0).must_have("/Z7");
}

#[test]
fn msvc_include() {
    let test = Test::msvc();
    test.gcc().include("foo/bar").file("foo.c").compile("foo");

    test.cmd(0).must_have("/I").must_have("foo/bar");
}

#[test]
fn msvc_define() {
    let test = Test::msvc();
    test.gcc()
        .define("FOO", "bar")
        .define("BAR", None)
        .file("foo.c")
        .compile("foo");

    test.cmd(0).must_have("/DFOO=bar").must_have("/DBAR");
}

#[test]
fn msvc_static_crt() {
    let test = Test::msvc();
    test.gcc().static_crt(true).file("foo.c").compile("foo");

    test.cmd(0).must_have("/MT");
}

#[test]
fn msvc_no_static_crt() {
    let test = Test::msvc();
    test.gcc().static_crt(false).file("foo.c").compile("foo");

    test.cmd(0).must_have("/MD");
}
