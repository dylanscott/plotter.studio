import { defineConfig } from "@rspack/cli";
import { rspack } from "@rspack/core";

export default defineConfig({
  entry: {
    main: "./src/index.ts",
  },
  plugins: [
    new rspack.HtmlRspackPlugin({
      title: "plotter.studio",
    }),
  ],
  experiments: {
    asyncWebAssembly: true,
  },
});
