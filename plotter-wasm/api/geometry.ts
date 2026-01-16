import type { Length } from "./units";

// see `plotter-rs/src/geometry.rs`
export type Point = [x: Length<"in">, y: Length<"in">];

export type Polyline = Point[];

export interface BoundingBox {
  left: Length<"in">;
  top: Length<"in">;
  right: Length<"in">;
  bottom: Length<"in">;
}

// see `plotter-rs/src/simplify.rs`
export type SimplifiedSvgNode = Path | Group;

export interface Path {
  type: "path";
  id?: string;
  subpaths: Polyline[];
}

export interface Group {
  type: "group";
  id?: string;
  children: SimplifiedSvgNode[];
}
