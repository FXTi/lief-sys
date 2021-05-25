extern crate bindgen;
extern crate cmake;

use cmake::Config;
use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir_path = {
        let out_dir =
            env::var("OUT_DIR").expect("Unable to get value of OUT_DIR environment variable");
        PathBuf::from(out_dir)
    };

    let target = env::var("TARGET").unwrap();

    // Run cmake to build lief
    let mut config = Config::new("lief");
    config
        .generator("Ninja")
        .define("CMAKE_BUILD_TYPE", "Release")
        .define("BUILD_SHARED_LIBS", "on")
        .define("LIEF_PYTHON_API", "off")
        .define("LIEF_EXAMPLES", "off")
        .define("LIEF_USE_CCACHE", "off")
        .define("LIEF_INSTALL_COMPILED_EXAMPLES", "off");
    if target.contains("windows") {
        config.cxxflag("/EHsc");
    }
    let dst = config.build();

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    println!("cargo:rustc-link-lib=dylib=LIEF");

    let lief_c_header = {
        let path = out_dir_path.join("include").join("LIEF").join("LIEF.h");
        if !path.exists() {
            panic!("LIEF C header not found");
        }
        path
    };

    let bindings = bindgen::Builder::default()
        .header(lief_c_header.to_string_lossy())
        .clang_arg(format!("-I{}", dst.join("include").display()))
        .prepend_enum_name(false)
        .rustified_enum("*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_dir_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    if target.contains("darwin") {
        println!("cargo:rustc-link-lib=dylib=c++");
    } else if target.contains("linux") {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    }
}
