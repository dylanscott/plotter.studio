use kurbo::PathEl;
use tiny_skia_path::{PathSegment, PathSegmentsIter};

use crate::geometry::{BoundingBox, Group, Node, Point, Polyline};

pub struct DigestResult {
    pub geometry: Group,
    pub bounding_box: BoundingBox,
}

pub struct DigestOptions {
    pub dpi: f32,
    pub curve_tolerance: f64,
}

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

fn convert_group(group: &usvg::Group, tolerance: f64) -> Group {
    let id = if group.id().is_empty() {
        None
    } else {
        Some(group.id().to_string())
    };

    let mut children = Vec::new();

    for child in group.children() {
        match child {
            usvg::Node::Group(g) => {
                children.push(Node::Group(convert_group(g, tolerance)));
            }
            usvg::Node::Path(p) => {
                let path_nodes = convert_path(p, tolerance);
                children.extend(path_nodes);
            }
            usvg::Node::Image(_) | usvg::Node::Text(_) => {
                // ignored
            }
        }
    }

    Group { children, id }
}

fn convert_path(path: &usvg::Path, tolerance: f64) -> Vec<Node> {
    let path_id = if path.id().is_empty() {
        None
    } else {
        Some(path.id().to_string())
    };

    let mut polylines: Vec<Polyline> = Vec::new();
    let mut current_points: Vec<Point> = Vec::new();
    let mut is_closed = false;

    kurbo::flatten(KurboPath(path.data()), tolerance, |el| match el {
        PathEl::MoveTo(pt) => {
            if !current_points.is_empty() {
                polylines.push(Polyline {
                    points: std::mem::take(&mut current_points),
                    closed: is_closed,
                    id: None,
                });
            }
            is_closed = false;
            current_points.push(Point(pt.x, pt.y));
        }
        PathEl::LineTo(pt) => {
            current_points.push(Point(pt.x, pt.y));
        }
        PathEl::ClosePath => {
            is_closed = true;
        }
        _ => {
            panic!("kurbo::flatten should only produce straight line segments!")
        }
    });

    // don't forget the last polyline
    if !current_points.is_empty() {
        polylines.push(Polyline {
            points: current_points,
            closed: is_closed,
            id: None,
        });
    }

    match polylines.len() {
        0 => Vec::new(),
        1 => {
            let mut polyline = polylines.pop().unwrap();
            polyline.id = path_id;
            vec![Node::Path(polyline)]
        }
        _ => {
            // multiple polylines - wrap in a group with the path's id
            let children = polylines.into_iter().map(Node::Path).collect();
            vec![Node::Group(Group {
                children,
                id: path_id,
            })]
        }
    }
}

/// Wrapper around `tiny_skia_path::Path` that implements `IntoIterator<Item = kurbo::PathEl>`.
struct KurboPath<'a>(&'a tiny_skia_path::Path);

impl<'a> IntoIterator for KurboPath<'a> {
    type Item = PathEl;
    type IntoIter = KurboPathIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        KurboPathIter(self.0.segments())
    }
}

struct KurboPathIter<'a>(PathSegmentsIter<'a>);

impl Iterator for KurboPathIter<'_> {
    type Item = PathEl;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|seg| match seg {
            PathSegment::MoveTo(pt) => PathEl::MoveTo(kurbo::Point::new(pt.x as f64, pt.y as f64)),
            PathSegment::LineTo(pt) => PathEl::LineTo(kurbo::Point::new(pt.x as f64, pt.y as f64)),
            PathSegment::QuadTo(p1, p2) => PathEl::QuadTo(
                kurbo::Point::new(p1.x as f64, p1.y as f64),
                kurbo::Point::new(p2.x as f64, p2.y as f64),
            ),
            PathSegment::CubicTo(p1, p2, p3) => PathEl::CurveTo(
                kurbo::Point::new(p1.x as f64, p1.y as f64),
                kurbo::Point::new(p2.x as f64, p2.y as f64),
                kurbo::Point::new(p3.x as f64, p3.y as f64),
            ),
            PathSegment::Close => PathEl::ClosePath,
        })
    }
}
