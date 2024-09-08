#[cfg(not(target_family = "wasm"))]
pub(crate) mod onnxruntime;
#[cfg(target_family = "wasm")]
#[path = "runtimes/onnxruntime_wasm.rs"]
pub(crate) mod onnxruntime;
