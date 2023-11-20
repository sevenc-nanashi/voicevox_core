fn main() {
    println!("cargo:rustc-link-arg=--no-entry");
    println!("cargo:rustc-link-arg=-sERROR_ON_UNDEFINED_SYMBOLS=0");
}
