var N=null,E="",T="t",U="u",searchIndex={};
var R=["option","build","result","Configures the optimization level of the generated object…","Run the compiler, generating the file `output`","Get the compiler that's in use for this configuration.","error","command","osstring","to_owned","clone_into","try_from","borrow_mut","try_into","type_id","cc::windows_registry","formatter"];
searchIndex["cc"]={"doc":"A library for build scripts to compile custom C code","i":[[3,"Build","cc","A builder for compilation of a native static library.",N,N],[3,"Error",E,"Represents an internal error that occurred, with an…",N,N],[3,"Tool",E,"Configuration used to represent an invocation of a C…",N,N],[0,"windows_registry",E,"A helper module to probe the Windows Registry when looking…",N,N],[4,"VsVers",R[15],"A version of Visual Studio",N,N],[13,"Vs12",E,"Visual Studio 12 (2013)",0,N],[13,"Vs14",E,"Visual Studio 14 (2015)",0,N],[13,"Vs15",E,"Visual Studio 15 (2017)",0,N],[13,"Vs16",E,"Visual Studio 16 (2019)",0,N],[5,"find",E,"Attempts to find a tool within an MSVC installation using…",N,[[["str"],["str"]],[R[0],[R[7]]]]],[5,"find_tool",E,"Similar to the `find` function above, this function will…",N,[[["str"],["str"]],[R[0],["tool"]]]],[5,"find_vs_version",E,"Find the most recent installed version of Visual Studio",N,[[],[R[2],["vsvers","string"]]]],[11,"new","cc","Construct a new instance of a blank set of configuration.",1,[[],[R[1]]]],[11,"include",E,"Add a directory to the `-I` or include path for headers",1,[[["self"],["p"]],[R[1]]]],[11,"define",E,"Specify a `-D` variable with an optional value.",1,[[["self"],["str"],["v"]],[R[1]]]],[11,"object",E,"Add an arbitrary object file to link in",1,[[["self"],["p"]],[R[1]]]],[11,"flag",E,"Add an arbitrary flag to the invocation of the compiler",1,[[["self"],["str"]],[R[1]]]],[11,"is_flag_supported",E,"Run the compiler to test if it accepts the given flag.",1,[[["self"],["str"]],[R[2],["bool",R[6]]]]],[11,"flag_if_supported",E,"Add an arbitrary flag to the invocation of the compiler if…",1,[[["self"],["str"]],[R[1]]]],[11,"shared_flag",E,"Set the `-shared` flag.",1,[[["self"],["bool"]],[R[1]]]],[11,"static_flag",E,"Set the `-static` flag.",1,[[["self"],["bool"]],[R[1]]]],[11,"file",E,"Add a file which will be compiled",1,[[["self"],["p"]],[R[1]]]],[11,"files",E,"Add files which will be compiled",1,[[["self"],["p"]],[R[1]]]],[11,"cpp",E,"Set C++ support.",1,[[["self"],["bool"]],[R[1]]]],[11,"cuda",E,"Set CUDA C++ support.",1,[[["self"],["bool"]],[R[1]]]],[11,"warnings_into_errors",E,"Set warnings into errors flag.",1,[[["self"],["bool"]],[R[1]]]],[11,"warnings",E,"Set warnings flags.",1,[[["self"],["bool"]],[R[1]]]],[11,"extra_warnings",E,"Set extra warnings flags.",1,[[["self"],["bool"]],[R[1]]]],[11,"cpp_link_stdlib",E,"Set the standard library to link against when compiling…",1,[[["self"],["v"]],[R[1]]]],[11,"cpp_set_stdlib",E,"Force the C++ compiler to use the specified standard…",1,[[["self"],["v"]],[R[1]]]],[11,"target",E,"Configures the target this configuration will be compiling…",1,[[["self"],["str"]],[R[1]]]],[11,"host",E,"Configures the host assumed by this configuration.",1,[[["self"],["str"]],[R[1]]]],[11,"opt_level",E,R[3],1,[[["self"],["u32"]],[R[1]]]],[11,"opt_level_str",E,R[3],1,[[["self"],["str"]],[R[1]]]],[11,"debug",E,"Configures whether the compiler will emit debug…",1,[[["self"],["bool"]],[R[1]]]],[11,"out_dir",E,"Configures the output directory where all object files and…",1,[[["self"],["p"]],[R[1]]]],[11,"compiler",E,"Configures the compiler to be used to produce output.",1,[[["self"],["p"]],[R[1]]]],[11,"archiver",E,"Configures the tool used to assemble archives.",1,[[["self"],["p"]],[R[1]]]],[11,"cargo_metadata",E,"Define whether metadata should be emitted for cargo…",1,[[["self"],["bool"]],[R[1]]]],[11,"pic",E,"Configures whether the compiler will emit position…",1,[[["self"],["bool"]],[R[1]]]],[11,"use_plt",E,"Configures whether the Procedure Linkage Table is used for…",1,[[["self"],["bool"]],[R[1]]]],[11,"static_crt",E,"Configures whether the /MT flag or the /MD flag will be…",1,[[["self"],["bool"]],[R[1]]]],[11,"try_compile",E,R[4],1,[[["self"],["str"]],[R[2],[R[6]]]]],[11,"compile",E,R[4],1,[[["self"],["str"]]]],[11,"try_expand",E,"This will return a result instead of panicing; see…",1,[[["self"]],[R[2],["vec",R[6]]]]],[11,"expand",E,"Run the compiler, returning the macro-expanded version of…",1,[[["self"]],["vec",["u8"]]]],[11,"get_compiler",E,R[5],1,[[["self"]],["tool"]]],[11,"try_get_compiler",E,R[5],1,[[["self"]],[R[2],["tool",R[6]]]]],[11,"to_command",E,"Converts this compiler into a `Command` that's ready to be…",2,[[["self"]],[R[7]]]],[11,"path",E,"Returns the path for this compiler.",2,[[["self"]],["path"]]],[11,"args",E,"Returns the default set of arguments to the compiler…",2,N],[11,"env",E,"Returns the set of environment variables needed for this…",2,N],[11,"cc_env",E,"Returns the compiler command in format of CC environment…",2,[[["self"]],[R[8]]]],[11,"cflags_env",E,"Returns the compiler flags in format of CFLAGS environment…",2,[[["self"]],[R[8]]]],[11,"is_like_gnu",E,"Whether the tool is GNU Compiler Collection-like.",2,[[["self"]],["bool"]]],[11,"is_like_clang",E,"Whether the tool is Clang-like.",2,[[["self"]],["bool"]]],[11,"is_like_msvc",E,"Whether the tool is MSVC-like.",2,[[["self"]],["bool"]]],[11,"from",E,E,1,[[[T]],[T]]],[11,"into",E,E,1,[[["self"]],[U]]],[11,R[9],E,E,1,[[["self"]],[T]]],[11,R[10],E,E,1,N],[11,R[11],E,E,1,[[[U]],[R[2]]]],[11,"borrow",E,E,1,[[["self"]],[T]]],[11,R[14],E,E,1,[[["self"]],["typeid"]]],[11,R[12],E,E,1,[[["self"]],[T]]],[11,R[13],E,E,1,[[["self"]],[R[2]]]],[11,"from",E,E,3,[[[T]],[T]]],[11,"into",E,E,3,[[["self"]],[U]]],[11,R[9],E,E,3,[[["self"]],[T]]],[11,R[10],E,E,3,N],[11,R[11],E,E,3,[[[U]],[R[2]]]],[11,"borrow",E,E,3,[[["self"]],[T]]],[11,R[14],E,E,3,[[["self"]],["typeid"]]],[11,R[12],E,E,3,[[["self"]],[T]]],[11,R[13],E,E,3,[[["self"]],[R[2]]]],[11,"from",E,E,2,[[[T]],[T]]],[11,"into",E,E,2,[[["self"]],[U]]],[11,R[9],E,E,2,[[["self"]],[T]]],[11,R[10],E,E,2,N],[11,R[11],E,E,2,[[[U]],[R[2]]]],[11,"borrow",E,E,2,[[["self"]],[T]]],[11,R[14],E,E,2,[[["self"]],["typeid"]]],[11,R[12],E,E,2,[[["self"]],[T]]],[11,R[13],E,E,2,[[["self"]],[R[2]]]],[11,"from",R[15],E,0,[[[T]],[T]]],[11,"into",E,E,0,[[["self"]],[U]]],[11,R[9],E,E,0,[[["self"]],[T]]],[11,R[10],E,E,0,N],[11,R[11],E,E,0,[[[U]],[R[2]]]],[11,"borrow",E,E,0,[[["self"]],[T]]],[11,R[14],E,E,0,[[["self"]],["typeid"]]],[11,R[12],E,E,0,[[["self"]],[T]]],[11,R[13],E,E,0,[[["self"]],[R[2]]]],[11,"eq",E,E,0,[[["self"],["vsvers"]],["bool"]]],[11,"default","cc",E,1,[[],[R[1]]]],[11,"clone",R[15],E,0,[[["self"]],["vsvers"]]],[11,"clone","cc",E,1,[[["self"]],[R[1]]]],[11,"clone",E,E,3,[[["self"]],[R[6]]]],[11,"clone",E,E,2,[[["self"]],["tool"]]],[11,"from",E,E,3,[[[R[6]]],[R[6]]]],[11,"fmt",R[15],E,0,[[["self"],[R[16]]],[R[2]]]],[11,"fmt","cc",E,1,[[["self"],[R[16]]],[R[2]]]],[11,"fmt",E,E,3,[[["self"],[R[16]]],[R[2]]]],[11,"fmt",E,E,2,[[["self"],[R[16]]],[R[2]]]]],"p":[[4,"VsVers"],[3,"Build"],[3,"Tool"],[3,"Error"]]};
searchIndex["gcc_shim"]={"doc":E,"i":[],"p":[]};
initSearch(searchIndex);addSearchOptions(searchIndex);