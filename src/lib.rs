#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test() {
        unsafe {
            let hello_lief_pathbuf = PathBuf::from("testbins/hello_lief.bin");
            let hello_lief_path = {
                if !hello_lief_pathbuf.exists() {
                    panic!("hello_lief.bin not exists!");
                }
                fs::canonicalize(&hello_lief_pathbuf).expect("fs::canonicalize failed!")
            };
            if let Some(path) = hello_lief_path.to_str() {
                let hello_lief = elf_parse(CString::new(path).unwrap().as_ptr()).as_mut().unwrap();
                assert_eq!(hello_lief.header.file_type, LIEF_ELF_E_TYPE::LIEF_ELF_ET_DYN);
            }
        }
    }
}
