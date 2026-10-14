use std::{
    path::{Path, PathBuf},
    time::SystemTime,
};

pub mod record;
pub use record::CLRecord;

mod error;
pub use error::{CLFError, CLFResult};

#[allow(dead_code)]
#[derive(Debug)]
pub struct CLFile {
    pub records: Vec<CLRecord>,
    path: PathBuf,
    last_write: Option<SystemTime>,
}

#[allow(dead_code)]
impl CLFile {
    pub fn new(path: impl AsRef<Path>) -> CLFResult<Self> {
        if path.as_ref().exists() {
            Self::load(path.as_ref())
        } else {
            Ok(Self {
                records: Vec::default(),
                path: Self::verified_path_buf(path.as_ref())?,
                last_write: None,
            })
        }
    }
}

mod file;
mod iterate;
mod path;
mod timestamp;
