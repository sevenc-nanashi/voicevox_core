fn main() {
    println!("cargo:rustc-link-arg=--no-entry");
    println!("cargo:rustc-link-arg=-sERROR_ON_UNDEFINED_SYMBOLS=0");
    println!("cargo:rustc-link-arg=-sEXPORTED_FUNCTIONS=['_message']");
    println!("cargo:rustc-link-arg=-sEXPORTED_RUNTIME_METHODS=['ccall']");
    println!("cargo:rustc-link-arg=-sMODULARIZE=1");
    println!("cargo:rustc-link-arg=-sEXPORT_ES6=1");
    println!("cargo:rustc-link-arg=-s'EXPORT_NAME=\"RawVoicevoxCore\"'");
}
