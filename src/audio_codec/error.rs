#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Conversion(String),
    
    WrongHeader,
    WrongFmt,
    WrongFmtInfo(String),

    Custom(String),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for Error {}
impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}