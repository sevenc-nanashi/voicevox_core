use std::path::PathBuf;

pub fn get_dict_path() -> PathBuf {
    std::env::temp_dir().join("openjtalk_dict")
}
