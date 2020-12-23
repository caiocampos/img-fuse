use std::ffi::OsStr;
use std::path::{Path, PathBuf, MAIN_SEPARATOR};

use glob::{glob_with, GlobError};
use image::image_dimensions;
use rand::{thread_rng, Rng};
use usvg::Options as SvgOptions;
use usvg::Tree as SvgTree;

use crate::lib::constant::img::OPT;

#[derive(Debug)]
pub struct ImageInfo {
    pub width: u32,
    pub height: u32,
    pub path: String,
}

impl ImageInfo {
    pub fn random<S: ToString>(folder: &S) -> Option<Self> {
        let images = list_images(folder);
        let img_number = match images.len() {
            0 => return None,
            1 => 0,
            n => thread_rng().gen_range(0..n),
        };
        Self::by_path(&images[img_number])
    }

    pub fn by_folder_path<S: ToString>(folder: &S, path: &S) -> Option<Self> {
        let path = parse_folder(folder) + &path.to_string();
        Self::by_path(&path)
    }

    pub fn by_path<P: AsRef<OsStr>>(path: &P) -> Option<Self> {
        match path.as_ref().to_str() {
            Some(p) => match image_dimensions(p) {
                Ok((w, h)) => Some(ImageInfo {
                    width: w,
                    height: h,
                    path: p.into(),
                }),
                _ => None,
            },
            _ => None,
        }
    }
}

fn parse_folder<S: ToString>(folder: &S) -> String {
    let mut folder = folder.to_string();
    if !folder.ends_with(MAIN_SEPARATOR) {
        folder.push(MAIN_SEPARATOR);
    }
    folder
}

fn list_images<S: ToString>(folder: &S) -> Vec<PathBuf> {
    let pattern = parse_folder(folder) + "*.*";
    match glob_with(&pattern, OPT) {
        Ok(paths) => paths.filter_map(filter_file).collect(),
        _ => Vec::new(),
    }
}

fn filter_file(entry: Result<PathBuf, GlobError>) -> Option<PathBuf> {
    if let Ok(path) = entry {
        if path.is_file() {
            return match path.extension() {
                Some(ext) if is_valid_ext(ext) => Some(path),
                _ => None,
            };
        }
    }
    None
}

fn is_valid_ext(extension: &OsStr) -> bool {
    if let Some(ext) = extension.to_str() {
        return match ext.to_lowercase().as_str() {
            "jpg" | "jpeg" | "png" => true,
            _ => false,
        };
    }
    false
}

pub enum SVG {
    Str(String),
    File(PathBuf),
    Data(Vec<u8>),
}

impl From<&str> for SVG {
    fn from(svg: &str) -> Self {
        SVG::Str(svg.into())
    }
}

impl From<&String> for SVG {
    fn from(svg: &String) -> Self {
        SVG::Str(svg.into())
    }
}

impl From<&PathBuf> for SVG {
    fn from(svg: &PathBuf) -> Self {
        SVG::File(svg.clone())
    }
}

impl From<&[u8]> for SVG {
    fn from(svg: &[u8]) -> Self {
        SVG::Data(Vec::from(svg))
    }
}

impl SVG {
    pub fn save_png(&self, out: &str) -> Result<(), String> {
        let rtree = match self.load() {
            Ok(tree) => tree,
            Err(err) => return Err(err),
        };
        let img = resvg::render(&rtree, usvg::FitTo::Original, None).unwrap();
        match img.save_png(Path::new(out)) {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("Could not save image \nError: {}", err.to_string())),
        }
    }

    fn load(&self) -> Result<SvgTree, String> {
        let mut opt = SvgOptions::default();
        let res = match self {
            SVG::Str(origin) => SvgTree::from_str(origin, &opt),
            SVG::File(origin) => {
                opt.path = Some(origin.into());
                SvgTree::from_file(origin, &opt)
            }
            SVG::Data(origin) => SvgTree::from_data(origin, &opt),
        };
        match res {
            Ok(tree) => Ok(tree),
            Err(err) => Err(format!("Could not load image \nError: {}", err.to_string())),
        }
    }
}
