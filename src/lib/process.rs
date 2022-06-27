use super::{
    configuration::Conf,
    image::{ImageInfo, SVG},
    template::process as template_process,
};

pub fn process(conf_file_path: &str) {
    let config = Conf::load(conf_file_path);
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
    match template_process(&config, &images) {
        Ok(res) => {
            let svg = SVG::from(&res);
            if let Err(err) = svg.save_png(&config.output_path) {
                panic!("{}", err);
            }
        }
        Err(err) => panic!("{}", err),
    }
}
