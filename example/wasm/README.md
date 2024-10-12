# Web サンプルコード (emscripten によるバインディング経由)

voicevox_core ライブラリ の WebAssembly バインディングを使った音声合成のサンプルコードです。

1. 以下のスクリプトを実行する：

```bash
set -eux
cargo build -p voicevox_core_c_api --target wasm32-unknown-emscripten --profile web-release
cp ./target/wasm32-unknown-emscripten/web-release/voicevox_core_wasm_api.{mjs,wasm} ./example/wasm/src/artifacts/
```

2. OpenJtalk の辞書を zip にして、`./example/wasm/public/` に配置する

```bash
cd crates/test_util/data
zip -r ../../../example/wasm/public/open_jtalk_dic.zip open_jtalk_dic
```

3. sample.vvm を `./example/wasm/public/` に配置する
4. `pnpm install` する

## 記録

- 自前ビルドした。
  - 多分高速化が1.17.1から1.18.0の間に入ってる。
- マルチスレッドwasmで「ハローワールド」を合成するのに5.4秒くらい。
- WebGPU/WebGL/WebNNはどちらも動かない。
  - WebGL は int64 非対応、なんとかバイパスしても Where に対応してないので死。
    - https://github.com/microsoft/onnxruntime/blob/main/js/web/docs/webgl-operators.md
  - WebGPU：
    - crates/voicevox_core/wasm_library.js の `import('onnxruntime-web')` を `import('onnxruntime-web/webgpu')` にしないと Execution Provider が認識されないので注意。
    - [やけにアクセシビリティの低いメッセージ](https://files.slack.com/files-pri/T03C4RC8V-F06PUEAE8PP/image.png)が出たりする。
    - `Conv_515` でクラッシュ。Conv1d/Conv2d しか対応してないって書いてあるけど、多分そういう問題ではない。
      - ソースは勘。
      - 多分どこかでクラッシュして、「大体クラッシュするのは Conv1d/2d 以外を呼び出したときだろ」って判断で書かれた Onnxruntime コードがそういうエラーメッセージを出してる。
    - decode.onnx だけ[wonnx](https://github.com/webonnx/wonnx)を使うのも試したけど失敗。
      - `ai.onnx.ml` に対応してない。
        - https://github.com/webonnx/wonnx/issues/156
  - WebNN：
    - 前提：`about:flags`から WebNN を有効にする。
    - numberがthrowされる謎のエラーが出る。
- 2024/10/12：動いた！
  - 1.17まで戻した。
  - WebGL：落ちた：`input tensor[0] check failed: expected shape '[,]' but got [94,80]`
    - 謎。
  - WebGPU：動いた！
    - とはいってもそんなに早くならなかった。残念。
  - WebNN：1.17まで下げた影響により、存在が抹消されたのでN/A。
