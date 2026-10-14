use serde::{ser::Error, Serialize, Serializer};
impl Serialize for super::PointID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.0 {
            Some(0) => {
                Err(S::Error::custom("point numbers must be greater than 0"))
            }
            Some(v) => serializer.serialize_u16(v),
            None => serializer.serialize_str("0"),
        }
    }
}
