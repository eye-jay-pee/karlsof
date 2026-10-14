#[allow(dead_code)]
impl super::CLRecord {
    pub fn station(&self) -> Option<f64> {
        self.kind.encodes_station().then_some(self.distance)
    }
}
