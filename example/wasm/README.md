# Web サンプルコード (emscripten によるバインディング経由)

voicevox_core ライブラリ の WebAssembly バインディングを使った音声合成のサンプルコードです。

1. [onnxruntime を static library としてビルドする](https://onnxruntime.ai/docs/build/web.html)
2. 以下のスクリプトを実行する：

```bash
set -eux
export ORT_LIB_LOCATION=/path/to/onnxruntime/build/Linux/Release
cargo build -p voicevox_core_c_api --target wasm32-unknown-emscripten -r
cp ./target/wasm32-unknown-emscripten/release/voicevox_core_wasm_api.{mjs,wasm} ./example/wasm/src/artifacts/
```

3. OpenJtalk の辞書を zip にして、`./example/wasm/public/` に配置する

```bash
cd crates/test_util/data
zip -r ../../../example/wasm/public/open_jtalk_dic.zip open_jtalk_dic
```

4. sample.vvm を `./example/wasm/public/` に配置する
