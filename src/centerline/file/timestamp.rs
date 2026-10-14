use super::{CLFError, CLFResult};
use crate::centerline::file::SystemTime;

#[allow(dead_code)]
impl super::CLFile {
    fn timestamp_actual(&self) -> Option<SystemTime> {
        self.path.metadata().ok()?.modified().ok()
    }
    fn timestamp_expected(&self) -> Option<SystemTime> {
        self.last_write
    }
    pub fn check_timestamp(&self) -> CLFResult<()> {
        match (self.timestamp_actual(), self.timestamp_expected()) {
            (None, None) => Ok(()),
            (None, Some(_)) => Err(CLFError::UnexpectedFileFound),
            (Some(_), None) => Err(CLFError::ExpectedFileNotFound),
            (Some(a), Some(e)) => {
                (a == e).then_some(()).ok_or(CLFError::TimestampMismatch)
            }
        }
    }
    pub fn set_timestamp(&mut self) -> CLFResult<()> {
        self.last_write = Some(self.path.metadata()?.modified()?);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use crate::centerline::file::{CLFError, CLFile};

    #[test]
    fn no_ts_on_files_that_dont_exist() {
        let a =
            CLFile::new("data/a_valid_file_that_does_not_exist.cl").unwrap();
        assert!(a.last_write.is_none());
    }

    #[test]
    fn some_ts_on_files_that_exist() {
        let a = CLFile::new("data/bb_k7_south.cl").unwrap();
        assert!(a.last_write.is_some());
    }

    #[test]
    fn no_expectations() {
        let a =
            CLFile::new("data/a_valid_file_that_does_not_exist.cl").unwrap();
        assert!(a.path.exists() == false);
        assert!(a.check_timestamp().is_ok());
    }

    #[test]
    fn expect_a_file() {
        let a = CLFile::new("data/bb_k7_south.cl").unwrap();
        assert!(a.path.exists());
        assert!(a.check_timestamp().is_ok());
    }

    #[test]
    fn can_set_new_path() {
        let temp_path = "data/new_path_temp.cl";
        let _ignored_error = std::fs::remove_file(temp_path);
        let mut a = CLFile::load("data/bb_k7_south.cl").unwrap();

        // copy to a new location
        // do not create the file until save() is called
        a.set_path(temp_path).unwrap();
        assert!(a.path.exists() == false);
        assert!(a.last_write.is_none());

        // save the file in the new location.
        a.save().unwrap();
        assert!(a.last_write.is_some());
        assert!(a.path.exists() == true);
    }

    #[test]
    fn notices_file_missing() {
        let temp_path = "data/missing_path_temp.cl";
        let _ignored_error = std::fs::remove_file(temp_path);
        let mut a = CLFile::load("data/bb_k7_south.cl").unwrap();

        // copy to a new location
        // do not create the file until save() is called
        a.set_path(temp_path).unwrap();
        assert!(a.path.exists() == false);
        assert!(a.last_write.is_none());

        // save the file in the new location.
        a.save().unwrap();
        assert!(a.last_write.is_some());
        assert!(a.path.exists() == true);

        // delete the file and expect an error
        std::fs::remove_file(&a.path).unwrap();
        assert!(a.path.exists() == false);
        assert!(matches!(
            a.check_timestamp().unwrap_err(),
            CLFError::UnexpectedFileFound
        ));
    }
}
