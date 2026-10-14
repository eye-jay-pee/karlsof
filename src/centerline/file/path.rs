use super::{CLFError, CLFResult};
use std::path::{Path, PathBuf};
#[allow(dead_code)]
impl super::CLFile {
    /// Just checks if the path is valid. It does NOT check if the file exists.
    /// Nor can it, becasue paths must be verified before files are created.
    pub fn verified_path_buf(path: impl AsRef<Path>) -> CLFResult<PathBuf> {
        let ext = path
            .as_ref()
            .extension()
            .ok_or(CLFError::WrongExten("".to_string()))?
            .to_str()
            .ok_or(CLFError::Utf8)?;

        if ext != "cl" {
            Err(CLFError::WrongExten(ext.to_string()))
        } else {
            Ok(PathBuf::from(path.as_ref()))
        }
    }
    pub fn get_path(&self) -> &Path {
        &self.path
    }
    pub fn set_path(&mut self, path: impl AsRef<Path>) -> CLFResult<()> {
        self.path = Self::verified_path_buf(path)?;
        self.last_write = None;
        Ok(())
    }
    pub fn with_path(mut self, path: impl AsRef<Path>) -> CLFResult<Self> {
        self.set_path(path)?;
        Ok(self)
    }
}
