pub struct Point(pub f64, pub f64);

pub struct Polyline {
    pub points: Vec<Point>,
    pub closed: bool,
    pub id: Option<String>,
}

pub struct Group {
    pub children: Vec<Node>,
    pub id: Option<String>,
}

pub enum Node {
    Path(Polyline),
    Group(Group),
}

pub struct BoundingBox {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}
