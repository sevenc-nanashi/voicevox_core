use base64::{engine::general_purpose, Engine as _};
use log::info;
use voicevox_core::blocking as core;

use crate::constants::get_dict_path;

pub fn create() -> String {
    let open_jtalk = core::OpenJtalk::new(get_dict_path()).unwrap();
    let synthesizer = core::Synthesizer::new(open_jtalk, &Default::default()).unwrap();

    let synthesizer_box = Box::new(synthesizer);

    let ptr = Box::into_raw(synthesizer_box) as usize;
    info!("Created Synthesizer: {:?}", ptr);

    ptr.to_string()
}

pub fn load_model(synthesizer_ptr: usize, model_ptr: usize) -> String {
    let synthesizer = unsafe { &mut *(synthesizer_ptr as *mut core::Synthesizer<core::OpenJtalk>) };
    let model = unsafe { &*(model_ptr as *mut core::VoiceModel) };

    synthesizer.load_voice_model(model).unwrap();

    info!("Loaded model: {:?}", &model.id());

    "".to_string()
}

pub fn tts(synthesizer_ptr: usize, text: &str) -> String {
    let synthesizer = unsafe { &mut *(synthesizer_ptr as *mut core::Synthesizer<core::OpenJtalk>) };
    let metas = synthesizer.metas();
    let model_id = metas[0].styles()[0].id();

    let result = synthesizer
        .tts(text, *model_id, &Default::default())
        .unwrap();

    general_purpose::STANDARD.encode(result)
}
