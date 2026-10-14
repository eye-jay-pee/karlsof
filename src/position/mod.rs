#[allow(dead_code)]
pub trait Position {
    fn northing(&self) -> Option<f64>;
    fn easting(&self) -> Option<f64>;
    fn elevation(&self) -> Option<f64>;
}
