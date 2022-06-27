use std::{fs::File, io::BufReader};

use serde::{Deserialize, Serialize};
use serde_json::from_reader as read_json;

use super::constant::conf::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Conf {
    #[serde(default)]
    pub back_folder: String,
    #[serde(default)]
    pub back_path: String,
    #[serde(default)]
    pub front_path: String,
    #[serde(default)]
    pub output_path: String,
    #[serde(default)]
    pub template_path: String,
    #[serde(default)]
    pub img_max_length: u32,
}

impl Default for Conf {
    fn default() -> Self {
        Conf {
            back_folder: DEFAULT_BACK_FOLDER.into(),
            back_path: DEFAULT_BACK_PATH.into(),
            front_path: DEFAULT_FRONT_PATH.into(),
            output_path: DEFAULT_OUTPUT_PATH.into(),
            template_path: DEFAULT_TEMPLATE_PATH.into(),
            img_max_length: DEFAULT_IMG_MAX_LENGTH,
        }
    }
}

impl Conf {
    pub fn load(path: &str) -> Self {
        let mut config = Self::default();
        if let Ok(file) = File::open(path) {
            if let Ok(json) = read_json(BufReader::new(file)) {
                config.fill_from(json);
            }
        }
        config
    }

    fn fill_from(&mut self, other: Self) {
        if !other.back_folder.is_empty() {
            self.back_folder = other.back_folder;
        }
        if !other.back_path.is_empty() {
            self.back_path = other.back_path;
        }
        if !other.front_path.is_empty() {
            self.front_path = other.front_path;
        }
        if !other.output_path.is_empty() {
            self.output_path = other.output_path;
        }
        if !other.template_path.is_empty() {
            self.template_path = other.template_path;
        }
        if other.img_max_length > 0 {
            self.img_max_length = other.img_max_length;
        }
    }
}
