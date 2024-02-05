use log::info;
use std::io::Write;
use voicevox_core::blocking as core;

pub fn load(model: &[u8]) -> String {
    let model_path = std::env::temp_dir().join(format!("model_{:x}.vvm", model.as_ptr() as usize));
    let mut file = std::fs::File::create(&model_path).unwrap();
    file.write_all(model).unwrap();
    let model = core::VoiceModel::from_path(model_path).unwrap();

    let id = model.id().to_string();

    let model_box = Box::new(model);

    let ptr = Box::into_raw(model_box) as usize;

    info!("Loaded model: {}@{:?}", id, ptr);

    ptr.to_string()
}
