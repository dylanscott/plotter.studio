<script lang="ts">
  let hasFile = $state(false);
  let isPlotting = $state(false);
  let progress = $state(0);
  let dryRun = $state(true);

  let progressInterval: number | undefined;

  function handleUpload() {
    hasFile = true;
  }

  function handlePlot() {
    if (isPlotting) return;

    isPlotting = true;
    progress = 0;

    progressInterval = window.setInterval(() => {
      progress += 2;
      if (progress >= 100) {
        progress = 100;
        isPlotting = false;
        if (progressInterval) {
          clearInterval(progressInterval);
          progressInterval = undefined;
        }
      }
    }, 100);
  }

  function handlePause() {
    if (progressInterval) {
      clearInterval(progressInterval);
      progressInterval = undefined;
    }
  }

  function handleStop() {
    if (progressInterval) {
      clearInterval(progressInterval);
      progressInterval = undefined;
    }
    isPlotting = false;
    progress = 0;
  }

  function formatTimeRemaining(currentProgress: number): string {
    // Mock: assume 23m 45s total (1425 seconds)
    const totalSeconds = 1425;
    const remainingSeconds = Math.round(
      totalSeconds * (1 - currentProgress / 100),
    );
    const minutes = Math.floor(remainingSeconds / 60);
    const seconds = remainingSeconds % 60;
    return `${minutes}m ${seconds.toString().padStart(2, "0")}s`;
  }
</script>

<div class="plot-status">
  <h3 class="section-header">Plot Status</h3>

  {#if !hasFile}
    <p class="no-file">No file loaded</p>
    <button class="upload-button" onclick={handleUpload} type="button">
      UPLOAD SVG
    </button>
  {:else}
    <div class="file-info">
      <div class="info-row">
        <span class="label">File:</span>
        <span class="value">example.svg</span>
      </div>
      <div class="info-row">
        <span class="label">Dimensions:</span>
        <span class="value">8.3 x 10.7 in</span>
      </div>
      <div class="info-row">
        <span class="label">Paths:</span>
        <span class="value">1,247</span>
      </div>
      <div class="info-row">
        <span class="label">Est. Time:</span>
        <span class="value">23m 45s</span>
      </div>
    </div>

    <label class="checkbox-row">
      <input type="checkbox" bind:checked={dryRun}>
      <span>Dry Run</span>
    </label>

    {#if isPlotting}
      <div class="progress-container">
        <div class="progress-bar">
          <div class="progress-fill" style="width: {progress}%"></div>
        </div>
        <div class="progress-info">
          <span>{progress}%</span>
          <span>{formatTimeRemaining(progress)} remaining</span>
        </div>
      </div>
    {/if}

    <div class="button-row">
      <button
        class="button primary"
        onclick={handlePlot}
        disabled={isPlotting}
        type="button"
      >
        PLOT
      </button>
      {#if isPlotting}
        <button class="button secondary" onclick={handlePause} type="button">
          PAUSE
        </button>
        <button class="button secondary" onclick={handleStop} type="button">
          STOP
        </button>
      {/if}
    </div>
  {/if}
</div>

<style>
  .plot-status {
    padding: 12px;
    font-family: var(--main-font);
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .section-header {
    font-size: 11px;
    letter-spacing: 1.2px;
    text-transform: uppercase;
    color: var(--text-secondary);
    margin: 0;
    font-weight: normal;
  }

  .no-file {
    font-size: 12px;
    color: var(--text-tertiary);
    margin: 0;
  }

  .upload-button {
    padding: 8px 16px;
    font-family: var(--main-font);
    font-size: 11px;
    letter-spacing: 0.5px;
    background: var(--accent-primary);
    color: var(--bg-surface);
    border: none;
    border-radius: 2px;
    cursor: pointer;
    align-self: flex-start;
  }

  .upload-button:hover {
    opacity: 0.9;
  }

  .file-info {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .info-row {
    display: flex;
    gap: 8px;
    font-size: 12px;
  }

  .label {
    color: var(--text-secondary);
  }

  .value {
    color: var(--text-primary);
  }

  .checkbox-row {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 12px;
    color: var(--text-primary);
    cursor: pointer;
  }

  .checkbox-row input[type="checkbox"] {
    cursor: pointer;
  }

  .progress-container {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .progress-bar {
    height: 8px;
    background: var(--bg-surface);
    border-radius: 2px;
    overflow: hidden;
    border: 1px solid var(--border-subtle);
  }

  .progress-fill {
    height: 100%;
    background: var(--accent-primary);
    transition: width 100ms linear;
  }

  .progress-info {
    display: flex;
    justify-content: space-between;
    font-size: 11px;
    color: var(--text-secondary);
  }

  .button-row {
    display: flex;
    gap: 8px;
  }

  .button {
    padding: 8px 16px;
    font-family: var(--main-font);
    font-size: 11px;
    letter-spacing: 0.5px;
    border: none;
    border-radius: 2px;
    cursor: pointer;
  }

  .button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .button.primary {
    background: var(--accent-primary);
    color: var(--bg-surface);
  }

  .button.primary:hover:not(:disabled) {
    opacity: 0.9;
  }

  .button.secondary {
    background: var(--bg-secondary);
    color: var(--text-primary);
    border: 1px solid var(--border-subtle);
  }

  .button.secondary:hover {
    background: var(--bg-primary);
  }
</style>
