
use serde::Deserialize;
use getset::{Getters, Setters};

use robjetives_config::{read_config_folder, BackFillable};

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
    #[getset(get = "pub", set = "pub")]
    pub name: Option<String>,
    #[getset(get = "pub", set = "pub")]
    pub url: Option<String>,
    #[getset(get = "pub", set = "pub")]
    pub user: Option<String>,
    #[getset(get = "pub", set = "pub")]
    pub password: Option<String>,
    #[getset(get = "pub", set = "pub")]
    pub retry: Option<u8>,

    #[getset(get = "pub", set = "pub")]
    pub app: Option<ConfigApp>,
}

#[derive(Debug, Deserialize, Getters, Setters)]
struct ConfigApp {
    #[getset(get = "pub", set = "pub")]
    pub name: Option<String>,
    #[getset(get = "pub", set = "pub")]
    pub license: Option<String>,
}

#[derive(Debug, Deserialize, Getters, Setters)]
struct Interfaces {
    #[getset(get = "pub", set = "pub")]
    pub systems: Vec<System>,
}

#[derive(Debug, Deserialize, Getters, Setters)]
struct System {
    #[getset(get = "pub", set = "pub")]
    pub name: Option<String>,
    #[getset(get = "pub", set = "pub")]
    pub url: Option<String>,
    #[getset(get = "pub", set = "pub")]
    pub user: Option<String>,
    #[getset(get = "pub", set = "pub")]
    pub password: Option<String>
}

impl BackFillable for Config {
    fn back_fill(&mut self, from: &Self) {
        if self.name.is_none() {
            self.set_name(from.name.clone());
        }
        if self.url.is_none() {
            self.set_url(from.url.clone());
        }
        if self.user.is_none() {
            self.set_user(from.user.clone());
        }
        if self.password.is_none() {
            self.set_password(from.password.clone());
        }
        if self.retry.is_none() {
            self.set_retry(from.retry);
        }

        // ConfigApp struct back_fill()
        if self.app.is_none() {
            let app = ConfigApp {
                license: from.app.as_ref().unwrap().license.clone(),
                name: from.app.as_ref().unwrap().name.clone(),
            };
            self.set_app(Some(app));

        } else {
            // it is non empty, but need to be back filled for config-app level as well
            //
            // well.. unless derive clone() to the struct earlier... 
            // might seem inefficient though... but straightforward approach
            let mut app = ConfigApp {
                license: self.app.as_ref().unwrap().license.clone(),
                name: self.app.as_ref().unwrap().name.clone(),
            };
            app.back_fill(from.app.as_ref().unwrap());
            self.set_app(Some(app));
        }
    }
}

impl BackFillable for ConfigApp {
    fn back_fill(&mut self, from: &Self) {
        if self.name.is_none() {
            self.set_name(from.name.clone());
        }
        if self.license.is_none() {
            self.set_license(from.license.clone());
        }
    }
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

    // ** back fill test
    let custom_config = read_config_folder("tests/custom-config", "toml", "config.toml").unwrap();
    let mut custom_config: Config = toml::from_str(custom_config.get("config.toml").unwrap()).unwrap();

    custom_config.back_fill(&config);

    assert_eq!(custom_config.name().as_ref().unwrap(), "fancy-app-mysql");
    assert_eq!(custom_config.url().as_ref().unwrap(), "http://localhost:3125");
    assert_eq!(custom_config.user().as_ref().unwrap(), "deborah");
    assert_eq!(custom_config.password().as_ref().unwrap(), "guide-post-hk");
    assert_eq!(custom_config.retry().unwrap(), 5);

    assert_eq!(custom_config.app().as_ref().unwrap().name().as_ref().unwrap(), "test-app");
    assert_eq!(custom_config.app().as_ref().unwrap().license().as_ref().unwrap(), "AGPL-3.0");

}
