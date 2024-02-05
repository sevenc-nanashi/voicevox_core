import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import fs from "node:fs/promises";

// https://vitejs.dev/config/
export default defineConfig(async () => {
  const releaseFiles = await fs.readdir(
    "../../../target/wasm32-unknown-emscripten/release"
  );
  const wasmFile = releaseFiles.find((file) => file.endsWith(".wasm"));
  const jsFile = releaseFiles.find((file) => file.endsWith(".js"));
  const wasmPath = `../../../target/wasm32-unknown-emscripten/release/${wasmFile}`;
  const jsPath = `../../../target/wasm32-unknown-emscripten/release/${jsFile}`;
  await fs.copyFile(wasmPath, "./public/voicevox_core_wasm_api.wasm");
  await fs.copyFile(jsPath, "./public/voicevox_core_wasm_api.js");

  return { plugins: [vue()] };
});
