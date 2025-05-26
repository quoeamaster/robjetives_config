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

/// trait for back-filling.
pub trait BackFillable {
    /// back fill the current object with another object of the same type named "from".
    fn back_fill(&mut self, from: &Self);
}

/*
#[macro_export]
/// a macro to generate getter method for a struct.
/// 
/// Usage:
/// field_accessor!(MyStruct, { age, decimal_value, name, })
/// 
/// example:
/// struct MyStruct {
///     age: u8,
///     name: &str,
///     decimal_value: float16,
/// }
/// 
/// field_accessor!(MyStruct, { age, decimal_value, name, });
/// 
/// let my_struct = MyStruct { age: 12, name: "John", decimal_value: 3.14 };
/// 
/// assert_eq!(my_struct.get_field_value("name").unwrap(), "John");
/// assert_eq!(my_struct.get_field_value("age").unwrap(), 12);
/// assert_eq!(my_struct.get_field_value("decimal_value").unwrap(), 3.14);
/// assert_ne!(my_struct.get_field_value("name").unwrap(), "John");
macro_rules! field_accessor {
    // macro definition (parameters and declaration syntax)
    // ident = identifier (e.g. struct name, variable name)
    // $(,)? = optional trailing comma
    //
    // usage example: field_accessor!(MyStruct, { field1, field2, field3, })
    ($struct_name:ident, { $($field:ident),* $(,)? }) => {
        // add an impl block to the struct
        impl $struct_name {
            pub fn get_field_value(&self, field: &str) -> Option<&str> {
                match field {
                    // generate code for each field
                    // the value part would be call self.$field.as_deref() simply 
                    // -> the actual value of the field after dereferencing.
                    $( stringify!($field) => self.$field.as_deref(), )*
                    // all others field(s) available in the struct 
                    // but not declared in the macro would be treated as None.
                    _ => None,
                }
            }
        }
    };
}
*/


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

    /*
    #[test]
    /// provide a folder with one matching config file(s). Expect NO error but ONE sized HashMap.
    fn macro_field_accessor_test() {

        #[derive(PartialEq, Eq, PartialOrd, Ord)]
        struct MyStruct {
            name: String,
        }
        
        field_accessor!(MyStruct, { name, });
        
        let my_struct = MyStruct {  name: "John".to_string() };
        assert_eq!(my_struct.get_field_value("name").unwrap(), "John");
        assert_ne!(my_struct.get_field_value("name").unwrap(), "Peter-Son*3");   
    }
    */

}
