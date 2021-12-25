use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
pub enum Error {
    #[error("data store disconnected")]
    FileMissing(#[from] io::Error),
    #[error("Failed to write data to {0}")]
    WriteFailed(String),
    #[error("Unknown error has occured")]
    Unknown,
}
