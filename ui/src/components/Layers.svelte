<script lang="ts">
  type Pen = { id: string; name: string; color: string; size: number };
  type Layer = { id: string; penId: string; visible: boolean };

  // Mock pens
  let pens = $state<Pen[]>([
    { id: 'pen1', name: 'Black 0.5mm', color: '#1a2332', size: 0.5 },
    { id: 'pen2', name: 'Red 0.3mm', color: '#cc0033', size: 0.3 },
  ]);

  // Mock layers
  let layers = $state<Layer[]>([{ id: 'layer1', penId: 'pen1', visible: true }]);

  function handleManagePenLibrary() {
    console.log('Manage pen library clicked');
  }

  function addLayer() {
    const newId = `layer${layers.length + 1}`;
    layers.push({ id: newId, penId: pens[0].id, visible: true });
  }

  function toggleVisibility(layerId: string) {
    const layer = layers.find((l) => l.id === layerId);
    if (layer) {
      layer.visible = !layer.visible;
    }
  }

  function getPenById(penId: string): Pen | undefined {
    return pens.find((p) => p.id === penId);
  }
</script>

<div class="layers">
  <button type="button" class="action-button" onclick={handleManagePenLibrary}>
    Manage Pen Library
  </button>

  <div class="layer-list">
    {#each layers as layer, index (layer.id)}
      {@const pen = getPenById(layer.penId)}
      <div class="layer-card">
        <div class="layer-header">
          <span class="layer-title">Layer {index + 1}</span>
          <button
            type="button"
            class="visibility-btn"
            onclick={() => toggleVisibility(layer.id)}
            title={layer.visible ? 'Hide layer' : 'Show layer'}
          >
            {layer.visible ? '\u{1F441}' : '\u{1F441}\u{FE0E}'}
          </button>
          <span class="drag-handle">≡</span>
        </div>

        <div class="layer-controls">
          <div class="pen-select-row">
            <span class="color-swatch" style="background-color: {pen?.color ?? '#888'}"></span>
            <select class="pen-select" bind:value={layer.penId}>
              {#each pens as p (p.id)}
                <option value={p.id}>{p.name}</option>
              {/each}
            </select>
          </div>
          <span class="path-count">847 paths</span>
        </div>
      </div>
    {/each}
  </div>

  <button type="button" class="action-button" onclick={addLayer}>+ Add Layer</button>

  <div class="divider">
    <span class="divider-label">Path Assignment</span>
  </div>

  <div class="path-tree">
    <div class="tree-item">
      <span class="tree-toggle">▼</span>
      <span class="tree-label">group1 (847)</span>
      <select class="tree-select">
        {#each layers as layer, index (layer.id)}
          <option value="layer{index + 1}">Layer {index + 1}</option>
        {/each}
      </select>
    </div>
    <div class="tree-item indent-1">
      <span class="tree-toggle">▶</span>
      <span class="tree-label">shapes (420)</span>
      <select class="tree-select">
        {#each layers as layer, index (layer.id)}
          <option value="layer{index + 1}">Layer {index + 1}</option>
        {/each}
      </select>
    </div>
    <div class="tree-item indent-2">
      <span class="tree-toggle">•</span>
      <span class="tree-label">path1</span>
      <select class="tree-select">
        {#each layers as layer, index (layer.id)}
          <option value="layer{index + 1}">Layer {index + 1}</option>
        {/each}
      </select>
    </div>
    <div class="tree-item indent-2">
      <span class="tree-toggle">•</span>
      <span class="tree-label">path2</span>
      <select class="tree-select">
        {#each layers as layer, index (layer.id)}
          <option value="layer{index + 1}">Layer {index + 1}</option>
        {/each}
      </select>
    </div>
  </div>
</div>

<style>
  .layers {
    display: flex;
    flex-direction: column;
    gap: 8px;
    font-family: var(--main-font);
  }

  .action-button {
    width: 100%;
    padding: 10px 12px;
    font-family: var(--main-font);
    font-size: 12px;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    color: var(--text-primary);
    background: var(--bg-surface);
    border: 1px solid var(--border-subtle);
    border-radius: 2px;
    cursor: pointer;
  }

  .action-button:hover {
    border-color: var(--border-emphasis);
  }

  .layer-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .layer-card {
    background: var(--bg-surface);
    border: 1px solid var(--border-subtle);
    border-radius: 2px;
    padding: 12px;
  }

  .layer-header {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 8px;
  }

  .layer-title {
    flex: 1;
    font-size: 12px;
    font-weight: 500;
    color: var(--text-primary);
  }

  .visibility-btn {
    width: 24px;
    height: 24px;
    border: none;
    background: transparent;
    cursor: pointer;
    font-size: 14px;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0;
    opacity: 0.7;
  }

  .visibility-btn:hover {
    opacity: 1;
  }

  .drag-handle {
    font-size: 14px;
    color: var(--text-tertiary);
    cursor: grab;
    user-select: none;
  }

  .layer-controls {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
  }

  .pen-select-row {
    display: flex;
    align-items: center;
    gap: 8px;
    flex: 1;
  }

  .color-swatch {
    width: 12px;
    height: 12px;
    border-radius: 2px;
    flex-shrink: 0;
  }

  .pen-select {
    flex: 1;
    height: 28px;
    padding: 0 8px;
    border: 1px solid var(--border-subtle);
    background: var(--bg-surface);
    color: var(--text-primary);
    font-family: var(--main-font);
    font-size: 12px;
    cursor: pointer;
  }

  .pen-select:hover {
    border-color: var(--border-emphasis);
  }

  .path-count {
    font-size: 11px;
    color: var(--text-tertiary);
    white-space: nowrap;
  }

  .divider {
    display: flex;
    align-items: center;
    margin: 8px 0;
  }

  .divider-label {
    font-size: 10px;
    letter-spacing: 1.2px;
    text-transform: uppercase;
    color: var(--text-tertiary);
  }

  .path-tree {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .tree-item {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 12px;
  }

  .tree-item.indent-1 {
    padding-left: 16px;
  }

  .tree-item.indent-2 {
    padding-left: 32px;
  }

  .tree-toggle {
    width: 12px;
    font-size: 10px;
    color: var(--text-tertiary);
    flex-shrink: 0;
  }

  .tree-label {
    flex: 1;
    color: var(--text-secondary);
  }

  .tree-select {
    height: 24px;
    padding: 0 6px;
    border: 1px solid var(--border-subtle);
    background: var(--bg-surface);
    color: var(--text-primary);
    font-family: var(--main-font);
    font-size: 11px;
    cursor: pointer;
  }

  .tree-select:hover {
    border-color: var(--border-emphasis);
  }
</style>
