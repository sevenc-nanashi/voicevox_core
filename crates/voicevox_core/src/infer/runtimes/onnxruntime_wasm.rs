#![allow(unsafe_code)]
use std::{
    collections::HashMap,
    ffi::{CStr, CString},
    fmt::Debug,
    sync::{LazyLock, Mutex},
};

use anyhow::anyhow;
use duplicate::duplicate_item;
use ndarray::{Array, Dimension};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{
    devices::{DeviceSpec, GpuSpec, SupportedDevices},
    error::ErrorRepr,
};

use super::super::{
    DecryptModelError, InferenceRuntime, InferenceSessionOptions, InputScalarKind,
    OutputScalarKind, OutputTensor, ParamInfo, PushInputTensor,
};

static RESULTS: LazyLock<Mutex<HashMap<String, String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

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

pub(crate) mod blocking {
    use super::*;

    static ONNXRUNTIME: LazyLock<Mutex<Option<&'static Onnxruntime>>> = LazyLock::new(|| Mutex::new(None));
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
    pub struct Onnxruntime {}

    impl Onnxruntime {
        pub fn get() -> Option<&'static Self> {
            *ONNXRUNTIME.lock().expect("mutex poisoned")
        }

        pub fn init_once() -> crate::Result<&'static Self> {
            let mut ort = ONNXRUNTIME.lock().expect("mutex poisoned");
            if ort.is_none() {
                *ort = Some(Box::leak(Box::new(Onnxruntime {})));
            }
            Ok(&*ort.expect("ort is none"))
        }

        pub fn supported_devices(&self) -> crate::Result<SupportedDevices> {
            <Self as InferenceRuntime>::supported_devices(self)
        }
    }
}

impl InferenceRuntime for blocking::Onnxruntime {
    type Session = OnnxruntimeSession;
    type RunContext<'a> = OnnxruntimeRunContext<'a>;

    const DISPLAY_NAME: &'static str = "Web版のONNX Runtime";

    fn supported_devices(&self) -> crate::Result<SupportedDevices> {
        Ok(SupportedDevices {
            cpu: true,
            cuda: true,
            dml: false,
        })
    }

    fn new_session(
        &self,
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
            let use_gpu = options.device != DeviceSpec::Cpu;
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

    fn test_gpu(&self, gpu: GpuSpec) -> anyhow::Result<()> {
        // とりあえずGPUは使えることにする
        Ok(())
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

// とりあえずコピペ、後で整理
pub(crate) mod tokio {
    use super::*;
    use ref_cast::{ref_cast_custom, RefCastCustom};

    use crate::SupportedDevices;

    /// ONNX Runtime。
    ///
    /// シングルトンであり、インスタンスは高々一つ。
    ///
    /// # Rust APIにおけるインスタンスの共有
    ///
    /// インスタンスは[voicevox-ort]側に作られる。Rustのクレートとしてこのライブラリを利用する場合、
    /// ブロッキング版APIやvoicevox-ortを利用する他クレートともインスタンスが共有される。
    ///
    #[cfg_attr(feature = "load-onnxruntime", doc = "```")]
    #[cfg_attr(not(feature = "load-onnxruntime"), doc = "```compile_fail")]
    /// # use voicevox_core as another_lib;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// # if cfg!(windows) {
    /// #     // Windows\System32\onnxruntime.dllを回避
    /// #     voicevox_core::blocking::Onnxruntime::init_once()
    /// #         .filename(test_util::ONNXRUNTIME_DYLIB_PATH)
    /// #         .exec()?;
    /// # }
    /// let ort1 = voicevox_core::tokio::Onnxruntime::init_once().exec().await?;
    /// let ort2 = another_lib::blocking::Onnxruntime::get().expect("`ort1`と同一のはず");
    /// assert_eq!(ptr_addr(ort1), ptr_addr(ort2));
    ///
    /// fn ptr_addr(obj: &impl Sized) -> usize {
    ///     obj as *const _ as _
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [voicevox-ort]: https://github.com/VOICEVOX/ort
    #[derive(Debug, RefCastCustom)]
    #[repr(transparent)]
    pub struct Onnxruntime(pub(crate) super::blocking::Onnxruntime);

    impl Onnxruntime {
        /// ONNX Runtimeのライブラリ名。
        #[cfg(feature = "load-onnxruntime")]
        #[cfg_attr(docsrs, doc(cfg(feature = "load-onnxruntime")))]
        // ブロッキング版と等しいことはテストで担保
        pub const LIB_NAME: &'static str = "onnxruntime";

        /// 推奨されるONNX Runtimeのバージョン。
        #[cfg(feature = "load-onnxruntime")]
        #[cfg_attr(docsrs, doc(cfg(feature = "load-onnxruntime")))]
        // ブロッキング版と等しいことはテストで担保
        pub const LIB_VERSION: &'static str = ort::downloaded_version!();

        /// [`LIB_NAME`]と[`LIB_VERSION`]からなる動的ライブラリのファイル名。
        ///
        /// WindowsとAndroidでは[`LIB_UNVERSIONED_FILENAME`]と同じ。
        ///
        /// [`LIB_NAME`]: Self::LIB_NAME
        /// [`LIB_VERSION`]: Self::LIB_VERSION
        /// [`LIB_UNVERSIONED_FILENAME`]: Self::LIB_UNVERSIONED_FILENAME
        #[cfg(feature = "load-onnxruntime")]
        #[cfg_attr(docsrs, doc(cfg(feature = "load-onnxruntime")))]
        pub const LIB_VERSIONED_FILENAME: &'static str =
            super::blocking::Onnxruntime::LIB_VERSIONED_FILENAME;

        /// [`LIB_NAME`]からなる動的ライブラリのファイル名。
        ///
        /// [`LIB_NAME`]: Self::LIB_NAME
        #[cfg(feature = "load-onnxruntime")]
        #[cfg_attr(docsrs, doc(cfg(feature = "load-onnxruntime")))]
        pub const LIB_UNVERSIONED_FILENAME: &'static str =
            super::blocking::Onnxruntime::LIB_UNVERSIONED_FILENAME;

        #[ref_cast_custom]
        pub(crate) const fn from_blocking(blocking: &super::blocking::Onnxruntime) -> &Self;

        /// インスタンスが既に作られているならそれを得る。
        ///
        /// 作られていなければ`None`を返す。
        pub fn get() -> Option<&'static Self> {
            super::blocking::Onnxruntime::get().map(Self::from_blocking)
        }

        /// ONNX Runtimeをロードして初期化する。
        ///
        /// 一度成功したら、以後は引数を無視して同じ参照を返す。
        pub async fn init_once() -> crate::Result<&'static Self> {
            // TODO: ちゃんとエラーハンドリングする
            let ort = super::blocking::Onnxruntime::init_once().expect("failed to load ONNX Runtime");
            Ok(Self::from_blocking(ort))
        }

        #[cfg(test)]
        pub(crate) async fn from_test_util_data() -> anyhow::Result<&'static Self> {
            crate::task::asyncify(super::blocking::Onnxruntime::from_test_util_data)
                .await
                .map(Self::from_blocking)
        }

        /// ONNX Runtimeとして利用可能なデバイスの情報を取得する。
        pub fn supported_devices(&self) -> crate::Result<SupportedDevices> {
            self.0.supported_devices()
        }
    }
}
