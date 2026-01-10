import { Group } from "./geometry";
import { Length } from "./units";

// ============================================================================
// WASM Module Interface
// ============================================================================

export interface PlotTransform {
  /** X-axis offset in inches */
  offsetX: Length<"in">;
  
  /** Y-axis offset in inches */
  offsetY: Length<"in">;
  
  /** Scale multiplier (1.0 = 100%, affects curve subdivision) */
  scale: number;
}

export type HandlingMode = 'technical' | 'handwriting' | 'sketching' | 'constant';

/**
 * Options for SVG digestion.
 * These settings affect how curves are simplified and how the SVG is clipped.
 */
export interface DigestOptions extends PlotTransform {
  /** Handling mode affects curve simplification tolerance */
  handlingMode: HandlingMode;
  
  /** Plottable area dimensions (for clipping) */
  width: Length<"in">;
  height: Length<"in">;
};

/**
 * Result of SVG digestion
 */
export interface DigestResult {
  /** Simplified tree of groups and polylines */
  root: Group;
  
  /** Bounding box of SVG geometry (used for centering) */
  boundingBox: {
    left: Length<"in">;
    top: Length<"in">;
    right: Length<"in">;
    bottom: Length<"in">;
  };
  
  /** Total number of polylines in the result */
  polylineCount: number;
  
  /** Total number of points across all polylines */
  pointCount: number;
}

/**
 * Interface for the Rust WASM module.
 * Currently only implements SVG parsing and simplification.
 * Motion planning API will be added in future work.
 */
export interface WASMPlotModule {
  /**
   * Parse SVG and simplify to polyline representation.
   * 
   * This function:
   * - Parses the SVG structure
   * - Converts all curves (Bezier, arcs, etc.) to polyline approximations
   * - Applies the transform (offset, scale)
   * - Clips to the plottable area
   * - Adjusts subdivision based on handlingMode and scale
   * 
   * @param svgContent Raw SVG file content as string
   * @param opts Digest options (transform, handling mode, plottable area)
   * @returns Simplified tree of groups and polylines
   */
  digestSVG(svgContent: string, opts: DigestOptions): DigestResult;
}
