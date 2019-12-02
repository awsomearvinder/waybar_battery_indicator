#[derive(Debug)]
pub enum Error{
    SerdeError(serde_json::error::Error),
}

impl From<serde_json::error::Error> for Error {
    fn from(e :serde_json::error::Error) -> Error{
        Error::SerdeError(e)
    }
}