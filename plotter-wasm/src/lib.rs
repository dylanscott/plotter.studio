use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsError, JsValue};

use plotter::simplify::{digest_svg, DigestOptions};

#[wasm_bindgen]
pub fn simplify(svg: &str, opts: JsValue) -> Result<JsValue, JsError> {
    let digest_opts: DigestOptions = from_value(opts)?;
    let digest_result =
        digest_svg(svg, &digest_opts).map_err(|err| JsError::new(&err.to_string()))?;
    Ok(to_value(&digest_result)?)
}
