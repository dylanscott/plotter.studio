import { defineConfig } from "@rspack/cli";
import { rspack } from "@rspack/core";

export default defineConfig({
  entry: {
    main: "./src/index.ts",
  },
  experiments: {
    asyncWebAssembly: true,
    css: true,
  },
  resolve: {
    extensions: [".mjs", ".js", ".ts", ".svelte"],
    mainFields: ["svelte", "browser"],
  },
  plugins: [
    new rspack.HtmlRspackPlugin({
      title: "plotter.studio",
    }),
  ],
  module: {
    rules: [
      {
        test: /\.svelte\.ts$/,
        use: ["svelte-loader", "builtin:swc-loader"],
      },
      {
        test: /(?<!\.svelte)\.ts$/,
        loader: "builtin:swc-loader",
      },
      {
        test: /\.(svelte|svelte\.js)$/,
        use: "svelte-loader",
      },
    ],
  },
});
