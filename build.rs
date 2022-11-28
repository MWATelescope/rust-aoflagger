extern crate cxx_build;
use std::env;

fn main() {
    println!("cargo:rustc-cfg=verbose");
    match env::var("DOCS_RS").as_deref() {
        Ok("1") => (),
        _ => {
            let mut build = cxx_build::bridge("src/cxx_aoflagger.rs");
            build
                .flag_if_supported("-std=c++11")
                .flag_if_supported("-Wno-nonportable-include-path")
                .flag_if_supported("-Wno-unused-parameter")
                .file("src/cxx_aoflagger.cc");
            // Tell cargo to search for includes in AOFLAGGER_INCLUDE_DIR, if
            // it's defined.
            if let Ok(dir) = std::env::var("AOFLAGGER_INCLUDE_DIR") {
                build.include(dir);
            }
            build.compile("cxx_aoflagger");
            // Tell cargo to tell rustc to link the aoflagger shared library.
            println!("cargo:rustc-link-lib=dylib=aoflagger");
            // Tell cargo to search in the library dir AOFLAGGER_LIB, if it's
            // defined.
            if let Ok(lib) = std::env::var("AOFLAGGER_LIB") {
                println!("cargo:rustc-link-search=native={lib}");
            }
        }
    }

    println!("cargo:rerun-if-env-changed=AOFLAGGER_INCLUDE_DIR");
    println!("cargo:rerun-if-env-changed=AOFLAGGER_LIB");

    println!("cargo:rerun-if-changed=src/cxx_aoflagger.rs");
    println!("cargo:rerun-if-changed=src/cxx_aoflagger.cc");

    println!("cargo:rerun-if-changed=build.rs");
}
