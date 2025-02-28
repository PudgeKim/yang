mod loader;
mod yaml_data;
mod errors;


#[cfg(test)]
mod tests {
    use crate::loader::YangLoader;

    #[test]
    fn load() {
        let mut loader = YangLoader::new(
            "/Users/YourName/Desktop/Resource".to_string(),
            vec!["test".to_string()]
        );
        loader.load();

        let resource = loader.get_resource("test");
        println!("resource: {:?}", resource);
    }
}
