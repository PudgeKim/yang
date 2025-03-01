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
            /// ```yaml
            /// Name: Test1
            /// Npc:
            ///   - Name: Goblin
            ///     Type:
            ///       Aggressive: false
            ///     Properties:
            ///       - Hp: 100
            ///         Mp: 10
            ///   - Name: KingGoblin
            ///     Item: [Legend_Sword, Legend_Armor]
            ///     Type:
            ///       Aggressive: true
            ///     Properties:
            ///       - Hp: 1000
            ///         Mp: 300
            /// ```
            /// In the above data, Type and Properties cannot be allowed because it exceeds 2 depth.
            serde_yaml::Value::Sequence(seq) => {
                for (table_index, seq_v) in seq.iter().enumerate() {
                    if let Some(map) = seq_v.as_mapping() {
                        for (_, map_v) in map {
                            if map_v.is_mapping() {
                                return Err((Some(table_index), String::from("Nested mapping cannot be exceed 2 depth.")))
                            }
                            if let Some(inner_seq) = map_v.as_sequence() {
                                for inner_seq_v in inner_seq {
                                    if inner_seq_v.is_mapping() {
                                        return Err((Some(table_index), String::from("Nested mapping cannot be exceed 2 depth.")))
                                    }
                                }
                            }
                        }
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

    pub fn get_data_as_vec<T: DeserializeOwned>(&self, key: &str) -> Vec<T> {
        self
            .properties
            .get(key)
            .and_then(|v| v.as_sequence())
            .into_iter()
            .flat_map(|seq| seq
                .into_iter()
                .filter_map(|v| serde_yaml::from_value::<T>(v.clone()).ok())
            )
            .collect()
    }
}