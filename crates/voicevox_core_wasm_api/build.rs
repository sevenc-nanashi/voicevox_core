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
    println!("cargo:rustc-link-arg=-o./target/wasm32-unknown-emscripten/release/voicevox_core_wasm_api.js");
    println!("cargo:rustc-link-arg=-DEMSCRIPTEN_STANDALONE_WASM");
}
