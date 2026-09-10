#[allow(dead_code)]
#[derive(Debug)]
pub enum ParseError {
    CSV(csv::Error),
    WrongExten,
    NotUTF8,
}
pub type ParseResult<T> = Result<T, ParseError>;

impl From<csv::Error> for ParseError {
    fn from(src: csv::Error) -> Self {
        Self::CSV(src)
    }
}
impl From<std::io::Error> for ParseError {
    fn from(src: std::io::Error) -> Self {
        Self::CSV(csv::Error::from(src))
    }
}
