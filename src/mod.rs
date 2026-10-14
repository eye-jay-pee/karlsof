#[allow(dead_code)]
pub struct Point {
    number: Option<u16>,
    x: f64,
    y: f64,
}
#[allow(dead_code)]
pub struct Anchor {
    point: Point,
    distance: f64,
}
#[allow(dead_code)]
pub struct Segment {
    start: Anchor,
    end: Anchor,
    radius: Option<Anchor>,
}
#[allow(dead_code)]
pub struct CenterLine {
    segments: Vec<Segment>,
}
