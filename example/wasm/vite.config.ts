import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import fs from "node:fs/promises";

// https://vitejs.dev/config/
export default defineConfig(async () => {
  return { plugins: [vue()] };
});
