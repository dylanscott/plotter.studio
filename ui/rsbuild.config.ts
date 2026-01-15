import { defineConfig } from "@rsbuild/core";
import { pluginSvelte } from "@rsbuild/plugin-svelte";

export default defineConfig({
  plugins: [
    pluginSvelte({
      preprocessOptions: {
        typescript: {
          tsconfigFile: "./tsconfig.json",
        },
      },
    }),
  ],
  source: {
    entry: { index: "./src/index.ts" },
    define: {
      "process.env.RSTEST": false,
    },
  },
  html: {
    title: "plotter.studio",
  },
  tools: {
    rspack: {
      experiments: {
        asyncWebAssembly: true,
        css: true,
      },
    },
  },
  resolve: {
    conditionNames: process.env.RSTEST ? ["browser"] : ["..."],
  },
});
