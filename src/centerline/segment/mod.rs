pub mod anchor;
pub use anchor::{Anchor, Point};

mod display;

#[allow(dead_code)]
pub struct Segment {
    start: Anchor,
    end: Anchor,
    radius: Option<Point>,
}

// or maybe
#[allow(dead_code)]
pub struct SegmentAlt {
    start: Anchor,
    end: Anchor,
    radius: Option<Point>,
}
