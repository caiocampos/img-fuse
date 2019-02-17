use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

const CONF_FILE: &str = "img.conf.json";

const DEFAULT_BACK_FOLDER: &str = "./img/";
const DEFAULT_BACK_PATH: &str = "";
const DEFAULT_FRONT_PATH: &str = "./front.png";

#[derive(Serialize, Deserialize, Debug)]
pub struct Conf {
    #[serde(default = "String::default")]
    pub back_folder: String,
    #[serde(default = "String::default")]
    pub back_path: String,
    #[serde(default = "String::default")]
    pub front_path: String,
}

impl Default for Conf {
    fn default() -> Self {
        Conf {
            back_folder: DEFAULT_BACK_FOLDER.to_string(),
            back_path: DEFAULT_BACK_PATH.to_string(),
            front_path: DEFAULT_FRONT_PATH.to_string(),
        }
    }
}

pub fn load_conf() -> Conf {
    let mut config = match File::open(CONF_FILE) {
        Ok(file) => {
            let reader = BufReader::new(file);
            match serde_json::from_reader(reader) {
                Ok(json) => json,
                Err(_) => Conf::default(),
            }
        }
        Err(_) => Conf::default(),
    };

    if config.back_folder.is_empty() && config.back_path.is_empty() {
        config.back_folder = DEFAULT_BACK_FOLDER.to_string();
    }
    if config.front_path.is_empty() {
        config.front_path = DEFAULT_FRONT_PATH.to_string();
    }
    config
}
