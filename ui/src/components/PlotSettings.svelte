<script lang="ts">
  // Speed & Quality
  let handlingMode = $state<
    "technical" | "handwriting" | "sketching" | "constant"
  >("technical");
  let acceleration = $state<"maximum" | "high" | "standard" | "slow">(
    "standard",
  );
  let penRaisingSpeed = $state<
    "maximum" | "standard" | "slow" | "very-slow" | "dead-slow"
  >("standard");
  let penLoweringSpeed = $state<
    "maximum" | "standard" | "slow" | "very-slow" | "dead-slow"
  >("standard");

  // Optimization
  let randomizeStart = $state(false);
  // @ts-ignore
  let optimization = $state<"none" | "least" | "basic" | "full">("full");
</script>

<div class="plot-settings">
  <section class="subsection">
    <h3 class="subsection-label">Speed & Quality</h3>

    <div class="form-row">
      <label class="label" for="handling-mode">Handling Mode</label>
      <select id="handling-mode" class="dropdown" bind:value={handlingMode}>
        <option value="technical">Technical Drawing</option>
        <option value="handwriting">Handwriting</option>
        <option value="sketching">Sketching</option>
        <option value="constant">Constant Speed</option>
      </select>
    </div>

    <div class="form-row">
      <label class="label" for="acceleration">Acceleration</label>
      <select id="acceleration" class="dropdown" bind:value={acceleration}>
        <option value="maximum">Maximum</option>
        <option value="high">High</option>
        <option value="standard">Standard</option>
        <option value="slow">Slow</option>
      </select>
    </div>

    <div class="form-row">
      <label class="label" for="pen-raising">Pen Raising Speed</label>
      <select id="pen-raising" class="dropdown" bind:value={penRaisingSpeed}>
        <option value="maximum">Maximum</option>
        <option value="standard">Standard</option>
        <option value="slow">Slow</option>
        <option value="very-slow">Very Slow</option>
        <option value="dead-slow">Dead Slow</option>
      </select>
    </div>

    <div class="form-row">
      <label class="label" for="pen-lowering">Pen Lowering Speed</label>
      <select id="pen-lowering" class="dropdown" bind:value={penLoweringSpeed}>
        <option value="maximum">Maximum</option>
        <option value="standard">Standard</option>
        <option value="slow">Slow</option>
        <option value="very-slow">Very Slow</option>
        <option value="dead-slow">Dead Slow</option>
      </select>
    </div>
  </section>

  <section class="subsection">
    <h3 class="subsection-label">Optimization</h3>

    <label class="checkbox-row">
      <input type="checkbox" class="checkbox" bind:checked={randomizeStart}>
      <span class="checkbox-label">Randomize Closed Path Start</span>
    </label>

    <fieldset class="radio-group">
      <legend class="radio-legend">Path Optimization</legend>

      <label class="radio-row">
        <input
          type="radio"
          name="optimization"
          value="none"
          class="radio"
          bind:group={optimization}
        >
        <span class="radio-content">
          <span class="radio-label">None</span>
          <span class="radio-description">Preserve path order</span>
        </span>
      </label>

      <label class="radio-row">
        <input
          type="radio"
          name="optimization"
          value="least"
          class="radio"
          bind:group={optimization}
        >
        <span class="radio-content">
          <span class="radio-label">Least</span>
          <span class="radio-description">Connect adjoining paths</span>
        </span>
      </label>

      <label class="radio-row">
        <input
          type="radio"
          name="optimization"
          value="basic"
          class="radio"
          bind:group={optimization}
        >
        <span class="radio-content">
          <span class="radio-label">Basic</span>
          <span class="radio-description">Allow path reordering</span>
        </span>
      </label>

      <label class="radio-row">
        <input
          type="radio"
          name="optimization"
          value="full"
          class="radio"
          bind:group={optimization}
        >
        <span class="radio-content">
          <span class="radio-label">Full</span>
          <span class="radio-description">Allow path reversal</span>
        </span>
      </label>
    </fieldset>
  </section>
</div>

<style>
  .plot-settings {
    display: flex;
    flex-direction: column;
    gap: 16px;
    font-family: var(--main-font);
  }

  .subsection {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .subsection-label {
    font-size: 10px;
    font-weight: 400;
    letter-spacing: 1.2px;
    text-transform: uppercase;
    color: var(--text-tertiary);
    margin: 0 0 4px 0;
  }

  .form-row {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .label {
    font-size: 12px;
    color: var(--text-secondary);
  }

  .dropdown {
    width: 100%;
    height: 32px;
    padding: 0 8px;
    border: 1px solid var(--border-subtle);
    border-radius: 2px;
    background: var(--bg-surface);
    color: var(--text-primary);
    font-family: var(--main-font);
    font-size: 12px;
    cursor: pointer;
    appearance: none;
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 12 12'%3E%3Cpath fill='%23666' d='M3 5l3 3 3-3'/%3E%3C/svg%3E");
    background-repeat: no-repeat;
    background-position: right 8px center;
  }

  .dropdown:hover {
    border-color: var(--border-emphasis);
  }

  .dropdown:focus {
    outline: none;
    border-color: var(--accent-primary);
  }

  .checkbox-row {
    display: flex;
    align-items: center;
    gap: 8px;
    cursor: pointer;
  }

  .checkbox {
    width: 16px;
    height: 16px;
    margin: 0;
    cursor: pointer;
    accent-color: var(--accent-primary);
  }

  .checkbox-label {
    font-size: 12px;
    color: var(--text-secondary);
  }

  .radio-group {
    display: flex;
    flex-direction: column;
    gap: 8px;
    border: none;
    margin: 8px 0 0 0;
    padding: 0;
  }

  .radio-legend {
    font-size: 12px;
    color: var(--text-secondary);
    margin-bottom: 8px;
    padding: 0;
  }

  .radio-row {
    display: flex;
    align-items: flex-start;
    gap: 8px;
    cursor: pointer;
  }

  .radio {
    width: 16px;
    height: 16px;
    margin: 0;
    flex-shrink: 0;
    cursor: pointer;
    accent-color: var(--accent-primary);
  }

  .radio-content {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .radio-label {
    font-size: 12px;
    color: var(--text-secondary);
  }

  .radio-description {
    font-size: 11px;
    color: var(--text-tertiary);
  }
</style>
