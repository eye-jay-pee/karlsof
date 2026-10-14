impl<I: Into<u16>> From<Option<I>> for super::PointID {
    fn from(value: Option<I>) -> Self {
        Self(value.map(Into::into))
    }
}

