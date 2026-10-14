#[allow(dead_code)]
#[derive(Debug)]
pub enum CLFError {
    Csv(csv::Error),
    Io(std::io::Error),
    WrongExten(String),
    UnexpectedFileFound,
    ExpectedFileNotFound,
    TimestampMismatch,
    Utf8,
}
pub type CLFResult<T> = Result<T, CLFError>;

// #  Note on Self::Utf8:
// `csv::Error` has utf8 error, but it contains private fields,
// so this crate cannot create new instances of it. use
// CLFError::Utf8 instead.

impl From<csv::Error> for CLFError {
    fn from(src: csv::Error) -> Self {
        Self::Csv(src)
    }
}
impl From<std::io::Error> for CLFError {
    fn from(src: std::io::Error) -> Self {
        Self::Csv(csv::Error::from(src))
    }
}
