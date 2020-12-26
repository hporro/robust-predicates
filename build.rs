use std::env;
use std::path::{Path};
use cc;

// heavily inspired by 
// https://github.com/aircloud/rust-c-demo/blob/master/hello-from-generated-code-3/build.rs

fn main() {
    cc::Build::new()
        .file("src/predicates.c")
        .compile("predicates");

    let pwd_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = Path::new(&*pwd_dir).join("lib");
    println!("cargo:rustc-link-search=native={}", path.to_str().unwrap());
    println!("cargo:rustc-link-lib=dylib=predicates");
}