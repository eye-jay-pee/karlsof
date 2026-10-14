use super::CLRecord;

impl IntoIterator for super::CLFile {
    type Item = CLRecord;
    type IntoIter = std::vec::IntoIter<CLRecord>;

    fn into_iter(self) -> Self::IntoIter {
        self.records.into_iter()
    }
}
