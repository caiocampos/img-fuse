use std::path::PathBuf;
use glob::{glob_with, MatchOptions};
use image::DynamicImage;
use rand::Rng;

pub fn load_random(folder: &String) -> DynamicImage {
    let mut pattern = folder.clone();
    if !pattern.ends_with("/") {
        pattern.push_str("/");
    }
    pattern.push_str("*.{jpg,jpeg,png}");
    let opt = MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };
    for entry in glob_with(&pattern, &opt).unwrap() {
        match entry {
            Ok(path) => println!("{:?}", path.display()),
            Err(e) => println!("{:?}", e),
        }
    }

    image::open("test.jpg").unwrap()
}

pub fn list_images(folder: &String) -> Vec<PathBuf> {
    let mut pattern = folder.clone();
    if !pattern.ends_with("/") {
        pattern.push_str("/");
    }
    pattern.push_str("*.{jpg,jpeg,png}");
    let opt = MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };
    glob_with(&pattern, &opt).unwrap().filter_map(|entry| entry.ok()).collect()
}