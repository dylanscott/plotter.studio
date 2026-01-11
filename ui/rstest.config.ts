import { withRsbuildConfig } from "@rstest/adapter-rsbuild";
import { defineConfig } from "@rstest/core";

export default defineConfig({
  extends: withRsbuildConfig({ cwd: __dirname }),
  testEnvironment: "jsdom",
  setupFiles: ["./rstest.setup.ts"],
});
