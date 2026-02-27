import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

export default defineConfig({
  plugins: [react()],
  root: ".",
  build: {
    sourcemap: true,
  },
  optimizeDeps: {
    exclude: ["@dotdm/cdm"],
    entries: ["src/main.tsx"],
  },
});
