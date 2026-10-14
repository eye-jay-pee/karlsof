use super::{Anchor, Point};
use std::fmt::{Display, Error, Formatter};

impl Display for Anchor {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let pt: &Point = &self.point;
        write!(f, "{} we", pt)
    }
}

#[cfg(test)]
mod test {
    use super::{Anchor, Point};
    use crate::centerline::file::CLFile;

    #[test]
    fn can_print_anchors() {
        let a = CLFile::new("data/bb_k7_south.cl").unwrap();
        assert!(a.get_path().exists());
        assert!(a.check_timestamp().is_ok());

        for record in a.into_iter() {
            let pt = Anchor::new(
                Point::new(record.northing, record.easting),
                record.distance,
            );
            println!("{}", pt);
        }
    }
}
