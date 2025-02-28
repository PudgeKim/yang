use std::io;

#[derive(Debug)]
pub struct ErrorInfo {
    name: String,
    table_index: Option<usize>,
    err_msg: String,
}

impl ErrorInfo {
    pub fn new(
        name: String,
        table_index: Option<usize>,
        err_msg: String,
    ) -> Self {
        Self { name, table_index, err_msg }
    }
}

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

