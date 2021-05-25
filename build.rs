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

    // Run cmake to build lief
    let dst = Config::new("lief")
        .generator("Ninja")
        .define("CMAKE_BUILD_TYPE", "Release")
        .define("BUILD_SHARED_LIBS", "on")
        .define("LIEF_PYTHON_API", "off")
        .define("LIEF_EXAMPLES", "off")
        .define("LIEF_USE_CCACHE", "off")
        .define("LIEF_INSTALL_COMPILED_EXAMPLES", "off")
        .build();

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );

    let lief_c_header = {
        let path = out_dir_path.join("include").join("LIEF").join("LIEF.h");
        if !path.exists() {
            panic!("LIEF C header not found")
        }
        path
    };

    let bindings = bindgen::Builder::default()
        .header(lief_c_header.to_string_lossy())
        .clang_arg(format!("-I{}", dst.join("include").display()))
        //.prepend_enum_name(false)
        //.rustified_enum("*")
        .derive_partialord(true)
        .no_partialeq(
            "max_align_t|__fsid_t|imaxdiv_t|\
            __crt_locale_data_public|__crt_locale_pointers|__crt_locale_pointers|_Mbstatet|_Lldiv_t|__crt_locale_data|__crt_multibyte_data|\
            Macho_Header_t|Macho_Command_t|Macho_Symbol_t|Macho_Section_t|Macho_Segment_t|Macho_Binary_t|\
            Pe_DosHeader_t|Pe_Header_t|Pe_OptionalHeader_t|Pe_DataDirectory_t|\
            Pe_Section_t|Pe_ImportEntry_t|Pe_Import_t|Pe_Binary_t|\
            Elf_Section_t|Elf_Segment_t|Elf_Header_t|Elf_DynamicEntry_t|Elf_DynamicEntry_Library_t|\
            Elf_DynamicEntry_SharedObject_t|Elf_DynamicEntry_Array_t|Elf_DynamicEntry_Rpath_t|\
            Elf_DynamicEntry_RunPath_t|Elf_DynamicEntry_Flags_t|Elf_Symbol_t|Elf_Binary_t")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_dir_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    #[cfg(target_family = "unix")]
    println!("cargo:rustc-link-lib=dylib=stdc++");
}
