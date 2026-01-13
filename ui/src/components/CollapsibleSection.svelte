<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    title: string;
    expanded: boolean;
    onToggle: () => void;
    children: Snippet;
  }

  let { title, expanded, onToggle, children }: Props = $props();
</script>

<div class="collapsible-section">
  <button class="header" onclick={onToggle} type="button">
    <span class="indicator">{expanded ? '▼' : '▶'}</span>
    <span class="title">{title}</span>
  </button>

  {#if expanded}
    <div class="content">{@render children()}</div>
  {/if}
</div>

<style>
  .collapsible-section {
    border: 1px solid var(--border-subtle);
    border-radius: 2px;
  }

  .header {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    padding: 12px;
    border: none;
    background: transparent;
    cursor: pointer;
    text-align: left;
  }

  .header:hover {
    background: var(--bg-secondary);
  }

  .indicator {
    font-size: 10px;
    color: var(--text-secondary);
    width: 12px;
  }

  .title {
    font-size: 11px;
    letter-spacing: 1.2px;
    text-transform: uppercase;
    color: var(--text-secondary);
  }

  .content {
    padding: 16px;
    background: var(--bg-surface);
    border-top: 1px solid var(--border-subtle);
  }
</style>
