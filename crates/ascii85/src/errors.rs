use thiserror::Error;

#[derive(Error, Debug)]
pub enum Ascii85Error {
    #[error("Invalid format: {message}")]
    InvalidFormat { message: String },
}
