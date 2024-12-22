import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import { viteStaticCopy as staticCopy } from "vite-plugin-static-copy";

// https://vitejs.dev/config/
export default defineConfig(async () => {
  return {
    plugins: [
      vue(),
      staticCopy({
        targets: [
          {
            src: "./node_modules/onnxruntime-web/dist/*.wasm",
            dest: "./",
          },
        ],
      }),
    ],
    server: {
      headers: {
        "Cross-Origin-Opener-Policy": "same-origin",
        "Cross-Origin-Embedder-Policy": "require-corp",
      },
    },
  };
});
