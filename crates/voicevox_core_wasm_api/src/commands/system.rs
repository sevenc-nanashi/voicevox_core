use crate::constants::get_dict_path;
use log::info;

static OPEN_JTALK_DICT: &[u8] = include_bytes!("../openjtalk_dict.zip");

fn init_logger() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::Builder::from_default_env()
        .target(env_logger::Target::Stderr)
        .init();
    info!("Initialized logger");
}

fn init_dict() {
    let dict_file = std::io::Cursor::new(OPEN_JTALK_DICT);
    info!("Extracting openjtalk dict");
    zip::ZipArchive::new(dict_file)
        .unwrap()
        .extract(&get_dict_path())
        .unwrap();
}

pub fn init() -> String {
    init_logger();

    init_dict();

    info!("Initialized");

    "".to_string()
}

pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
