pub mod file;

pub mod segment;
pub use segment::Segment;

#[allow(dead_code)]
pub struct CenterLine {
    segments: Vec<Segment>,
}
