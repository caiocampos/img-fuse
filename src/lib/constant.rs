pub mod img {
    use glob::MatchOptions;

    pub const OPT: MatchOptions = MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };
}

pub mod conf {
    pub const DEFAULT_BACK_FOLDER: &str = "./img/";
    pub const DEFAULT_BACK_PATH: &str = "";
    pub const DEFAULT_FRONT_PATH: &str = "./front.png";
    pub const DEFAULT_OUTPUT_PATH: &str = "./out.png";
    pub const DEFAULT_TEMPLATE_PATH: &str = "./pattern.svg";
    pub const DEFAULT_IMG_MAX_LENGTH: u32 = 1600;

    pub const CONF_FILE: &str = "img.conf.json";
}
