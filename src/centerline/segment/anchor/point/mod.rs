mod display;

#[allow(dead_code)]
pub struct Point {
    northing: f64,
    easting: f64,
}

#[allow(dead_code)]
impl Point {
    pub fn new(northing: f64, easting: f64) -> Self {
        Self { northing, easting }
    }
}
