use rand::Rng;

mod conf;
mod file;

fn main() {
    let max = 1000;
    let img_number = rand::thread_rng().gen_range(1, max);

    let config = conf::load_conf();

    println!("{:?}", config);

    let list = file::list_images(&config.back_folder);
    println!("{:?}", list);

    println!("{}", img_number);
}
