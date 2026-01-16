import type { Length, Polyline, SimplifiedSvgNode } from "plotter-wasm";

export interface Layer {
  color: string;
  thickness: Length<"in">;
  paths: Polyline[];
}

export function allSubpaths(node: SimplifiedSvgNode): Polyline[] {
  switch (node.type) {
    case "path":
      return node.subpaths;
    case "group":
      return node.children.flatMap(allSubpaths);
  }
}
