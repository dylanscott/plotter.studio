use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Point(pub f64, pub f64);

// A shape consisting of straight-line segments, which may be closed (in which
// case a line segment connects the first and last points).
#[derive(Serialize, Deserialize, Debug)]
pub struct Polyline {
    pub points: Vec<Point>,
    pub closed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BoundingBox {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}

// Approximates the Bézier path with a series of line segments. Each subpath
// (separated by `MoveTo` segments) is split into a separate Polyline.
// All the heavy lifting is done by `kurbo::flatten`.
pub fn approximate_path(path: &tiny_skia_path::Path, tolerance: f64) -> Vec<Polyline> {
    let mut subpaths: Vec<Polyline> = Vec::new();
    let mut current_points: Vec<Point> = Vec::new();
    let mut is_closed = false;

    kurbo::flatten(KurboPath(path), tolerance, |el| match el {
        kurbo::PathEl::MoveTo(pt) => {
            if !current_points.is_empty() {
                subpaths.push(Polyline {
                    points: std::mem::take(&mut current_points),
                    closed: is_closed,
                });
            }
            is_closed = false;
            current_points.push(Point(pt.x, pt.y));
        }
        kurbo::PathEl::LineTo(pt) => {
            current_points.push(Point(pt.x, pt.y));
        }
        kurbo::PathEl::ClosePath => {
            is_closed = true;
        }
        _ => {
            panic!("kurbo::flatten should only produce straight line segments!")
        }
    });

    // don't forget the last subpath
    if !current_points.is_empty() {
        subpaths.push(Polyline {
            points: current_points,
            closed: is_closed,
        });
    }

    subpaths
}

// Wrapper around `tiny_skia_path::Path` that implements `IntoIterator<Item = kurbo::PathEl>`.
struct KurboPath<'a>(&'a tiny_skia_path::Path);

impl<'a> IntoIterator for KurboPath<'a> {
    type Item = kurbo::PathEl;
    type IntoIter = KurboPathIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        KurboPathIter(self.0.segments())
    }
}

struct KurboPathIter<'a>(tiny_skia_path::PathSegmentsIter<'a>);

impl Iterator for KurboPathIter<'_> {
    type Item = kurbo::PathEl;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|seg| match seg {
            tiny_skia_path::PathSegment::MoveTo(pt) => {
                kurbo::PathEl::MoveTo(kurbo::Point::new(pt.x as f64, pt.y as f64))
            }
            tiny_skia_path::PathSegment::LineTo(pt) => {
                kurbo::PathEl::LineTo(kurbo::Point::new(pt.x as f64, pt.y as f64))
            }
            tiny_skia_path::PathSegment::QuadTo(p1, p2) => kurbo::PathEl::QuadTo(
                kurbo::Point::new(p1.x as f64, p1.y as f64),
                kurbo::Point::new(p2.x as f64, p2.y as f64),
            ),
            tiny_skia_path::PathSegment::CubicTo(p1, p2, p3) => kurbo::PathEl::CurveTo(
                kurbo::Point::new(p1.x as f64, p1.y as f64),
                kurbo::Point::new(p2.x as f64, p2.y as f64),
                kurbo::Point::new(p3.x as f64, p3.y as f64),
            ),
            tiny_skia_path::PathSegment::Close => kurbo::PathEl::ClosePath,
        })
    }
}
