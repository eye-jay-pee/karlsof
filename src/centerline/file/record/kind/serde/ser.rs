use serde::{Serialize, Serializer};
impl Serialize for super::CLRecordKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::LineEndPoint => serializer.serialize_str("L"),
            Self::PointOfCurve => serializer.serialize_str("PC"),
            Self::RadiusPoint => serializer.serialize_str("R"),
            Self::PointOfTangent => serializer.serialize_str("PT"),
        }
    }
}
