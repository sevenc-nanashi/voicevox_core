// TODO: #802 の時点でiOS以外不要になっているはずなので、このbuild.rsは丸ごと消す
// (iOSのためにbuild_util/make_ios_xcframework.bashの修正は必要)
fn main() {
    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    {
        println!("cargo:rustc-link-arg=-Wl,-rpath,@loader_path/");
        println!("cargo:rustc-link-arg=-Wl,-install_name,@rpath/libvoicevox_core.dylib");
    }

    if std::env::var("TARGET").unwrap() == "wasm32-unknown-emscripten" {
        use regex::Regex;
        use std::fs;
        // println!("cargo:rustc-link-arg=-sERROR_ON_UNDEFINED_SYMBOLS=0");
        // // TODO: WARNにしたいけど、これにするとemccがクラッシュする（どうして...）
        // // println!("cargo:rustc-link-arg=-sWARN_ON_UNDEFINED_SYMBOLS=1");
        // println!(
        //     "cargo:rustc-link-arg=-sEXPORTED_FUNCTIONS=['{}']",
        //     functions.join("','")
        // );
        // println!("cargo:rustc-link-arg=-sEXPORTED_RUNTIME_METHODS=['ccall']");
        // println!("cargo:rustc-link-arg=-sEXPORT_NAME=\"RawVoicevoxCore\"");
        // println!("cargo:rustc-link-arg=-sMODULARIZE=1");
        println!("cargo:rustc-link-arg=-sTOTAL_STACK=128MB");
        println!("cargo:rustc-link-arg=-sINITIAL_MEMORY=512MB");
        println!("cargo:rustc-link-arg=-sALLOW_MEMORY_GROWTH=1");

        let re = Regex::new(r#"pub (?:unsafe )?extern "C" fn (\w+)"#).unwrap();
        let mut functions = vec![
            "_malloc".to_string(),
            "_free".to_string(),
            "_setenv".to_string(),
        ];
        let lib_rs = fs::read_to_string("src/lib.rs").unwrap();
        let wasm_rs = fs::read_to_string("src/wasm.rs").unwrap();
        for cap in re.captures_iter(&lib_rs) {
            let line_number = lib_rs[..cap.get(0).unwrap().start()].lines().count();
            let cfg_line = lib_rs.lines().nth(line_number - 2).unwrap();
            if cfg_line.contains("cfg") {
                continue;
            }

            functions.push(format!("_{}", &cap[1]));
        }
        for cap in re.captures_iter(&wasm_rs) {
            functions.push(format!("_{}", &cap[1]));
        }
        println!(
            "cargo:rustc-link-arg=-sEXPORTED_RUNTIME_METHODS=[\"{}\"]",
            [
                "ccall",
                "dynCall",
                "stackSave",
                "stackRestore",
                "stackAlloc",
                "stackFree"
            ]
            .join("\",\"")
        );
        println!(
            "cargo:rustc-link-arg=-sEXPORTED_FUNCTIONS=['{}']",
            functions.join("','")
        );
        // TODO: ちゃんと絞る
        println!("cargo:rustc-link-arg=-sEXPORT_ALL=1");

        // println!("cargo:rustc-link-arg=--no-entry");
        let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
        let target_dir = out_dir
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap();
        println!(
            "cargo:rustc-link-arg=-o{}/voicevox_core_wasm_api.mjs",
            target_dir.display()
        );
        println!("cargo:rustc-link-arg=-sERROR_ON_UNDEFINED_SYMBOLS=0");
        println!("cargo:rustc-link-arg=-sEXPORT_NAME=VoicevoxCore");
        println!("cargo:rustc-link-arg=-sASYNCIFY=1");
        // println!("cargo:rustc-link-arg=-sWASMFS");
        println!("cargo:rustc-link-arg=-sFORCE_FILESYSTEM");
        println!("cargo:rustc-link-arg=--no-entry");

        // 本当はvoicevox_core/build.rsに置きたいけどできない（当社調べ）なのでここに置く
        println!(
            "cargo:rustc-link-arg=--js-library={}",
            std::env::var("CARGO_MANIFEST_DIR").unwrap() + "/../voicevox_core/wasm_library.js"
        );
    }
}
