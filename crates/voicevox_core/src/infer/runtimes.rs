// #[cfg(not(target_family = "wasm"))]
// mod onnxruntime;
// #[cfg(target_family = "wasm")]
#[path = "runtimes/onnxruntime_wasm.rs"]
mod onnxruntime;

pub(crate) use self::onnxruntime::Onnxruntime;
