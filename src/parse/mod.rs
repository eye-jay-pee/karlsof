use serde::{Deserialize, Serialize};
use std::path::PathBuf;

mod deserialize;
use deserialize::deserialize_point_number;

mod serialize;
#[allow(unused_imports)]
use serialize::serialize_point_number;

mod error;
#[allow(unused_imports)]
use error::{ParseError, ParseResult};

#[derive(Debug)]
pub enum CLRecordKind {
    LineEndPoint,
    PointOfCurve,
    RadiusPoint,
    PointOfTangent,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct CLRecord {
    #[serde(deserialize_with = "deserialize_point_number")]
    #[serde(serialize_with = "serialize_point_number")]
    id: Option<u16>,
    distance: f64,
    kind: CLRecordKind,
    x: f64,
    y: f64,
}

#[allow(dead_code)]
#[derive(Default, Debug)]
pub struct CLFile {
    records: Vec<CLRecord>,
    pathbuf: PathBuf,
}

#[allow(dead_code)]
pub mod file {
    use super::{CLFile, CLRecord, ParseError, ParseResult};
    use csv::{ReaderBuilder, WriterBuilder};
    use std::fs::File;
    use std::path::PathBuf;
    impl CLFile {
        fn path_is_ok(path_text: impl AsRef<str>) -> ParseResult<PathBuf> {
            let path_buf = PathBuf::from(path_text.as_ref());
            let ext_raw = path_buf.extension().ok_or(ParseError::WrongExten)?;
            let ext_str = ext_raw.to_str().ok_or(ParseError::NotUTF8)?;

            if ext_str == "cl" {
                Ok(path_buf)
            } else {
                Err(ParseError::WrongExten)
            }
        }
        pub fn create_new(path: impl AsRef<str>) -> ParseResult<Self> {
            let baby = Self {
                records: Vec::new(),
                pathbuf: Self::path_is_ok(path)?,
            };
            let _file = File::create_new(&baby.pathbuf)?;
            Ok(baby)
        }
        pub fn load(path: impl AsRef<str>) -> ParseResult<Self> {
            let pb = Self::path_is_ok(path)?;
            let mut rdr = ReaderBuilder::new()
                .has_headers(false)
                .from_reader(File::open(&pb)?);
            Ok(Self {
                pathbuf: pb,
                records: rdr
                    .deserialize::<CLRecord>()
                    .into_iter()
                    .collect::<Result<Vec<_>, _>>()?,
            })
        }
        pub fn save(&self) -> csv::Result<()> {
            let file = File::open(&self.pathbuf)?;
            let mut wtr =
                WriterBuilder::new().has_headers(false).from_writer(file);

            for record in self.records.iter() {
                wtr.serialize(record)?;
            }
            wtr.flush()?;
            Ok(())
        }
        pub fn save_as(&mut self, path: impl AsRef<str>) -> csv::Result<()> {
            self.pathbuf = PathBuf::from(path.as_ref());
            self.save()
        }
    }
}
