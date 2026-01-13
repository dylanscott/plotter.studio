<script lang="ts">
  let xOffset = $state(0);
  let yOffset = $state(0);
  let scale = $state(1.0);

  function adjustValue(
    current: number,
    delta: number,
    min: number = -Infinity,
    max: number = Infinity
  ): number {
    const newValue = Math.round((current + delta) * 10) / 10;
    return Math.max(min, Math.min(max, newValue));
  }

  function reset() {
    xOffset = 0;
    yOffset = 0;
    scale = 1.0;
  }

  function center() {
    console.log('CENTER: needs actual SVG dimensions to calculate center position');
  }
</script>

<div class="position-scale">
  <div class="row">
    <label class="label" for="x-offset">X Offset</label>
    <div class="input-group">
      <button
        class="adjust-btn"
        type="button"
        onclick={() => (xOffset = adjustValue(xOffset, -0.1))}
      >
        -
      </button>
      <input
        id="x-offset"
        type="number"
        class="number-input"
        bind:value={xOffset}
        step="0.1"
      />
      <span class="unit">in</span>
      <button
        class="adjust-btn"
        type="button"
        onclick={() => (xOffset = adjustValue(xOffset, 0.1))}
      >
        +
      </button>
    </div>
  </div>

  <div class="row">
    <label class="label" for="y-offset">Y Offset</label>
    <div class="input-group">
      <button
        class="adjust-btn"
        type="button"
        onclick={() => (yOffset = adjustValue(yOffset, -0.1))}
      >
        -
      </button>
      <input
        id="y-offset"
        type="number"
        class="number-input"
        bind:value={yOffset}
        step="0.1"
      />
      <span class="unit">in</span>
      <button
        class="adjust-btn"
        type="button"
        onclick={() => (yOffset = adjustValue(yOffset, 0.1))}
      >
        +
      </button>
    </div>
  </div>

  <div class="row">
    <label class="label" for="scale">Scale</label>
    <div class="input-group">
      <button
        class="adjust-btn"
        type="button"
        onclick={() => (scale = adjustValue(scale, -0.1, 0.1))}
      >
        -
      </button>
      <input
        id="scale"
        type="number"
        class="number-input"
        bind:value={scale}
        step="0.1"
        min="0.1"
      />
      <button
        class="adjust-btn"
        type="button"
        onclick={() => (scale = adjustValue(scale, 0.1))}
      >
        +
      </button>
    </div>
  </div>

  <div class="actions">
    <button class="action-btn" type="button" onclick={center}>Center</button>
    <button class="action-btn" type="button" onclick={reset}>Reset</button>
  </div>
</div>

<style>
  .position-scale {
    display: flex;
    flex-direction: column;
    gap: 8px;
    font-family: var(--main-font);
  }

  .row {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .label {
    font-size: 12px;
    color: var(--text-secondary);
    text-transform: uppercase;
  }

  .input-group {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .adjust-btn {
    width: 24px;
    height: 24px;
    border: 1px solid var(--border-subtle);
    background: transparent;
    color: var(--text-primary);
    cursor: pointer;
    font-family: var(--main-font);

    font-size: 14px;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0;
  }

  .adjust-btn:hover {
    background: var(--bg-secondary);
  }

  .number-input {
    width: 60px;
    height: 24px;
    padding: 0 4px;
    border: 1px solid var(--border-subtle);
    background: var(--bg-surface);
    color: var(--text-primary);
    font-family: var(--main-font);
    font-size: 12px;
    text-align: right;
  }

  .number-input::-webkit-inner-spin-button,
  .number-input::-webkit-outer-spin-button {
    -webkit-appearance: none;
    margin: 0;
  }

  .number-input[type='number'] {
    appearance: textfield;
    -moz-appearance: textfield;
  }

  .unit {
    font-size: 12px;
    color: var(--text-tertiary);
    min-width: 16px;
  }

  .actions {
    display: flex;
    gap: 8px;
    margin-top: 8px;
  }

  .action-btn {
    flex: 1;
    padding: 8px 12px;
    border: 1px solid var(--border-subtle);
    background: transparent;
    color: var(--text-primary);
    font-family: var(--main-font);
    font-size: 11px;
    letter-spacing: 1px;
    text-transform: uppercase;
    cursor: pointer;
  }

  .action-btn:hover {
    background: var(--bg-secondary);
  }
</style>
