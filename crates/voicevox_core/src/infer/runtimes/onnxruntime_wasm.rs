#![allow(unsafe_code)]
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::sync::Mutex;
use std::{fmt::Debug, vec};

use anyhow::anyhow;
use duplicate::duplicate_item;
use ndarray::{Array, Dimension};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::devices::SupportedDevices;

use super::super::{
    DecryptModelError, InferenceRuntime, InferenceSessionOptions, InputScalarKind,
    OutputScalarKind, OutputTensor, ParamInfo, PushInputTensor,
};

static RESULTS: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Debug, Deserialize)]
struct SessionNewResult {
    handle: String,
}
#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
enum JsResult<T> {
    Ok(T),
    Err(String),
}

extern "C" {
    fn onnxruntime_inference_session_new(
        model: *const u8,
        model_len: usize,
        use_gpu: bool,
        callback: extern "C" fn(*const u8, *const u8) -> (),
    ) -> *const u8;
    fn onnxruntime_inference_session_run(
        handle: *const u8,
        inputs: *const u8,
        callback: extern "C" fn(*const u8, *const u8) -> (),
    ) -> *const u8;
    fn emscripten_sleep(millis: i32);
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub(crate) enum Onnxruntime {}

impl InferenceRuntime for Onnxruntime {
    type Session = OnnxruntimeSession;
    type RunContext<'a> = OnnxruntimeRunContext<'a>;

    fn supported_devices() -> crate::Result<SupportedDevices> {
        Ok(SupportedDevices {
            cpu: true,
            cuda: true,
            dml: false,
        })
    }

    fn new_session(
        model: impl FnOnce() -> std::result::Result<Vec<u8>, DecryptModelError>,
        options: InferenceSessionOptions,
    ) -> anyhow::Result<(
        Self::Session,
        Vec<ParamInfo<InputScalarKind>>,
        Vec<ParamInfo<OutputScalarKind>>,
    )> {
        unsafe {
            info!("creating new session");
            let model = model()?;
            let model_len = model.len();
            let cpu_num_threads = options.cpu_num_threads as usize;
            let use_gpu = options.use_gpu;
            let nonce =
                onnxruntime_inference_session_new(model.as_ptr(), model_len, use_gpu, js_callback);

            let nonce = CStr::from_ptr(nonce as *const i8)
                .to_str()
                .map_err(|err| anyhow!(err))?
                .to_string();
            info!("nonce: {}", nonce);

            let result = loop {
                let result = RESULTS.lock().expect("mutex poisoned").remove(&nonce);
                if let Some(result) = result {
                    break result;
                }
                emscripten_sleep(10);
            };

            let result: JsResult<SessionNewResult> = serde_json::from_str(&result)?;
            let result = match result {
                JsResult::Ok(result) => result,
                JsResult::Err(err) => return Err(anyhow!(err)),
            };

            let handle = result.handle;
            let session = OnnxruntimeSession { handle };
            Ok((session, vec![], vec![]))
        }
    }

    fn run(ctx: OnnxruntimeRunContext<'_>) -> anyhow::Result<Vec<OutputTensor>> {
        unsafe {
            let handle_cstr = CString::new(ctx.session.handle.clone())?;
            let inputs = serde_json::to_string(&ctx.inputs)?;
            let inputs_cstr = CString::new(inputs)?;
            let nonce = onnxruntime_inference_session_run(
                handle_cstr.into_raw() as _,
                inputs_cstr.into_raw() as _,
                js_callback,
            );
            let nonce = CStr::from_ptr(nonce as *const i8)
                .to_str()
                .map_err(|err| anyhow!(err))?
                .to_string();

            let result = loop {
                let result = RESULTS.lock().expect("mutex poisoned").remove(&nonce);
                if let Some(result) = result {
                    break result;
                }
                emscripten_sleep(10);
            };
            let result: JsResult<Vec<Tensor>> = serde_json::from_str(&result)?;
            let result = match result {
                JsResult::Ok(result) => result,
                JsResult::Err(err) => return Err(anyhow!(err)),
            };

            Ok(result
                .into_iter()
                .map(|tensor| {
                    let shape = tensor.shape;
                    match tensor.data {
                        TensorData::Int64(data) => {
                            unimplemented!()
                        }
                        TensorData::Float32(data) => {
                            OutputTensor::Float32(Array::from_shape_vec(shape, data).unwrap())
                        }
                    }
                })
                .collect())
        }
    }
}

extern "C" fn js_callback(nonce: *const u8, result: *const u8) {
    let nonce = unsafe { CStr::from_ptr(nonce as *const i8) }
        .to_str()
        .expect("invalid handle")
        .to_string();
    let result = unsafe { CStr::from_ptr(result as *const i8) }
        .to_str()
        .expect("invalid result")
        .to_string();
    info!("callback called with nonce: {}", nonce);
    RESULTS
        .lock()
        .expect("mutex poisoned")
        .insert(nonce, result);
}

pub(crate) struct OnnxruntimeSession {
    handle: String,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "kind", content = "array", rename_all = "camelCase")]
pub(crate) enum TensorData {
    Int64(Vec<i64>),
    Float32(Vec<f32>),
}
#[derive(Serialize, Deserialize)]
pub(crate) struct Tensor {
    data: TensorData,
    shape: Vec<usize>,
}

pub(crate) struct OnnxruntimeRunContext<'sess> {
    session: &'sess mut OnnxruntimeSession,
    inputs: Vec<Tensor>,
}

impl<'sess> From<&'sess mut OnnxruntimeSession> for OnnxruntimeRunContext<'sess> {
    fn from(sess: &'sess mut OnnxruntimeSession) -> Self {
        Self {
            session: sess,
            inputs: vec![],
        }
    }
}

impl PushInputTensor for OnnxruntimeRunContext<'_> {
    #[duplicate_item(
        method           T       kind_item;
        [ push_int64 ]   [ i64 ] [ Int64 ];
        [ push_float32 ] [ f32 ] [ Float32 ];
    )]
    fn method(&mut self, tensor: Array<T, impl Dimension + 'static>) -> anyhow::Result<()> {
        let shape = tensor.shape().to_vec();
        let tensor_vec = tensor.into_raw_vec();
        self.inputs.push(Tensor {
            data: TensorData::kind_item(tensor_vec),
            shape,
        });

        Ok(())
    }
}
