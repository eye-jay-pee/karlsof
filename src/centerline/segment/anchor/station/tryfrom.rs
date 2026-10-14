use super::Station;
use crate::centerline::file::record::CLRecord;

impl TryFrom<&CLRecord> for Station {
    type Error = ();

    fn try_from(value: &CLRecord) -> Result<Self, Self::Error> {
        match value.station() {
            None => Err(()),
            Some(value) => Ok(Station::new(value)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Station;
    use crate::centerline::file::CLFile;

    #[test]
    fn can_convert_from_file() {
        let a = CLFile::new("data/bb_k7_south.cl").unwrap();
        assert!(a.get_path().exists());
        assert!(a.check_timestamp().is_ok());

        for record in a.into_iter() {
            if let Some(sta) = Station::try_from(&record).ok() {
                println!("{}", sta);
            }
        }
    }
}
