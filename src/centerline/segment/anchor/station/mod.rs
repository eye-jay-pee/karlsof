mod display;
mod tryfrom;

#[allow(dead_code)]
pub struct Station {
    value: f64,
}

#[allow(dead_code)]
impl Station {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
    pub fn number(&self) -> i32 {
        (self.value as i32) / 100
    }
    pub fn plus(&self) -> f64 {
        self.value % 100.0
    }
}
