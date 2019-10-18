use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::error::Error;

use handlebars::Handlebars;

use crate::lib::configuration::Conf;
use crate::lib::image::ImageInfo;

pub fn process(conf: &Conf, images: &[&ImageInfo]) -> Result<String, String> {
    if images.is_empty() {
        return Err("No images to process".into());
    }
    let (mut handlebars, template) = (Handlebars::new(), "img");
    if let Err(err) = handlebars.register_template_file(template, conf.template_path.clone()) {
        return Err(err.description().into());
    }
    let (width, height) = to_target_length(images[0].width, images[0].height, conf.img_max_length);
    let mut data: BTreeMap<String, String> = BTreeMap::new();
    images.iter().enumerate().for_each(|(i, el)| {
        data.insert(format!("{}{}", "image", i), el.path.to_string());
    });
    data.insert("width".into(), width.to_string());
    data.insert("height".into(), height.to_string());
    match handlebars.render(template, &data) {
        Ok(res) => Ok(res),
        Err(err) => Err(err.desc),
    }
}

fn to_target_length(width: u32, height: u32, target: u32) -> (u32, u32) {
    match width.cmp(&height) {
        Ordering::Greater => (target, (height * target) / width),
        Ordering::Less => ((width * target) / height, target),
        Ordering::Equal => (target, target),
    }
}
