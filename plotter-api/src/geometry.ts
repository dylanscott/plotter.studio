import type { Length } from "./units";

/**
 * 2D point in inches
 */
export type Point = [x: Length<"in">, y: Length<"in">];

/**
 * A simplified version of an SVG, which inherits group structure but where all
   <path> elements have been replaced by polyline approximations.
 */
export type SimplifiedSVG = PolyLine | Group;

export interface PolyLine {
  type: "polyline";
  id?: string;
  points: Point[];
  closed: boolean;
}

export interface Group {
  type: "group";
  id?: string;
  children: Node[];
}
