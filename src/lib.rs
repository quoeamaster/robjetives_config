use std::io;
use std::io::Error;
use std::io::ErrorKind;
use std::collections::HashMap;
use std::fs;
use std::path;

use util::is_file_extension_supported;


// util module make it public; however if the fn within util module is private... they are still inaccessible.
pub mod util;

/// read all config files in a folder. Only file(s) that match the provide extension will be read. 
/// In case expect_config_file parameter is valid, that means single-file-mode; else multi-file-mode.
/// 
/// Differences between the modes is that in multi-file mode, the function returns all configs found. 
/// Whilst in single-file-mode, the function returns only the expected config file mentioned under the expect_config_file parameter.
pub fn read_config_folder(folder_path: &str, file_extension: &str, expect_config_file: &str) -> io::Result<HashMap<String, String>> {
    // validation (stop the function if invalid parameters are provided, more efficient)
    if !is_file_extension_supported(file_extension) {
        return Err(Error::new(ErrorKind::InvalidInput, "invalid file extension"));
    }

    let mut configs = HashMap::new();
    // is mutli or single file mode?
    let is_multi_file_mode = expect_config_file.trim().is_empty();

    // loop through all files in the folder
    for entry in fs::read_dir(folder_path)? {
        let entry = entry?;
        let path = entry.path();
        // [tip] path.file_name() only returns the final file name and not full-path.
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();

        // file matched the expression?
        // [alt] use file_name.ends_with("".concat(file_extension)) also can...
        if path.extension().unwrap().to_str().unwrap().to_string() != file_extension {
            continue;
        }
        
        // based on mode...
        if is_multi_file_mode || (file_name == expect_config_file) {
            let content = read_file_content(&path).unwrap();
            configs.insert(file_name, content);
        } else {
            continue;
        }
    }
    Ok(configs)
}

fn read_file_content(path: &path::PathBuf) -> io::Result<String> {
    Ok(util::read_config_file(path).unwrap())
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// provide a folder with no matching config file(s). Expect NO error but ZERO sized HashMap.
    fn read_config_folder_no_config_file_available_test() {
        let result = read_config_folder("src", "toml", "").unwrap();
        // expected 0 content as no toml file in src folder
        assert_eq!(result.len(), 0)
    }

    #[test]
    /// provide a non supported extension. Expect to have an error.
    fn read_config_folder_non_support_ext_test() {
        let result = read_config_folder("src", "rs", "");
        match result {
            Err(e) => assert_eq!(e.kind(), ErrorKind::InvalidInput),
            // should not happen in this use case
            _ => assert!(false),
        }
    }

}
