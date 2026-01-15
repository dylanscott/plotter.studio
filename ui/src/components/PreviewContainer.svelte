<script lang="ts">
  import type { Length } from "plotter-wasm";
  import { Application, Container } from "svelte-pixi";

  import PlotterFrame from "./PlotterFrame.svelte";

  interface Props {
    plotterWidth: Length<"in">;
    plotterHeight: Length<"in">;
  }

  let { plotterWidth, plotterHeight }: Props = $props();

  let width = $state(0);
  let height = $state(0);

  // compute scale so that children can specify positions in inches, with the
  // plotter's dimensions bounded to no more than 80% of the viewport's
  let scale = $derived.by(() => {
    const xScale = (0.8 * width) / plotterWidth;
    const yScale = (0.8 * height) / plotterHeight;
    return Math.min(xScale, yScale);
  });

  let bodyStyles = window.getComputedStyle(document.body);
  let bgColor = bodyStyles.getPropertyValue("--bg-primary");
  let gridColor = bodyStyles.getPropertyValue("--text-primary");
</script>

<div class="pixi-container" bind:clientWidth={width} bind:clientHeight={height}>
  {#if width && height}
    <Application {width} {height} background={bgColor} antialias>
      <Container
        {scale}
        x={(width - scale * plotterWidth) / 2}
        y={(height - scale * plotterHeight) / 2}
      >
        <PlotterFrame
          width={plotterWidth}
          height={plotterHeight}
          color={gridColor}
        />
      </Container>
    </Application>
  {/if}
</div>

<style>
  .pixi-container {
    width: 100%;
    height: 100%;
  }
</style>
