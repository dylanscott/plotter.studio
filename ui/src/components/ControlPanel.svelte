<script lang="ts">
  import PlotterConnection from "./PlotterConnection.svelte";
  import PlotStatus from "./PlotStatus.svelte";
  import CollapsibleSection from "./CollapsibleSection.svelte";
  import PositionScale from "./PositionScale.svelte";
  import Layers from "./Layers.svelte";
  import PlotSettings from "./PlotSettings.svelte";

  type SectionId = "position-scale" | "layers" | "plot-settings";

  let expandedSection: SectionId | null = $state("position-scale");

  function toggleSection(sectionId: SectionId) {
    expandedSection = expandedSection === sectionId ? null : sectionId;
  }
</script>

<div class="control-panel">
  <PlotterConnection />
  <PlotStatus />

  <div class="sections">
    <CollapsibleSection
      title="Position & Scale"
      expanded={expandedSection === 'position-scale'}
      onToggle={() => toggleSection('position-scale')}
    >
      <PositionScale />
    </CollapsibleSection>

    <CollapsibleSection
      title="Layers"
      expanded={expandedSection === 'layers'}
      onToggle={() => toggleSection('layers')}
    >
      <Layers />
    </CollapsibleSection>

    <CollapsibleSection
      title="Plot Settings"
      expanded={expandedSection === 'plot-settings'}
      onToggle={() => toggleSection('plot-settings')}
    >
      <PlotSettings />
    </CollapsibleSection>
  </div>
</div>

<style>
  .control-panel {
    height: 100%;
    overflow-y: auto;
    padding: 16px;
    font-family: var(--main-font);
    color: var(--text-primary);
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .sections {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }
</style>
