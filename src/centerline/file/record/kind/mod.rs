mod serde;

#[derive(Debug, Eq, PartialEq)]
pub enum CLRecordKind {
    LineEndPoint,
    PointOfCurve,
    RadiusPoint,
    PointOfTangent,
}

#[allow(dead_code)]
impl CLRecordKind {
    pub fn encodes_station(&self) -> bool {
        match self {
            Self::LineEndPoint => true,
            Self::PointOfCurve => true,
            Self::RadiusPoint => false,
            Self::PointOfTangent => true,
        }
    }
}
