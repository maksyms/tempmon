use std::collections::HashMap;

use confy;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TMConfig {
    pub report_interval: u64,
    pub endpoints: Vec<Endpoint>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Endpoint {
    pub name: String,
    pub url: String,
    pub method: String,
    pub body: Option<String>,
    pub headers: Option<HashMap<String, String>>,
}

impl Default for TMConfig {
    fn default() -> Self {
        TMConfig {
            report_interval: 15,
            endpoints: vec![],
        }
    }
}

impl TMConfig {
    pub fn load(exename: &str) -> Self {
        confy::load(&exename, Some(exename)).unwrap_or_default()
    }

    pub fn get_config_path(exename: &str) -> String {
        confy::get_configuration_file_path(&exename, Some(exename))
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
            .to_string()
    }
}
