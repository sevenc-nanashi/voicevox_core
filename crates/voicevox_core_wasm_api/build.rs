fn main() {
    println!("cargo:rustc-link-arg=--no-entry");
    println!("cargo:rustc-link-arg=-sERROR_ON_UNDEFINED_SYMBOLS=0");
    println!("cargo:rustc-link-arg=-sEXPORTED_FUNCTIONS=['_message']");
    println!("cargo:rustc-link-arg=-sEXPORTED_RUNTIME_METHODS=['ccall']");
    println!("cargo:rustc-link-arg=-sEXPORT_NAME=\"RawVoicevoxCore\"");
    println!("cargo:rustc-link-arg=-sMODULARIZE=1");
    println!("cargo:rustc-link-arg=-sALLOW_MEMORY_GROWTH=1");
    println!("cargo:rustc-link-arg=-sTOTAL_STACK=128MB");
    println!("cargo:rustc-link-arg=-sINITIAL_MEMORY=256MB");
    println!("cargo:rustc-link-arg=-sASSERTIONS=2");
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let target_dir = out_dir
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    println!(
        "cargo:rustc-link-arg=-o{}/voicevox_core_wasm_api.js",
        target_dir.display()
    );

    println!("cargo:rustc-link-arg=-DEMSCRIPTEN_STANDALONE_WASM");
}
