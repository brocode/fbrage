import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";
import wasm from "vite-plugin-wasm";

export default defineConfig({
  plugins: [wasm(), sveltekit()],
  build: {
    target: "esnext",
  },
  server: {
    fs: {
      allow: [
        // search up for workspace root
        //searchForWorkspaceRoot(process.cwd()),
        // your custom rules
        "./rage-webassembly/pkg/",
      ],
    },
  },
});
