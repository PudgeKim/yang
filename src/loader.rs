use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use crate::errors::LoadError;
use crate::yaml_data::YamlData;

pub struct YangLoader {
    file_path: String,
    file_names: Vec<String>,
    resources: HashMap<String, Vec<YamlData>>,
}

impl YangLoader {
    pub fn new(
        file_path: String,
        file_names: Vec<String>,
    ) -> Self {
        Self {
            file_path,
            file_names,
            resources: HashMap::new(),
        }
    }

    pub fn load(&mut self) -> Result<(), LoadError> {
        let file_path = Path::new(&self.file_path);

        for file_name in self.file_names.as_slice() {
            let full_path = file_path.join(self.add_yaml(file_name));
            let file = File::open(full_path)?;
            let reader = BufReader::new(file);
            let resources: Vec<YamlData> = serde_yaml::from_reader(reader)?;
            self.resources.insert(file_name.to_string(), resources);
        }

        Ok(())
    }

    fn add_yaml(&self, file_name: &str) -> PathBuf {
        let mut p = PathBuf::from(file_name);
        p.set_extension("yaml");
        p
    }
}

impl YangLoader {
    pub fn get_all_resources(&self) -> Vec<&[YamlData]> {
        self.resources
            .values()
            .into_iter()
            .map(|e| e.as_slice())
            .collect::<Vec<_>>()
    }

    pub fn get_resource(&self, file_name: &str) -> Option<&[YamlData]> {
        self.resources.get(file_name).map(|e| e.as_slice())
    }
}
