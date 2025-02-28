use std::collections::HashMap;
use serde::de::DeserializeOwned;
use crate::errors::ErrorInfo;

#[derive(Debug, serde::Deserialize)]
pub struct YamlData {
    #[serde(rename = "Id")]
    id: i64,
    #[serde(rename = "Name")]
    name: String,
    #[serde(flatten)]
    properties: HashMap<String, serde_yaml::Value>
}

impl YamlData {
    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn validate(&self) -> Result<(), Vec<ErrorInfo>> {
        let mut errors = vec![];

        self
            .properties
            .iter()
            .for_each(|(name, v)| {
                if let Err((table_index, err_msg)) = self.validate_table_value(v) {
                    errors.push(ErrorInfo::new(name.to_string(), table_index, err_msg))
                }
            });

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn validate_table_value(&self, v: &serde_yaml::Value) -> Result<(), (Option<usize>, String)> {
        match v {
            serde_yaml::Value::Mapping(_) => {
                return Err((None, String::from("Table value cannot be a mapping type.")));
            }
            serde_yaml::Value::Sequence(seq) => {
                for (table_index, v) in seq.into_iter().enumerate() {
                    if v.is_mapping() {
                        return Err((Some(table_index), String::from("Table value can be a sequence type but the value of sequence cannot be a mapping type.")));
                    }
                }
            }
            _ => { /* Do nothing */ }
        }

        Ok(())
    }
}

impl YamlData {
    pub fn get_data<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        self
            .properties
            .get(key)
            .and_then(|v| serde_yaml::from_value(v.clone()).ok())
    }
}