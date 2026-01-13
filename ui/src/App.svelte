<script lang="ts">
  import CanvasContainer from "./components/CanvasContainer.svelte";
  import ControlPanel from "./components/ControlPanel.svelte";
</script>

<div class="app">
  <div class="canvas-container">
    <CanvasContainer />
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

  .canvas-container {
    width: 65%;
    height: 100%;
  }

  .control-panel {
    width: 35%;
    height: 100%;
    background-color: var(--bg-secondary);
  }
</style>
