use std::{num::ParseIntError, vec};

use regex::Regex;
use thiserror::Error;

use crate::parsers::parser_toml::{TomlEntry, TomlRandomTable};

#[derive(Debug, Error)]
pub enum ConversionError {
    #[error("Keys format is invalid and cannot be parsed ({keys})")]
    InvalidKeysFormat { keys: String },
    #[error("Could not parse key (single int) into an Unsigned integer ({key})")]
    ParseSingleInt { key: String, source: ParseIntError },
    #[error("Could not parse key (multi int) into an Unsigned integer ({key})")]
    ParseMultiIntError { key: String, source: ParseIntError },
}

pub struct RandomTable {
    pub name: String,
    pub entries: Vec<Entry>,
}

impl TryFrom<TomlRandomTable> for RandomTable {
    type Error = ConversionError;

    fn try_from(value: TomlRandomTable) -> Result<Self, Self::Error> {
        let mut entries = vec![];
        for toml_entry in value.entry {
            entries.push(Entry::try_from(toml_entry)?);
        }
        Ok(Self {
            name: value.name,
            entries,
        })
    }
}

pub struct Entry {
    pub lower_bound: u32,
    pub upper_bound: u32,
    pub text: String,
}

impl TryFrom<TomlEntry> for Entry {
    type Error = ConversionError;

    fn try_from(value: TomlEntry) -> Result<Self, Self::Error> {
        let pure_number_regex =
            Regex::new(r"^[0-9]+$").expect("Hardcoded regex is not correct - It's a bug !");
        let lower_bound;
        let upper_bound;
        let keys = value.keys.trim();
        if pure_number_regex.is_match(keys) {
            lower_bound =
                keys.parse::<u32>()
                    .map_err(|source| ConversionError::ParseSingleInt {
                        key: keys.to_string(),
                        source,
                    })?;
            upper_bound = lower_bound;
        } else if keys.contains("-") {
            let splitted: Vec<&str> = keys.split('-').collect();
            if splitted.len() != 2 {
                return Err(ConversionError::InvalidKeysFormat {
                    keys: keys.to_string(),
                });
            }
            let first_key = splitted[0].parse::<u32>().map_err(|source| {
                ConversionError::ParseMultiIntError {
                    key: splitted[0].to_string(),
                    source,
                }
            })?;
            let second_key = splitted[1].parse::<u32>().map_err(|source| {
                ConversionError::ParseMultiIntError {
                    key: splitted[1].to_string(),
                    source,
                }
            })?;

            match first_key.cmp(&second_key) {
                std::cmp::Ordering::Equal => {
                    lower_bound = first_key;
                    upper_bound = lower_bound;
                }
                std::cmp::Ordering::Less => {
                    lower_bound = first_key;
                    upper_bound = second_key;
                }
                std::cmp::Ordering::Greater => {
                    lower_bound = second_key;
                    upper_bound = first_key;
                }
            }
        } else {
            return Err(ConversionError::InvalidKeysFormat { keys: value.keys });
        }

        Ok(Self {
            lower_bound,
            upper_bound,
            text: value.text,
        })
    }
}
