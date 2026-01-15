use serde::{Deserialize, Serialize};

use crate::geometry::{approximate_path, BoundingBox, Polyline};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SimplifiedSvgNode {
    Path(Path),
    Group(Group),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Path {
    pub id: Option<String>,
    pub subpaths: Vec<Polyline>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Group {
    pub id: Option<String>,
    pub children: Vec<SimplifiedSvgNode>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DigestResult {
    /// The simplified SVG geometry, with all units converted to inches.
    pub geometry: Group,

    /// The bounding box of the geometry, in inches.
    pub bounding_box: BoundingBox,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DigestOptions {
    /// The scale factor used to convert SVG units to inches.
    pub dpi: f32,

    /// The maximum error between curved segments and their approximations,
    /// specified in inches.
    pub curve_tolerance: f64,
}

/// Parses and "digests" an SVG into a simplified form where all shapes have
/// been replaced by approximations composed only of straight-line segments.
pub fn digest_svg(text: &str, opt: &DigestOptions) -> Result<DigestResult, usvg::Error> {
    let usvg_opt = usvg::Options {
        dpi: opt.dpi,
        ..Default::default()
    };
    let tree = usvg::Tree::from_data(text.as_bytes(), &usvg_opt)?;

    let bbox = tree.root().abs_bounding_box();
    let bounding_box = BoundingBox {
        left: bbox.left() as f64,
        top: bbox.top() as f64,
        right: bbox.right() as f64,
        bottom: bbox.bottom() as f64,
    };

    let geometry = convert_group(tree.root(), opt.curve_tolerance);

    Ok(DigestResult {
        geometry,
        bounding_box,
    })
}

fn convert_group(group: &usvg::Group, curve_tolerance: f64) -> Group {
    let mut children = Vec::new();

    for child in group.children() {
        match child {
            usvg::Node::Group(g) => {
                children.push(SimplifiedSvgNode::Group(convert_group(g, curve_tolerance)));
            }
            usvg::Node::Path(p) => {
                children.push(SimplifiedSvgNode::Path(convert_path(p, curve_tolerance)));
            }
            usvg::Node::Image(_) | usvg::Node::Text(_) => {
                // ignored
            }
        }
    }

    Group {
        id: Some(group.id())
            .filter(|&s| !s.is_empty())
            .map(str::to_string),
        children,
    }
}

fn convert_path(path: &usvg::Path, curve_tolerance: f64) -> Path {
    Path {
        id: Some(path.id())
            .filter(|&s| !s.is_empty())
            .map(str::to_string),
        subpaths: approximate_path(path.data(), curve_tolerance),
    }
}
