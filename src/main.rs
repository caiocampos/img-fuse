mod lib;

use lib::configuration::Conf;
use lib::image::ImageInfo;
use lib::image::SVG;
use lib::template::process;

use lib::constant::conf::CONF_FILE;

fn main() {
    let config = Conf::load(CONF_FILE);
    let front_img = ImageInfo::by_path(&config.front_path);
    let back_img = match config.back_path.is_empty() {
        true => ImageInfo::random(&config.back_folder),
        _ => ImageInfo::by_folder_path(&config.back_folder, &config.back_path),
    };
    let images: Vec<&ImageInfo> = match (&back_img, &front_img) {
        (Some(back), Some(front)) => vec![back, front],
        (Some(back), None) => vec![back],
        _ => Vec::new(),
    };
    match process(&config, &images) {
        Ok(res) => {
            let svg = SVG::from(&res);
            if let Err(err) = svg.save_png(&config.output_path) {
                panic!("{}", err);
            }
        }
        Err(err) => panic!("{}", err),
    }
}
