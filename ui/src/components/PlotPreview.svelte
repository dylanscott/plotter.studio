<script lang="ts">
  import * as PIXI from "pixi.js";
  import { Graphics } from "svelte-pixi";

  import type { Layer } from "../layer";

  interface Props {
    layers: Layer[];
  }

  let { layers }: Props = $props();

  function draw(graphics: PIXI.Graphics) {
    graphics.clear();

    for (const layer of layers) {
      for (const path of layer.paths) {
        const [startX, startY] = path[0];
        graphics.moveTo(startX, startY);
        for (const [x, y] of path.slice(1)) {
          graphics.lineTo(x, y);
        }
        graphics.stroke({ color: layer.color, width: layer.thickness });
      }
    }
  }
</script>

<Graphics {draw} />
