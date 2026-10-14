use super::Segment;
use std::fmt::{Display, Error, Formatter};

impl Display for Segment {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        // how to format segments?
        //
        // how to encode segments?

        write!(f, "not yet implemented")
    }
}

#[cfg(test)]
mod test {
    //use super::Segment;
    use crate::centerline::file::CLFile;

    #[test]
    fn can_print_segment() {
        let a = CLFile::new("data/bb_k7_south.cl").unwrap();
        assert!(a.get_path().exists());
        assert!(a.check_timestamp().is_ok());

        for window in a.records.windows(2) {
            let _start = &window[0];
            let _end = &window[1];
        }
    }
}
