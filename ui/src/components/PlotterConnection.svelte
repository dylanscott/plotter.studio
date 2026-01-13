<script lang="ts">
  type ConnectionState =
    | 'disconnected'
    | 'idle'
    | 'manual-positioning'
    | 'homing'
    | 'plotting'
    | 'paused'
    | 'error';

  let connectionState: ConnectionState = $state('disconnected');

  function handleConnect() {
    connectionState = 'idle';
  }

  function handleDisconnect() {
    connectionState = 'disconnected';
  }

  function handleManualPositioning() {
    console.log('Manual positioning clicked');
  }

  function handleAutoHome() {
    console.log('Auto home clicked');
  }
</script>

<section class="plotter-connection">
  <h2 class="section-header">Plotter</h2>

  {#if connectionState === 'disconnected'}
    <p class="status-line">Not Connected</p>
    <button type="button" class="action-button" onclick={handleConnect}>
      CONNECT PLOTTER
    </button>
  {:else}
    <div class="info-row">
      <span class="info-label">Model:</span>
      <span class="info-value">NextDraw 8511</span>
    </div>
    <div class="info-row">
      <span class="info-label">Size:</span>
      <span class="info-value">8.5 × 11 in</span>
    </div>

    <h3 class="subsection-label">Alignment</h3>

    <div class="button-group">
      <button type="button" class="action-button" onclick={handleManualPositioning}>
        Manual Positioning
      </button>
      <button type="button" class="action-button" onclick={handleAutoHome}>
        Auto Home
      </button>
      <button type="button" class="action-button" onclick={handleDisconnect}>
        Disconnect
      </button>
    </div>
  {/if}
</section>

<style>
  .plotter-connection {
    padding: 12px;
    font-family: var(--main-font);
  }

  .section-header {
    font-size: 11px;
    font-weight: 400;
    letter-spacing: 1.2px;
    text-transform: uppercase;
    color: var(--text-secondary);
    margin: 0 0 8px 0;
  }

  .status-line {
    font-size: 12px;
    color: var(--text-tertiary);
    margin: 0 0 12px 0;
  }

  .info-row {
    display: flex;
    gap: 8px;
    font-size: 12px;
    margin-bottom: 4px;
  }

  .info-label {
    color: var(--text-tertiary);
  }

  .info-value {
    color: var(--text-primary);
  }

  .subsection-label {
    font-size: 10px;
    font-weight: 400;
    letter-spacing: 1.2px;
    text-transform: uppercase;
    color: var(--text-tertiary);
    margin: 16px 0 8px 0;
  }

  .button-group {
    display: flex;
    flex-direction: column;
    gap: 8px;
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
</style>
