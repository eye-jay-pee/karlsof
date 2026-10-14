pub mod point;
pub use point::Point;

pub mod station;
//pub use station::Station;

mod display;

#[allow(dead_code)]
pub struct Anchor {
    point: Point,
    distance: f64,
}

#[allow(dead_code)]
impl Anchor {
    pub fn new(point: Point, distance: f64) -> Self {
        Self { point, distance }
    }
}
