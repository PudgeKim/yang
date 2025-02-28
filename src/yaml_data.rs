use std::collections::HashMap;

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


}