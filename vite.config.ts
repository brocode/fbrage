import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";
import wasm from "vite-plugin-wasm";

export default defineConfig({
  plugins: [wasm(), sveltekit()],
  define: {
    __FBRAGE_BUILD_DATE__: JSON.stringify(
      new Intl.DateTimeFormat("en-GB", { dateStyle: "medium", timeStyle: "short", timeZone: "Europe/Berlin" }).format(
        new Date(),
      ),
    ),
  },
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
