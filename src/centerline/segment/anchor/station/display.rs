use super::Station;
use std::fmt::{Display, Error, Formatter};

impl Display for Station {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}+{:.2}", self.number(), self.plus())
    }
}

#[cfg(test)]
mod test {
    use super::Station;
    use crate::centerline::file::CLFile;

    #[test]
    fn can_print_stations() {
        let a = CLFile::new("data/bb_k7_south.cl").unwrap();
        assert!(a.get_path().exists());
        assert!(a.check_timestamp().is_ok());

        for record in a.into_iter() {
            if let Some(sta) = record.station() {
                println!("{}", Station::new(sta));
            }
        }
    }
}
