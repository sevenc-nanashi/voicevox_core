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
    LoadModel { payload: LoadModelPayload },
}

#[derive(Debug, Serialize, Deserialize)]
struct LoadModelPayload {
    base64: String,
}

#[no_mangle]
pub fn message(message: *const c_char) -> *mut c_char {
    std::env::set_var("RUST_BACKTRACE", "full");
    let message = unsafe { CStr::from_ptr(message as *mut c_char) };
    let message = message.to_str().unwrap().to_string();
    let message = serde_json::from_str::<Message>(&message).unwrap();
    info!("Received: {}", message);
    let result = match message {
        Message::Init {} => commands::init(),
        Message::GetVersion {} => commands::get_version(),
        Message::LoadModel { payload } => commands::load_model(
            &general_purpose::STANDARD
                .decode(payload.base64)
                .expect("base64 decode error"),
        ),
    };
    info!("Done.");
    let result = CString::new(result).unwrap();

    result.into_raw()
}
