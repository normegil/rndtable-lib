use thiserror::Error;

use crate::model::{ConversionError, RandomTable};

pub mod parser_toml;


#[derive(Error, Debug)]
pub enum DeserializerError {
    #[error("Could not deserialize '{deserialize}' into toml: {source}")]
    TOMLDeserialization { deserialize: String, source: toml::de::Error },
    #[error(transparent)]
    Conversion { #[from] source: ConversionError },
}

#[derive(Error, Debug)]
pub enum SerializerError {
    #[error("Could not serialize '{to_serialize}' into toml: {source}")]
    TOMLSerialization { to_serialize: String, source: toml::ser::Error },
}

pub trait TableDeserializer {
    fn deserialize(&self, source: &str) -> Result<RandomTable, DeserializerError>;
}

pub trait TableSerializer {
    fn serialize(&self, source: &RandomTable) -> Result<String, SerializerError>;
}

pub trait TableParser: TableSerializer + TableDeserializer {}