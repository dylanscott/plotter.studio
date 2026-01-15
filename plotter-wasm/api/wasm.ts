import type { BoundingBox, Group } from "./geometry";

// `DigestResult` in `plotter-rs/src/simplify.rs`
export interface SimplifyResult {
  geometry: Group;
  boundingBox: BoundingBox;
}

// `DigestOptions` in `plotter-rs/src/simplify.rs`
export interface SimplifyOptions {
  dpi: number;
  curveTolerance: number;
}

export declare function simplify(
  svg: string,
  opts: SimplifyOptions,
): SimplifyResult;
