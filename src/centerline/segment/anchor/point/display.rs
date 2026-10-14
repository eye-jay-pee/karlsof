use super::Point;
use std::fmt::{Display, Error, Formatter};

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "N:{:.3}, E:{:.3}", self.northing, self.easting)
    }
}

#[cfg(test)]
mod test {
    use super::Point;
    use crate::centerline::file::CLFile;

    #[test]
    fn can_print_points() {
        let a = CLFile::new("data/bb_k7_south.cl").unwrap();
        assert!(a.get_path().exists());
        assert!(a.check_timestamp().is_ok());

        for record in a.into_iter() {
            let pt = Point::new(record.northing, record.easting);
            println!("{}", pt);
        }
    }
}
