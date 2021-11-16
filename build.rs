extern crate bindgen;

use std::env;


fn main() {
    let mut builder = bindgen::Builder::default()
        .clang_args(&["-x", "c++"])
        .header("voicevox_core/core.h");

    if let Ok(r) = env::var("SYSROOT") {
        builder = builder.clang_arg(format!("--sysroot={}", r))
    };

    let bindings = builder
        .dynamic_library_name("VoicevoxCore")
        .dynamic_link_require_all(true)
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/voicevox_core_bindings.rs")
        .expect("Couldn't write bindings!");
}