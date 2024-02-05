#!/usr/bin/env bash

# LC_ALL="C" \
#   PATH="/home/sevenc7c/.rustup/toolchains/1.74.0-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin:/home/sevenc7c/.rustup/toolchains/1.74.0-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/self-contained:/home/sevenc7c/emsdk:/home/sevenc7c/emsdk/upstream/emscripten:/home/sevenc7c/.pyenv/versions/3.9.16/bin:/home/sevenc7c/.pyenv/libexec:/home/sevenc7c/.pyenv/plugins/python-build/bin:/home/sevenc7c/.pyenv/plugins/pyenv-virtualenv/bin:/home/sevenc7c/.pyenv/plugins/pyenv-update/bin:/home/sevenc7c/.pyenv/plugins/pyenv-installer/bin:/home/sevenc7c/.pyenv/plugins/pyenv-doctor/bin:/home/sevenc7c/.bun/bin:/home/sevenc7c/.deno/bin:/home/sevenc7c/.rbenv/shims:/home/sevenc7c/.rbenv/bin:/home/sevenc7c/.pyenv/shims:/home/sevenc7c/.pyenv/bin:/home/sevenc7c/.local/bin:/home/sevenc7c/.nvm/versions/node/v18.18.2/bin:/home/sevenc7c/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/games:/usr/local/games:/usr/lib/wsl/lib:/snap/bin:/home/sevenc7c/go/bin:/home/sevenc7c/.local/opt/gradle/bin:/home/sevenc7c/android-ndk-r25c/toolchains/llvm/prebuilt/linux-x86_64/bin:/usr/lib/x86_64-linux-gnu/libgtk-3-0:/usr/lib/x86_64-linux-gnu/libgtk2.0-0:/home/sevenc7c/android-ndk-r25c/toolchains/llvm/prebuilt/linux-x86_64/bin" VSLANG="1033" \
#   "emcc" "-s" "EXPORTED_FUNCTIONS=[\"___externref_drop_slice\",\"___externref_heap_live_count\",\"___externref_table_alloc\",\"___externref_table_dealloc\",\"___wbindgen_exn_store\",\"___wbindgen_free\",\"___wbindgen_malloc\",\"___wbindgen_realloc\"]" "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/release/deps/voicevox_core.voicevox_core.818581907927ba-cgu.0.rcgu.o" "-L" "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/release/deps" "-L" "/home/sevenc7c/voicevox/wasm/core/target/release/deps" "-L" "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/release/build/open_jtalk-sys-24ef7e1a82f1779a/out/lib" "-L" "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/release/build/link-cplusplus-a81c9efb741f5051/out" "-L" "/home/sevenc7c/voicevox/wasm/onnxruntime/build/Linux/Release" "-L" "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/release/build/libz-sys-321d4405162debb6/out/lib" "-L" "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/release/build/libz-sys-321d4405162debb6/out/lib" "-L" "/home/sevenc7c/.rustup/toolchains/1.74.0-x86_64-unknown-linux-gnu/lib/rustlib/wasm32-unknown-emscripten/lib" "/tmp/rustcYsQ2TW/liblibz_sys-98fd721fd0cebcef.rlib" "/tmp/rustcYsQ2TW/libort-3975b8ab6373012b.rlib" "/tmp/rustcYsQ2TW/liblink_cplusplus-e4f1fabff3eee31c.rlib" "/home/sevenc7c/.rustup/toolchains/1.74.0-x86_64-unknown-linux-gnu/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-3dcba3b1153fa952.rlib" "-l" "stdc++" "-l" "openjtalk" "-l" "stdc++" "-l" "c" "-fuse-ld=lld" "--target=wasm32-unknown-emscripten" "-s" "DISABLE_EXCEPTION_CATCHING=0" "-L" "/home/sevenc7c/.rustup/toolchains/1.74.0-x86_64-unknown-linux-gnu/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/home/sevenc7c/.rustup/toolchains/1.74.0-x86_64-unknown-linux-gnu/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-o" "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/release/deps/voicevox_core.wasm" "-Oz" "-g0" "-sABORTING_MALLOC=0" "-Wl,--fatal-warnings" --no-entry
# BINARYEN="/home/sevenc7c/.local/share/cargo-web/binaryen/x86_64-unknown-linux-gnu/binaryen" \
#   CARGO_WEB_TARGET="wasm32-unknown-emscripten" \
#   CARGO_WEB_TARGET_DIR="/home/sevenc7c/voicevox/wasm/core/target" \
#   COMPILING_UNDER_CARGO_WEB="1" \
#   EMSCRIPTEN="/home/sevenc7c/.local/share/cargo-web/emscripten/x86_64-unknown-linux-gnu/emscripten" \
#   EMSCRIPTEN_FASTCOMP="/home/sevenc7c/.local/share/cargo-web/emscripten/x86_64-unknown-linux-gnu/emscripten-fastcomp" \
#   LLVM="/home/sevenc7c/.local/share/cargo-web/emscripten/x86_64-unknown-linux-gnu/emscripten-fastcomp" \
#   PATH="/home/sevenc7c/.local/share/cargo-web/emscripten/x86_64-unknown-linux-gnu/emscripten:/home/sevenc7c/emsdk:/home/sevenc7c/emsdk/upstream/emscripten:/home/sevenc7c/.pyenv/versions/3.9.16/bin:/home/sevenc7c/.pyenv/libexec:/home/sevenc7c/.pyenv/plugins/python-build/bin:/home/sevenc7c/.pyenv/plugins/pyenv-virtualenv/bin:/home/sevenc7c/.pyenv/plugins/pyenv-update/bin:/home/sevenc7c/.pyenv/plugins/pyenv-installer/bin:/home/sevenc7c/.pyenv/plugins/pyenv-doctor/bin:/home/sevenc7c/.bun/bin:/home/sevenc7c/.deno/bin:/home/sevenc7c/.rbenv/shims:/home/sevenc7c/.rbenv/bin:/home/sevenc7c/.pyenv/shims:/home/sevenc7c/.pyenv/bin:/home/sevenc7c/.local/bin:/home/sevenc7c/.nvm/versions/node/v18.18.2/bin:/home/sevenc7c/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/games:/usr/local/games:/usr/lib/wsl/lib:/snap/bin:/home/sevenc7c/go/bin:/home/sevenc7c/.local/opt/gradle/bin:/home/sevenc7c/android-ndk-r25c/toolchains/llvm/prebuilt/linux-x86_64/bin:/usr/lib/x86_64-linux-gnu/libgtk-3-0:/usr/lib/x86_64-linux-gnu/libgtk2.0-0:/home/sevenc7c/android-ndk-r25c/toolchains/llvm/prebuilt/linux-x86_64/bin" \
#   RUSTFLAGS="--cfg cargo_web -C link-arg=-s -C link-arg=NO_EXIT_RUNTIME=1 -C link-arg=-s -C link-arg=ALLOW_MEMORY_GROWTH=1  -o ./test.js" \
#   "cargo" "rustc" "--color" "always" "--target" "wasm32-unknown-emscripten" "--package" "voicevox_core_wasm_api" "--profile" "dev" "--lib" "--features" "" 
# LC_ALL="C"\
#   PATH="/home/sevenc7c/.rustup/toolchains/1.74.0-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin:/home/sevenc7c/.rustup/toolchains/1.74.0-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/self-contained:/home/sevenc7c/.local/share/cargo-web/emscripten/x86_64-unknown-linux-gnu/emscripten:/home/sevenc7c/emsdk:/home/sevenc7c/emsdk/upstream/emscripten:/home/sevenc7c/.pyenv/versions/3.9.16/bin:/home/sevenc7c/.pyenv/libexec:/home/sevenc7c/.pyenv/plugins/python-build/bin:/home/sevenc7c/.pyenv/plugins/pyenv-virtualenv/bin:/home/sevenc7c/.pyenv/plugins/pyenv-update/bin:/home/sevenc7c/.pyenv/plugins/pyenv-installer/bin:/home/sevenc7c/.pyenv/plugins/pyenv-doctor/bin:/home/sevenc7c/.bun/bin:/home/sevenc7c/.deno/bin:/home/sevenc7c/.rbenv/shims:/home/sevenc7c/.rbenv/bin:/home/sevenc7c/.pyenv/shims:/home/sevenc7c/.pyenv/bin:/home/sevenc7c/.local/bin:/home/sevenc7c/.nvm/versions/node/v18.18.2/bin:/home/sevenc7c/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/games:/usr/local/games:/usr/lib/wsl/lib:/snap/bin:/home/sevenc7c/go/bin:/home/sevenc7c/.local/opt/gradle/bin:/home/sevenc7c/android-ndk-r25c/toolchains/llvm/prebuilt/linux-x86_64/bin:/usr/lib/x86_64-linux-gnu/libgtk-3-0:/usr/lib/x86_64-linux-gnu/libgtk2.0-0:/home/sevenc7c/android-ndk-r25c/toolchains/llvm/prebuilt/linux-x86_64/bin"\
#   VSLANG="1033" \
#   "emcc" \
#   "-s" \
#   "EXPORTED_FUNCTIONS=[\"___externref_drop_slice\",\"___externref_heap_live_count\",\"___externref_table_alloc\",\"___externref_table_dealloc\",\"___wbindgen_exn_store\",\"___wbindgen_free\",\"___wbindgen_malloc\",\"___wbindgen_realloc\"]" \
#   "./test.2ocla09q95agnpqn.rcgu.o" \
#   "./test.2wvn63rmyvou3tbo.rcgu.o" \
#   "./test.3olnhb33txi8rrhv.rcgu.o" \
#   "./test.4458ki9krazuel1w.rcgu.o" \
#   "./test.4v44g4lng1h3iee2.rcgu.o" \
#   "./test.3a4muc5qf59z9mhv.rcgu.o" \
#   "-L" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps" \
#   "-L" \
#   "/home/sevenc7c/voicevox/wasm/core/target/debug/deps" \
#   "-L" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/build/open_jtalk-sys-7269483ede38add2/out/lib" \
#   "-L" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/build/link-cplusplus-cfecbe266bfd1047/out" \
#   "-L" \
#   "/home/sevenc7c/voicevox/wasm/onnxruntime/build/Linux/Release" \
#   "-L" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/build/libz-sys-e7a1dcc8961dfabc/out/lib" \
#   "-L" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/build/libz-sys-e7a1dcc8961dfabc/out/lib" \
#   "-L" \
#   "/home/sevenc7c/.rustup/toolchains/1.74.0-x86_64-unknown-linux-gnu/lib/rustlib/wasm32-unknown-emscripten/lib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libvoicevox_core-29bcf8b9d9073566.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libtokio-a2e37802074d9433.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libserde_json-d79e2cb842a3f4dc.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libryu-17aa25a08cc3da75.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libitoa-2410e7c75aa43d3c.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libindexmap-732b174fdd40285b.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libhashbrown-1e46cd0ab2466cbd.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libnanoid-2739c66850f772eb.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/librand-74f29f3e0a5e7cf0.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/librand_chacha-c77b98229fd84c61.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libppv_lite86-9a8f7d556ebf47a2.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/librand_core-d9d2703f00fd2d5c.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libzip-131f3575e5845ca1.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libflate2-3e9089a151d3034d.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/liblibz_sys-430039f8adaf6b3c.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libbyteorder-d0d6b5de39f2538d.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libcrc32fast-c8d47ce1de658c44.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libfutures-a6323982b3945067.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libfutures_executor-60c0328f8475bf67.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libfutures_util-0600ed7249fb6797.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libfutures_io-307473a1ba0f26a1.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libslab-fb299d7db20d9473.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libfutures_channel-dc7983e9f7fa6e5f.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libfutures_sink-408b7ce938ac3104.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libfutures_task-3584e1225a3cce64.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libpin_utils-3ab1f23dbb31d363.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libfutures_core-1187ea25c39555df.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libindexmap-de34e2a49c7998e2.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libequivalent-73ddaf34fafe8511.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libhashbrown-ddad945601183502.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libfs_err-70f3555fabb1b570.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libitertools-69c2dbb102e59a6a.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libeither-49ab0b410ab43689.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libort-f3c87b510e337bbc.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libhalf-0798b10039d86d00.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libtracing-aef9e4b37d5953d2.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/liblog-a6da658dcb2a295d.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libpin_project_lite-400947df1d2b9d76.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libtracing_core-a0d81c36e0652e1f.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/liblazy_static-08678406e4e49f5d.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libndarray-774284901117e9bf.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libmatrixmultiply-a089b6998b12b604.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libnum_complex-6764eb12fa34a132.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libnum_integer-43c08505eee84878.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libnum_traits-56b89e7fee5243ca.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/librawpointer-abbba67fb6dd65b9.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libenum_map-9f75646677015da3.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libuuid-5945f92f7c761f5f.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libgetrandom-c97465b97330a01a.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libopen_jtalk-9f0b994c0b24b875.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libopen_jtalk_sys-1ba6933567645802.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/liblink_cplusplus-31e063a1aa8a738e.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libthiserror-8e0dfaf67653e4b4.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libtempfile-6b8748c83570dda2.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libfastrand-faaeaf3f91392203.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libinstant-e69368741cab1b11.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/librustix-61c4f63efe26dfc2.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libbitflags-3e70f3efc5b6d6a6.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/liberrno-e4944b823d3c8d4a.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libio_lifetimes-70edf71b2118808a.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/liblibc-d09b9e71dc5b85e6.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libanyhow-006d7ff3b3ef5656.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libregex-2bc51af103e310d9.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libregex_automata-039228c6ebc78894.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libaho_corasick-9f73191160b73660.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libmemchr-40949b36f8855e32.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libregex_syntax-76bde550cf2fdd23.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libonce_cell-700e4c806eb169b3.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libserde-5853304b30e1cfbe.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libwasm_bindgen-5937b8ab76fba06f.rlib" \
#   "/home/sevenc7c/voicevox/wasm/core/target/wasm32-unknown-emscripten/debug/deps/libcfg_if-14f5693aa6900da8.rlib" \
#   "/home/sevenc7c/.rustup/toolchains/1.74.0-x86_64-unknown-linux-gnu/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-9cd6b2c45b002369.rlib" \
#   "/home/sevenc7c/.rustup/toolchains/1.74.0-x86_64-unknown-linux-gnu/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-44b2308dede5a1b0.rlib" \
#   "/home/sevenc7c/.rustup/toolchains/1.74.0-x86_64-unknown-linux-gnu/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-c8a5ee6ea5f2f3b3.rlib" \
#   "/home/sevenc7c/.rustup/toolchains/1.74.0-x86_64-unknown-linux-gnu/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-6d84014db14c429c.rlib" \
#   "/home/sevenc7c/.rustup/toolchains/1.74.0-x86_64-unknown-linux-gnu/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-921d6fc620b9ecfb.rlib" \
#   "/home/sevenc7c/.rustup/toolchains/1.74.0-x86_64-unknown-linux-gnu/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-1da87fb2387ab600.rlib" \
#   "/home/sevenc7c/.rustup/toolchains/1.74.0-x86_64-unknown-linux-gnu/lib/rustlib/wasm32-unknown-emscripten/lib/libminiz_oxide-52de0721570f2250.rlib" \
#   "/home/sevenc7c/.rustup/toolchains/1.74.0-x86_64-unknown-linux-gnu/lib/rustlib/wasm32-unknown-emscripten/lib/libadler-18edc3e6a4fbba93.rlib" \
#   "/home/sevenc7c/.rustup/toolchains/1.74.0-x86_64-unknown-linux-gnu/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-8a78e521e26a0ae0.rlib" \
#   "/home/sevenc7c/.rustup/toolchains/1.74.0-x86_64-unknown-linux-gnu/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-4012f4405b589216.rlib" \
#   "/home/sevenc7c/.rustup/toolchains/1.74.0-x86_64-unknown-linux-gnu/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-50e9ff29d780ea46.rlib" \
#   "/home/sevenc7c/.rustup/toolchains/1.74.0-x86_64-unknown-linux-gnu/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-70741ff116fa2869.rlib" \
#   "/home/sevenc7c/.rustup/toolchains/1.74.0-x86_64-unknown-linux-gnu/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-a75cbd20fbaa7368.rlib" \
#   "/home/sevenc7c/.rustup/toolchains/1.74.0-x86_64-unknown-linux-gnu/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-1b0e09c77ad5e928.rlib" \
#   "/home/sevenc7c/.rustup/toolchains/1.74.0-x86_64-unknown-linux-gnu/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-3dcba3b1153fa952.rlib" \
#   "-l" \ 
#   "stdc++" \
#   "-l" \
#   "openjtalk" \
#   "-l" \
#   "stdc++" \
#   "-l" \
#   "c" \
#   "-fuse-ld=lld" \
#   "--target=wasm32-unknown-emscripten" \
#   "-s" \
#   "DISABLE_EXCEPTION_CATCHING=0" \
#   "-L" \
#   "/home/sevenc7c/.rustup/toolchains/1.74.0-x86_64-unknown-linux-gnu/lib/rustlib/wasm32-unknown-emscripten/lib" \
#   "-L" \
#   "/home/sevenc7c/.rustup/toolchains/1.74.0-x86_64-unknown-linux-gnu/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" \
#   "-o" \
#   "./voicevox_core_wasm_api.js" \
#   "-O0" \
#   "-g" \
#   "-s" \
#   "NO_EXIT_RUNTIME=1" \
#   "-s" \
#   "ALLOW_MEMORY_GROWTH=1" \
#   "--no-entry" \
#   "-sERROR_ON_UNDEFINED_SYMBOLS=0" \
#   "-sABORTING_MALLOC=0" \
#   "-Wl,--fatal-warnings"
#
# export EMCC_CFLAGS="-o ./target/wasm32-unknown-emscripten/release/voicevox_core_wasm_api.js
#                     -s EXPORT_ES6=1
#                     -s MODULARIZE=1
#                     -s 'EXPORT_NAME=\"RawVoicevoxCore\"'
#                     -s EXPORTED_FUNCTIONS=['_greet']
#                     -s EXPORTED_RUNTIME_METHODS=ccall"
# ビルド
set -eux
export ORT_LIB_LOCATION=/home/sevenc7c/voicevox/wasm/onnxruntime/build/Linux/Release
cargo build --target wasm32-unknown-emscripten -p voicevox_core_wasm_api $@
cp ./target/wasm32-unknown-emscripten/debug/voicevox_core_wasm_api.{js,wasm} ./crates/voicevox_core_wasm_api/__gi_test_web/public
wasm2wat ./target/wasm32-unknown-emscripten/debug/voicevox_core_wasm_api.wasm --generate-names > ./__gi_test.wat
npx prettier --write ./crates/voicevox_core_wasm_api/__gi_test_web/public/voicevox_core_wasm_api.js
ruby wasm_build_replace.rb
