// emscriptenはstructの扱いがとんでもなく面倒なので、structを使わないバージョンを提供する。
use crate::*;

#[no_mangle]
pub extern "C" fn voicevox_make_default_initialize_options_wasm(
    acceleration_mode: *mut VoicevoxAccelerationMode,
    cpu_num_threads: *mut i32,
) {
    let options = voicevox_make_default_initialize_options();
    unsafe {
        *acceleration_mode = options.acceleration_mode.into();
        *cpu_num_threads = options.cpu_num_threads as i32;
    }
}

#[no_mangle]
pub unsafe extern "C" fn voicevox_synthesizer_new_wasm(
    open_jtalk: &OpenJtalkRc,
    options_acceleration_mode: VoicevoxAccelerationMode,
    options_cpu_num_threads: i32,
    out_synthesizer: NonNull<Box<VoicevoxSynthesizer>>,
) -> VoicevoxResultCode {
    let options = VoicevoxInitializeOptions {
        acceleration_mode: options_acceleration_mode.into(),
        cpu_num_threads: options_cpu_num_threads as u16,
    };
    voicevox_synthesizer_new(open_jtalk, options, out_synthesizer)
}

#[no_mangle]
pub unsafe extern "C" fn voicevox_make_default_synthesis_options_wasm(
    enable_interrogative_upspeak: *mut bool,
) {
    let options = voicevox_make_default_synthesis_options();
    unsafe {
        *enable_interrogative_upspeak = options.enable_interrogative_upspeak;
    }
}

#[no_mangle]
pub unsafe extern "C" fn voicevox_synthesizer_synthesis_wasm(
    synthesizer: &VoicevoxSynthesizer,
    audio_query_json: *const c_char,
    style_id: VoicevoxStyleId,
    options_enable_interrogative_upspeak: bool,
    output_wav_length: NonNull<usize>,
    output_wav: NonNull<*mut u8>,
) -> VoicevoxResultCode {
    let options = VoicevoxSynthesisOptions {
        enable_interrogative_upspeak: options_enable_interrogative_upspeak,
    };
    voicevox_synthesizer_synthesis(
        synthesizer,
        audio_query_json,
        style_id,
        options,
        output_wav_length,
        output_wav,
    )
}

#[no_mangle]
pub extern "C" fn voicevox_make_default_tts_options_wasm(enable_interrogative_upspeak: *mut bool) {
    let options = voicevox_make_default_tts_options();
    unsafe {
        *enable_interrogative_upspeak = options.enable_interrogative_upspeak;
    }
}

#[no_mangle]
pub unsafe extern "C" fn voicevox_synthesizer_tts_from_kana_wasm(
    synthesizer: &VoicevoxSynthesizer,
    kana: *const c_char,
    style_id: VoicevoxStyleId,
    options_enable_interrogative_upspeak: bool,
    output_wav_length: NonNull<usize>,
    output_wav: NonNull<*mut u8>,
) -> VoicevoxResultCode {
    let options = VoicevoxTtsOptions {
        enable_interrogative_upspeak: options_enable_interrogative_upspeak,
    };
    voicevox_synthesizer_tts_from_kana(
        synthesizer,
        kana,
        style_id,
        options,
        output_wav_length,
        output_wav,
    )
}

#[no_mangle]
pub unsafe extern "C" fn voicevox_synthesizer_tts_wasm(
    synthesizer: &VoicevoxSynthesizer,
    text: *const c_char,
    style_id: VoicevoxStyleId,
    options_enable_interrogative_upspeak: bool,
    output_wav_length: NonNull<usize>,
    output_wav: NonNull<*mut u8>,
) -> VoicevoxResultCode {
    let options = VoicevoxTtsOptions {
        enable_interrogative_upspeak: options_enable_interrogative_upspeak,
    };
    voicevox_synthesizer_tts(
        synthesizer,
        text,
        style_id,
        options,
        output_wav_length,
        output_wav,
    )
}
