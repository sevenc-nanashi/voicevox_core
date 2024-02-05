mod commands;
mod constants;
use base64::{engine::general_purpose, Engine as _};
use log::info;
use serde::{Deserialize, Serialize};
use std::ffi::{c_char, CStr, CString};
use strum::Display;

#[derive(Debug, Serialize, Deserialize, Display)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
enum Message {
    Init {},
    GetVersion {},
    ModelLoad {
        payload: ModelLoadPayload,
    },
    SynthesizerCreate {},
    SynthesizerLoadModel {
        payload: SynthesizerLoadModelPayload,
    },
    SynthesizerTts {
        payload: SynthesizerTtsPayload,
    },
}

#[derive(Debug, Serialize, Deserialize)]
struct ModelLoadPayload {
    base64: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SynthesizerLoadModelPayload {
    synthesizer: usize,
    model: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct SynthesizerTtsPayload {
    synthesizer: usize,
    text: String,
}

#[no_mangle]
pub fn message(message: *const c_char) -> *mut c_char {
    std::env::set_var("RUST_BACKTRACE", "full");
    let message = unsafe { CStr::from_ptr(message as *mut c_char) };
    let message = message.to_str().unwrap().to_string();
    let message = serde_json::from_str::<Message>(&message).unwrap();
    info!("Received: {}", message);
    let result = match message {
        Message::Init {} => commands::system::init(),
        Message::GetVersion {} => commands::system::get_version(),
        Message::ModelLoad { payload } => commands::model::load(
            &general_purpose::STANDARD
                .decode(payload.base64)
                .expect("base64 decode error"),
        ),
        Message::SynthesizerCreate {} => commands::synthesizer::create(),
        Message::SynthesizerLoadModel { payload } => {
            commands::synthesizer::load_model(payload.synthesizer, payload.model)
        }
        Message::SynthesizerTts { payload } => {
            commands::synthesizer::tts(payload.synthesizer, &payload.text)
        }
    };
    info!("Done.");
    let result = CString::new(result).unwrap();

    result.into_raw()
}
