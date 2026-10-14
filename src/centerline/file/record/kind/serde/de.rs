use serde::{de::Error, Deserialize, Deserializer};
impl<'de> Deserialize<'de> for super::CLRecordKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match <&str>::deserialize(deserializer)?.trim() {
            "L" => Ok(Self::LineEndPoint),
            "PC" => Ok(Self::PointOfCurve),
            "R" => Ok(Self::RadiusPoint),
            "PT" => Ok(Self::PointOfTangent),
            _ => Err(D::Error::custom("invalid record kind")),
        }
    }
}
