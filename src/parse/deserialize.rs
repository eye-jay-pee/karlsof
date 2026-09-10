use super::CLRecordKind;
use serde::de::Error;
use serde::{Deserialize, Deserializer};

#[allow(dead_code)]
pub fn deserialize_point_number<'de, D>(
    deserializer: D,
) -> Result<Option<u16>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let n: u16 = s.parse().expect("invalid point number");
    Ok(if n == 0 { None } else { Some(n) })
}

impl<'de> Deserialize<'de> for CLRecordKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match <&str>::deserialize(deserializer)? {
            "L" => Ok(CLRecordKind::LineEndPoint),
            "PC" => Ok(CLRecordKind::PointOfCurve),
            "R" => Ok(CLRecordKind::RadiusPoint),
            "PT" => Ok(CLRecordKind::PointOfTangent),
            _ => Err(D::Error::custom("invalid record kind")),
        }
    }
}
