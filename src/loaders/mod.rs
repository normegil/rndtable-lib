use thiserror::Error;

mod filesystem;

#[derive(Error, Debug)]
pub enum ReadError {
    #[error("Could not read '{id}': {source}")]
    ReadIOError { id: String, source: std::io::Error },
}

#[derive(Error, Debug)]
pub enum WriterError {}

pub trait Loader: Reader + Writer {}

pub trait Reader {
    fn read(&self, id: String) -> Result<String, ReadError>;
}

pub trait Writer {
    fn write(&self, id: String, content: String) -> WriterError;
}
