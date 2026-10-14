use super::{CLFResult, CLRecord};
use csv::{ReaderBuilder, WriterBuilder};
use std::path::Path;

impl super::CLFile {
    pub fn load(path: impl AsRef<Path>) -> CLFResult<Self> {
        let mut clf = Self {
            records: Vec::default(),
            path: Self::verified_path_buf(path.as_ref())?,
            last_write: None,
        };

        // should yeild a DNE error if the file does not exist
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_path(&clf.path)?;

        clf.records = rdr
            .deserialize::<CLRecord>()
            .collect::<Result<Vec<_>, _>>()?;

        clf.set_timestamp()?;
        Ok(clf)
    }
    #[allow(dead_code)]
    pub fn save(&mut self) -> CLFResult<()> {
        self.check_timestamp()?;

        let mut wtr = WriterBuilder::new()
            .has_headers(false)
            .from_path(&self.path)?;

        self.records
            .iter()
            .try_for_each(|record| wtr.serialize(record))?;

        wtr.flush()?;
        self.set_timestamp()?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use crate::centerline::file::{CLFError, CLFile};

    #[test]
    fn can_load_file() {
        let clf = CLFile::load("data/bb_k7_south.cl").unwrap();
        assert!(clf.last_write.is_some());
    }

    #[test]
    fn can_save_file() {
        let temp_path = "data/can_save_check.cl";
        let _ignored_error = std::fs::remove_file(temp_path);
        let mut clf = CLFile::load("data/bb_k7_south.cl").unwrap();

        clf.set_path(temp_path).unwrap();
        assert!(clf.path.exists() == false);
        assert!(clf.last_write.is_none());

        clf.save().unwrap();
    }
}
