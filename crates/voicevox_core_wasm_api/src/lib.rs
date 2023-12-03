mod common;
use base64::{engine::general_purpose, Engine as _};
use common::RUNTIME;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::c_char;
use std::ffi::CString;
use std::io::Write;
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
enum Message {
    GetVersion(()),
    LoadModel { payload: LoadModelPayload },
}

static MODELS: Lazy<Mutex<HashMap<String, voicevox_core::VoiceModel>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Debug, Serialize, Deserialize)]
struct LoadModelPayload {
    base64: String,
}

fn get_version() -> String {
    voicevox_core::VERSION.to_string()
}

fn load_model(model: &[u8]) -> String {
    let mut temp_model = tempfile::NamedTempFile::new().unwrap();
    temp_model.write_all(model).unwrap();
    let temp_model_path = temp_model.path().to_str().unwrap();
    let model = RUNTIME
        .block_on(voicevox_core::VoiceModel::from_path(temp_model_path))
        .unwrap();
    let mut models = MODELS.lock().unwrap();
    let model_id = model.id().raw_voice_model_id().clone();
    models.insert(model_id.clone(), model);

    model_id
}

#[no_mangle]
pub fn message(message: *const c_char) -> *mut c_char {
    dbg!("message");
    let message = unsafe { CString::from_raw(message as *mut c_char) };
    let message = message.into_string().unwrap();
    dbg!(&message);
    let message = serde_json::from_str::<Message>(&message).unwrap();
    let result = match message {
        Message::GetVersion(()) => serde_json::to_string(&get_version()).unwrap(),
        Message::LoadModel { payload } => load_model(
            &general_purpose::STANDARD_NO_PAD
                .decode(payload.base64)
                .unwrap(),
        ),
    };
    let result = CString::new(result).unwrap();

    result.into_raw()
}
