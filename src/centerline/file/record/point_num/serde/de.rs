use serde::{de::Error, Deserialize, Deserializer};
impl<'de> Deserialize<'de> for super::PointID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let trimmed = <&str>::deserialize(deserializer)?.trim();
        let parsed: u16 = trimmed
            .parse()
            .map_err(|e| Error::custom(format!("{}", e)))?;
        Ok(Self((parsed != 0).then_some(parsed)))
    }
}
