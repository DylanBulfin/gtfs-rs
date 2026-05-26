use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    #[cfg(feature = "schedule_parse")]
    CsvError(csv::Error),
    #[cfg(feature = "zip")]
    ZipError(zip::result::ZipError),
}

#[cfg(feature = "schedule_parse")]
impl From<csv::Error> for Error {
    fn from(value: csv::Error) -> Self {
        Self::CsvError(value)
    }
}

#[cfg(feature = "zip")]
impl From<zip::result::ZipError> for Error {
    fn from(value: zip::result::ZipError) -> Self {
        Self::ZipError(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Testing"))?;

        Ok(())
    }
}

impl std::error::Error for Error {}
