import type { Length } from "./units";

/**
 * 2D point in inches (standard unit used for geometry)
 */
export type Point = [x: Length<"in">, y: Length<"in">];

// ============================================================================
// SVG Data Model (Simplified Polylines)
// ============================================================================

/**
 * After WASM digestion, all curves are converted to polyline approximations.
 * The tree structure is simplified to groups and polylines only.
 */
export type Node = PolyLine | Group;

export interface PolyLine {
  type: "polyline";
  id?: string; // from original SVG, if specified
  points: Point[];
  closed: boolean;
}

export interface Group {
  type: "group";
  id?: string; // from original SVG, if specified
  children: Node[];
}
