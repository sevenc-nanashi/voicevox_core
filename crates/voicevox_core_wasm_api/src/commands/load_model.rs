use log::info;
use std::{ffi::c_void, io::Write};
use voicevox_core::blocking as core;

pub fn load_model(model: &[u8]) -> String {
    let mut temp_model = tempfile::NamedTempFile::new().unwrap();
    temp_model.write_all(model).unwrap();
    let temp_model_path = temp_model.path().to_str().unwrap();
    let model = core::VoiceModel::from_path(temp_model_path).unwrap();

    let model_box = Box::new(model);

    let ptr = Box::leak(model_box) as *mut core::VoiceModel as *mut c_void as usize;
    info!("Loaded model: {:?}", ptr);

    ptr.to_string()
}
