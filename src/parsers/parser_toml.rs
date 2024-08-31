use serde::{Deserialize, Serialize};

use crate::model::{Entry, RandomTable};

use super::{DeserializerError, SerializerError, TableDeserializer, TableParser, TableSerializer};

pub struct Parser {}

impl TableParser for Parser {}

impl TableDeserializer for Parser {
    fn deserialize(&self, source: &str) -> Result<RandomTable, DeserializerError> {
        let toml_rnd_table: TomlRandomTable =
            toml::from_str(source).map_err(|source| DeserializerError::TOMLDeserialization {
                deserialize: source.to_string(),
                source,
            })?;
        Ok(toml_rnd_table.try_into()?)
    }
}

impl TableSerializer for Parser {
    fn serialize(&self, source: &RandomTable) -> Result<String, SerializerError> {
        let toml_rndtable-lib = TomlRandomTable::from(source);
        toml::to_string_pretty(&toml_rndtable-lib).map_err(|source| {
            SerializerError::TOMLSerialization {
                to_serialize: format!("{:?}", source),
                source,
            }
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TomlRandomTable {
    pub name: String,
    pub entry: Vec<TomlEntry>,
}

impl From<&RandomTable> for TomlRandomTable {
    fn from(value: &RandomTable) -> Self {
        let mut entry = Vec::new();

        for source_entry in &value.entries {
            entry.push(TomlEntry::from(source_entry))
        }

        Self {
            entry,
            name: value.name.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TomlEntry {
    pub keys: String,
    pub text: String,
}

impl From<&Entry> for TomlEntry {
    fn from(value: &Entry) -> Self {
        let keys = match value.lower_bound.cmp(&value.upper_bound) {
            std::cmp::Ordering::Equal => value.lower_bound.to_string(),
            std::cmp::Ordering::Less => {
                value.lower_bound.to_string() + "-" + &value.upper_bound.to_string()
            }
            std::cmp::Ordering::Greater => {
                value.upper_bound.to_string() + "-" + &value.lower_bound.to_string()
            }
        };
        TomlEntry {
            keys,
            text: value.text.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Entry;

    #[test]
    fn test_deserialize() {
        let toml_data = r#"
            name = "Sample Table"
            [[entry]]
            keys = "1-3"
            text = "Entry 1-3"
            [[entry]]
            keys = "4-6"
            text = "Entry 4-6"
            [[entry]]
            keys = "7"
            text = "Entry 7"
        "#;

        let result = Parser {}.deserialize(toml_data);
        let random_table = result.unwrap();

        assert_eq!(random_table.name, "Sample Table");
        assert_eq!(random_table.entries.len(), 3);

        assert_eq!(random_table.entries[0].lower_bound, 1);
        assert_eq!(random_table.entries[0].upper_bound, 3);
        assert_eq!(random_table.entries[0].text, "Entry 1-3");

        assert_eq!(random_table.entries[1].lower_bound, 4);
        assert_eq!(random_table.entries[1].upper_bound, 6);
        assert_eq!(random_table.entries[1].text, "Entry 4-6");

        assert_eq!(random_table.entries[2].lower_bound, 7);
        assert_eq!(random_table.entries[2].upper_bound, 7);
        assert_eq!(random_table.entries[2].text, "Entry 7");
    }

    #[test]
    fn test_serialize() {
        let random_table = RandomTable {
            name: "Sample Table".to_string(),
            entries: vec![
                Entry {
                    lower_bound: 1,
                    upper_bound: 3,
                    text: String::from("First entry"),
                },
                Entry {
                    lower_bound: 4,
                    upper_bound: 4,
                    text: String::from("Second entry"),
                },
            ],
        };

        let result = Parser {}.serialize(&random_table);

        let toml_string = result.unwrap();
        let expected_toml = r#"
name = "Sample Table"

[[entry]]
keys = "1-3"
text = "First entry"

[[entry]]
keys = "4"
text = "Second entry"
"#;

        assert_eq!(toml_string.trim(), expected_toml.trim());
    }
}
