use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let lib_path = manifest_dir.join("..").join("libs").join("sdl2_ttf").join("SDL2_ttf-2.22.0").join("lib").join("x64");
    
    println!("cargo:rustc-link-search=native={}", lib_path.display());
    println!("cargo:rerun-if-changed=build.rs");
}
