<script lang="ts">
  import type { Length } from "plotter-wasm";

  import PreviewContainer from "./components/PreviewContainer.svelte";
  import ControlPanel from "./components/ControlPanel.svelte";
</script>

<div class="app">
  <div class="preview-container">
    <PreviewContainer
      plotterWidth={11 as Length<"in">}
      plotterHeight={8.5 as Length<"in">}
    />
  </div>
  <div class="control-panel">
    <ControlPanel />
  </div>
</div>

<style>
  .app {
    display: flex;
    width: 100vw;
    height: 100vh;
    overflow: hidden;
  }

  .preview-container {
    width: 65%;
    height: 100%;
  }

  .control-panel {
    width: 35%;
    height: 100%;
    background-color: var(--bg-secondary);
  }
</style>
