extern crate cxx_build;
use std::env;

#[allow(clippy::if_same_then_else, clippy::needless_bool)]
fn infer_static(name: &str) -> bool {
    if std::env::var(format!("{}_STATIC", name.to_uppercase())).is_ok() {
        true
    } else if std::env::var(format!("{}_DYNAMIC", name.to_uppercase())).is_ok() {
        false
    } else if std::env::var("PKG_CONFIG_ALL_STATIC").is_ok() {
        true
    } else if std::env::var("PKG_CONFIG_ALL_DYNAMIC").is_ok() {
        false
    } else {
        false
    }
}

fn main() {
    println!("cargo:rustc-cfg=verbose");
    match env::var("DOCS_RS").as_deref() {
        Ok("1") => (),
        _ => {
            let mut build = cxx_build::bridge("src/cxx_aoflagger.rs");
            build
                .flag_if_supported("-std=c++11")
                // .flag_if_supported("-Wno-nonportable-include-path")
                .flag_if_supported("-Wno-unused-parameter")
                .file("src/cxx_aoflagger.cc");
            // Tell cargo to search for includes in AOFLAGGER_INCLUDE_DIR, if
            // it's defined.
            // Otherwise if we're on macos, check homebrew.
            if let Ok(dir) = std::env::var("AOFLAGGER_INCLUDE_DIR") {
                build.include(dir);
            } else if let Ok(dir) = std::env::var("HOMEBREW_PREFIX") {
                build.include(format!("{}/include", dir));
            }

            build.compile("cxx_aoflagger");
            // Tell cargo to tell rustc to link the aoflagger library.
            if infer_static("aoflagger") {
                println!("cargo:rustc-link-lib=static=aoflagger");
            } else {
                println!("cargo:rustc-link-lib=dylib=aoflagger");
            }
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
