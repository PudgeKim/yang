use std::io;

pub enum LoadError {
    IoError(io::Error),
    SerdeYamlError(serde_yaml::Error),
}

impl From<io::Error> for LoadError {
    fn from(err: io::Error) -> Self {
        LoadError::IoError(err)
    }
}

impl From<serde_yaml::Error> for LoadError {
    fn from(err: serde_yaml::Error) -> Self {
        LoadError::SerdeYamlError(err)
    }
}