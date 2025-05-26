
use serde::Deserialize;
use getset::{Getters, Setters};

use robjetives_config::read_config_folder;

#[test]
fn folder_multi_load_test() {
    // load the default-config folder
    let back_fill_configs = read_config_folder("tests/default-config", "toml", "").unwrap();

    // load the custom-config folder
    let custom_configs = read_config_folder("tests/custom-config", "toml", "").unwrap();

    // checks
    assert_eq!(back_fill_configs.len(), 2);
    assert_eq!(custom_configs.len(), 1);

    // expected values
    let content = back_fill_configs.get("interface.toml").unwrap();
    assert_eq!(content.contains("name = \"msg-queue\""), true);
    assert_eq!(content.contains("url = \"http://localhost:9897\""), true);
    assert_eq!(content.contains("random = nah~~~"), false);

    let content = back_fill_configs.get("config.toml").unwrap();
    assert_eq!(content.contains("retry = 5"), true);
    assert_eq!(content.contains("license = \"MIT\""), true);
    assert_eq!(content.contains("random = nah~~~"), false);

    let content = custom_configs.get("config.toml").unwrap();
    assert_eq!(content.contains("user = \"deborah\""), true);
    assert_eq!(content.contains("license = \"AGPL-3.0\""), true);
    assert_eq!(content.contains("random = nah~~~"), false);
}

#[test]
fn folder_single_load_test() {
    let back_fill_config = read_config_folder("tests/default-config", "toml", "config.toml").unwrap();

    // checks
    assert_eq!(back_fill_config.len(), 1);

    // check contents
    let content = back_fill_config.get("config.toml").unwrap();
    assert_eq!(content.contains("retry = 5"), true);
    assert_eq!(content.contains("license = \"MIT\""), true);
    assert_eq!(content.contains("url = \"http://localhost:3125\""), true);
    assert_eq!(content.contains("random = nah~~~"), false);
}

#[derive(Debug, Deserialize, Getters, Setters)]
struct Config {
    #[get = "pub"]
    #[set = "pub"]
    pub name: Option<String>,
    #[get = "pub"]
    #[set = "pub"]
    pub url: Option<String>,
    #[get = "pub"]
    #[set = "pub"]
    pub user: Option<String>,
    #[get = "pub"]
    #[set = "pub"]
    pub password: Option<String>,
    #[get = "pub"]
    #[set = "pub"]
    pub retry: Option<u8>,

    #[get = "pub"]
    #[set = "pub"]
    pub app: Option<ConfigApp>,
}

#[derive(Debug, Deserialize, Getters, Setters)]
struct ConfigApp {
    #[get = "pub"]
    #[set = "pub"]
    pub name: Option<String>,
    #[get = "pub"]
    #[set = "pub"]
    pub license: Option<String>,
}

#[derive(Debug, Deserialize, Getters, Setters)]
struct Interfaces {
    #[get = "pub"]
    #[set = "pub"]
    pub systems: Vec<System>,
}

#[derive(Debug, Deserialize, Getters, Setters)]
struct System {
    #[get = "pub"]
    #[set = "pub"]
    pub name: Option<String>,
    #[get = "pub"]
    #[set = "pub"]
    pub url: Option<String>,
    #[get = "pub"]
    #[set = "pub"]
    pub user: Option<String>,
    #[get = "pub"]
    #[set = "pub"]
    pub password: Option<String>
}

#[test]
fn config_file_deserialization_test() {
    let back_fill_configs = read_config_folder("tests/default-config", "toml", "").unwrap();

    // conversion
    // assume no error
    let config: Config = toml::from_str(back_fill_configs.get("config.toml").unwrap()).unwrap();
    
    assert_eq!(config.app().as_ref().unwrap().license().as_ref().unwrap(), "MIT");
    assert_eq!(config.retry().unwrap(), 5);
    assert_eq!(config.user().as_ref().unwrap(), "root");
    assert_ne!(config.user().as_ref().unwrap(), "hahaha-wrong-value");

    let interface_obj: Interfaces = toml::from_str(back_fill_configs.get("interface.toml").unwrap()).unwrap();
    
    assert_eq!(interface_obj.systems().len(), 2);
    assert_eq!(interface_obj.systems().get(0).unwrap().url().as_ref().unwrap(), "http://localhost:8090");
    assert_eq!(interface_obj.systems().get(0).unwrap().user().as_ref().unwrap(), "guest");
    assert_ne!(interface_obj.systems().get(0).unwrap().user().as_ref().unwrap(), "hahaha-wrong-value");

    assert_eq!(interface_obj.systems().get(1).unwrap().url().as_ref().unwrap(), "http://localhost:9897");
    assert_eq!(interface_obj.systems().get(1).unwrap().name().as_ref().unwrap(), "mem-cache");
    assert_ne!(interface_obj.systems().get(1).unwrap().name().as_ref().unwrap(), "hahaha-wrong-value");
}
