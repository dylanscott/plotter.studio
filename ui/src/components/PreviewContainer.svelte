<script lang="ts">
  import { type Length, simplify } from "plotter-wasm";
  import { Application, Container } from "svelte-pixi";

  import { type Layer, allSubpaths } from "../layer";
  import PlotPreview from "./PlotPreview.svelte";
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

  let result = simplify(
    `
<svg viewBox="0 0 11 8.5" xmlns="http://www.w3.org/2000/svg">
  <circle cx="50%" cy="50%" r="4" fill="transparent" stroke="black" stroke-width="0.1" />
</svg>`,
    {
      dpi: 96,
      curveTolerance: 0.002,
    },
  );
  let mm = 0.04 as Length<"in">;
  let layer: Layer = {
    paths: allSubpaths(result.geometry),
    color: "black",
    thickness: mm,
  };
</script>

<div class="pixi-container" bind:clientWidth={width} bind:clientHeight={height}>
  {#if width && height}
    <Application {width} {height} background={bgColor} antialias>
      <Container
        {scale}
        x={(width - scale * plotterWidth) / 2}
        y={(height - scale * plotterHeight) / 2}
      >
        <PlotPreview layers={[layer]} />
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
