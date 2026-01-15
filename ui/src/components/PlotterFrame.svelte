<script lang="ts">
  import * as PIXI from "pixi.js";
  import type { Length } from "plotter-wasm";
  import { Graphics } from "svelte-pixi";

  interface Props {
    width: Length<"in">;
    height: Length<"in">;
    color: string;
  }

  let { width, height, color }: Props = $props();

  function draw(graphics: PIXI.Graphics) {
    graphics.clear();
    graphics.rect(0, 0, width, height).stroke({ color, pixelLine: true });
  }
</script>

<Graphics {draw} />
