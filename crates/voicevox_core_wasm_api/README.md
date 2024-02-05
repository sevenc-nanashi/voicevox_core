# voicevox core wasm api

- `EXPORTED_FUNCTIONS`足すのが面倒なので、`message`という関数を呼び出すようにしてる
- `message`は JSON を取って文字列を返す。
- JSON の構造は web/src/voicevoxCore.ts を参照

## Rust 側準備

- onnxruntime の wasm ビルドが必要
  - https://onnxruntime.ai/docs/build/web.html
  - `--build_wasm_static_lib`をつけてビルドする
- OpenJtalk 辞書を zip にしたやつが必要
  - src/openjtalk_dict.zip に配置

## Vite 側準備

- pnpm を使う（個人の好み）
- public/下に色々入れる
  - sample.vvm
  - voicevox_core_wasm_api.js
  - voicevox_core_wasm_api.wasm

## ビルド

```
export ORT_LIB_LOCATION=/home/sevenc7c/voicevox/wasm/onnxruntime/build/Linux/Release
cargo build --target wasm32-unknown-emscripten -p voicevox_core_wasm_api --release $@
cp ./target/wasm32-unknown-emscripten/release/voicevox_core_wasm_api.{js,wasm} ./crates/voicevox_core_wasm_api/__gi_test_web/public
```
