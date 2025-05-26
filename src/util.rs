use std::fs;
use std::path::Path;
use std::io;

/// read a config file's content.
/// 
/// # examples
/// ```
/// use robjetives_config::util::read_config_file;
/// 
/// let file_path = "src/util.rs";
/// let content = read_config_file(file_path).unwrap();
/// 
/// assert!(content.contains("pub fn read_config_file"));
/// assert!(content.contains("fn read_config_file_test()"));
/// ```
pub fn read_config_file<P: AsRef<Path>>(path: P) -> io::Result<String> {
    fs::read_to_string(path)
}

/// check if the file extension is supported
/// 
/// # examples
/// ``` 
/// use robjetives_config::util::is_file_extension_supported;   
/// 
/// assert!(is_file_extension_supported("toml"));
/// assert!(!is_file_extension_supported("json"));
/// assert!(!is_file_extension_supported("txt-non-supported"));
/// ```
pub fn is_file_extension_supported(file_extension: &str) -> bool {
    match file_extension {
        "toml" => true,
        "json" => false,
        // anything else
        _ => false,
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_config_file_test() {
        let content = read_config_file("src/util.rs").unwrap();

        // if the file contains ... 
        // a. pub fn read_config_file
        // b. does it contain a fn "fn read_config_file_test()"
        assert!(content.contains("pub fn read_config_file"));
        assert!(content.contains("fn read_config_file_test()"));

        //assert_eq!(result, 4);
    }
}