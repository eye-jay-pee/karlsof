use super::CLRecordKind;
use serde::{ser::Error, Serialize, Serializer};

#[allow(dead_code)]
pub fn serialize_point_number<S>(
    value: &Option<u16>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(0) => {
            Err(S::Error::custom("Point numbers must be greater than 0"))
        }
        Some(v) => serializer.serialize_u16(*v),
        None => serializer.serialize_str("0"),
    }
}

impl Serialize for CLRecordKind {
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
