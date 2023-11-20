use std::any::Any;
use std::mem::ManuallyDrop;
use std::sync::Arc;
use std::{fmt::Debug, vec};

use anyhow::anyhow;
use duplicate::duplicate_item;
use ndarray::{Array, Dimension};
use once_cell::sync::Lazy;
use ort::{
    environment::Environment, tensor::TensorElementDataType, ExecutionProvider,
    GraphOptimizationLevel, LoggingLevel, SessionBuilder, Value,
};

use crate::devices::SupportedDevices;

use self::assert_send::AssertSend;

use super::super::{
    DecryptModelError, InferenceRuntime, InferenceSessionOptions, InputScalarKind,
    OutputScalarKind, OutputTensor, ParamInfo, PushInputTensor,
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub(crate) enum Onnxruntime {}

impl InferenceRuntime for Onnxruntime {
    type Session = OnnxruntimeSession;
    type RunContext<'a> = OnnxruntimeRunContext<'a>;

    fn supported_devices() -> crate::Result<SupportedDevices> {
        Ok(SupportedDevices {
            cpu: true,
            cuda: ExecutionProvider::CUDA(Default::default()).is_available(),
            dml: ExecutionProvider::DirectML(Default::default()).is_available(),
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
        let cpu_num_threads: i16 = options
            .cpu_num_threads
            .try_into()
            .map_err(|_| anyhow!("cpu_num_threads must be in range `i16`"))?;
        let mut builder = SessionBuilder::new(&ENVIRONMENT)?
            .with_optimization_level(GraphOptimizationLevel::Level1)?
            .with_intra_threads(cpu_num_threads)?
            .with_inter_threads(cpu_num_threads)?;

        if options.use_gpu {
            #[cfg(feature = "directml")]
            {
                use ort::ExecutionMode;

                builder = builder
                    .with_disable_mem_pattern()?
                    .with_execution_mode(ExecutionMode::ORT_SEQUENTIAL)?
                    .with_append_execution_provider_directml(0)?;
            }

            #[cfg(not(feature = "directml"))]
            {
                builder = builder.with_execution_providers([])?;
            }
        }

        let model = model()?;
        let model = model.into_boxed_slice();
        let model = Box::leak(model);
        let model_ptr = model as *mut [u8];
        let sess = AssertSend::from(builder.with_model_from_memory(model)?);

        let input_param_infos = sess
            .inputs
            .iter()
            .map(|info| {
                let dt = match info.input_type {
                    TensorElementDataType::Float32 => Ok(InputScalarKind::Float32),
                    TensorElementDataType::Float64 => Err("ONNX_TENSOR_ELEMENT_DATA_TYPE_DOUBLE"),
                    TensorElementDataType::Uint8 => Err("ONNX_TENSOR_ELEMENT_DATA_TYPE_UINT8"),
                    TensorElementDataType::Int8 => Err("ONNX_TENSOR_ELEMENT_DATA_TYPE_INT8"),
                    TensorElementDataType::Uint16 => Err("ONNX_TENSOR_ELEMENT_DATA_TYPE_UINT16"),
                    TensorElementDataType::Int16 => Err("ONNX_TENSOR_ELEMENT_DATA_TYPE_INT16"),
                    TensorElementDataType::Int32 => Err("ONNX_TENSOR_ELEMENT_DATA_TYPE_INT32"),
                    TensorElementDataType::Int64 => Ok(InputScalarKind::Int64),
                    TensorElementDataType::String => Err("ONNX_TENSOR_ELEMENT_DATA_TYPE_STRING"),
                    TensorElementDataType::Uint32 => Err("ONNX_TENSOR_ELEMENT_DATA_TYPE_UINT32"),
                    TensorElementDataType::Uint64 => Err("ONNX_TENSOR_ELEMENT_DATA_TYPE_UINT64"),
                    TensorElementDataType::Bool => Err("ONNX_TENSOR_ELEMENT_DATA_TYPE_BOOL"),
                    TensorElementDataType::Float16 => Err("ONNX_TENSOR_ELEMENT_DATA_TYPE_FLOAT16"),
                    TensorElementDataType::Bfloat16 => {
                        Err("ONNX_TENSOR_ELEMENT_DATA_TYPE_BFLOAT16")
                    }
                }
                .map_err(|actual| {
                    anyhow!("unsupported input datatype `{actual}` for `{}`", info.name)
                })?;

                Ok(ParamInfo {
                    name: info.name.clone().into(),
                    dt,
                    ndim: Some(info.dimensions.len()),
                })
            })
            .collect::<anyhow::Result<_>>()?;

        let output_param_infos = sess
            .outputs
            .iter()
            .map(|info| {
                let dt = match info.output_type {
                    TensorElementDataType::Float32 => Ok(OutputScalarKind::Float32),
                    TensorElementDataType::Float64 => Err("ONNX_TENSOR_ELEMENT_DATA_TYPE_DOUBLE"),
                    TensorElementDataType::Uint8 => Err("ONNX_TENSOR_ELEMENT_DATA_TYPE_UINT8"),
                    TensorElementDataType::Int8 => Err("ONNX_TENSOR_ELEMENT_DATA_TYPE_INT8"),
                    TensorElementDataType::Uint16 => Err("ONNX_TENSOR_ELEMENT_DATA_TYPE_UINT16"),
                    TensorElementDataType::Int16 => Err("ONNX_TENSOR_ELEMENT_DATA_TYPE_INT16"),
                    TensorElementDataType::Int32 => Err("ONNX_TENSOR_ELEMENT_DATA_TYPE_INT32"),
                    TensorElementDataType::Int64 => Err("ONNX_TENSOR_ELEMENT_DATA_TYPE_INT64"),
                    TensorElementDataType::String => Err("ONNX_TENSOR_ELEMENT_DATA_TYPE_STRING"),
                    TensorElementDataType::Uint32 => Err("ONNX_TENSOR_ELEMENT_DATA_TYPE_UINT32"),
                    TensorElementDataType::Uint64 => Err("ONNX_TENSOR_ELEMENT_DATA_TYPE_UINT64"),
                    TensorElementDataType::Bool => Err("ONNX_TENSOR_ELEMENT_DATA_TYPE_BOOL"),
                    TensorElementDataType::Float16 => Err("ONNX_TENSOR_ELEMENT_DATA_TYPE_FLOAT16"),
                    TensorElementDataType::Bfloat16 => {
                        Err("ONNX_TENSOR_ELEMENT_DATA_TYPE_BFLOAT16")
                    }
                }
                .map_err(|actual| {
                    anyhow!("unsupported output datatype `{actual}` for `{}`", info.name)
                })?;

                Ok(ParamInfo {
                    name: info.name.clone().into(),
                    dt,
                    ndim: Some(info.dimensions.len()),
                })
            })
            .collect::<anyhow::Result<_>>()?;

        let sess = OnnxruntimeSession {
            sess: ManuallyDrop::new(sess),
            model: model_ptr,
        };

        return Ok((sess, input_param_infos, output_param_infos));

        static ENVIRONMENT: Lazy<Arc<Environment>> = Lazy::new(|| {
            Environment::builder()
                .with_name(env!("CARGO_PKG_NAME"))
                .with_log_level(LOGGING_LEVEL)
                .build()
                .unwrap()
                .into_arc()
        });

        const LOGGING_LEVEL: LoggingLevel = if cfg!(debug_assertions) {
            LoggingLevel::Verbose
        } else {
            LoggingLevel::Warning
        };
    }

    fn run(
        OnnxruntimeRunContext { sess, inputs, ptr }: OnnxruntimeRunContext<'_>,
    ) -> anyhow::Result<Vec<OutputTensor>> {
        // FIXME: 現状では`f32`のみ対応。実行時にsessionからdatatypeが取れるので、別の型の対応も
        // おそらく可能ではあるが、それが必要になるよりもortクレートへの引越しが先になると思われる
        // のでこのままにする。

        if !sess
            .outputs
            .iter()
            .all(|info| matches!(info.output_type, TensorElementDataType::Float32))
        {
            unimplemented!(
                "currently only `ONNX_TENSOR_ELEMENT_DATA_TYPE_FLOAT` is supported for output",
            );
        }

        let outputs = sess.run(inputs)?;

        drop(ptr);

        Ok(outputs
            .iter()
            .map(|o| OutputTensor::Float32((*(*o).try_extract().unwrap().view()).to_owned()))
            .collect())
    }
}

pub(crate) struct OnnxruntimeSession {
    sess: ManuallyDrop<AssertSend<ort::session::InMemorySession<'static>>>,
    model: *mut [u8],
}

#[allow(unsafe_code)]
unsafe impl Send for OnnxruntimeSession {}

impl Drop for OnnxruntimeSession {
    fn drop(&mut self) {
        #[allow(unsafe_code)]
        unsafe {
            // InMemorySessionのデストラクタ内でモデルのメモリに触ってるとまずい(未確認)ので手で開放する
            ManuallyDrop::drop(&mut self.sess);
            drop(Box::from_raw(self.model));
        }
    }
}

pub(crate) struct OnnxruntimeRunContext<'sess> {
    sess: &'sess mut AssertSend<ort::session::InMemorySession<'static>>,
    inputs: Vec<Value<'static>>,
    ptr: AutoDeallocVec,
}

impl<'sess> From<&'sess mut OnnxruntimeSession> for OnnxruntimeRunContext<'sess> {
    fn from(sess: &'sess mut OnnxruntimeSession) -> Self {
        Self {
            sess: &mut *sess.sess,
            inputs: vec![],
            ptr: AutoDeallocVec::new(),
        }
    }
}

impl PushInputTensor for OnnxruntimeRunContext<'_> {
    #[duplicate_item(
        method           T;
        [ push_int64 ]   [ i64 ];
        [ push_float32 ] [ f32 ];
    )]
    fn method(&mut self, tensor: Array<T, impl Dimension + 'static>) {
        let array = Box::new(tensor.into_dyn().into());
        let array = Box::leak(array);
        let array_ptr = array as *mut dyn Any;
        self.inputs
            .push(Value::from_array(self.sess.allocator(), array).unwrap());
        self.ptr.push(array_ptr);
    }
}

struct AutoDeallocVec {
    inner: Vec<*mut dyn Any>,
}

impl AutoDeallocVec {
    fn new() -> Self {
        Self { inner: vec![] }
    }

    fn push(&mut self, ptr: *mut dyn Any) {
        self.inner.push(ptr);
    }
}

impl Drop for AutoDeallocVec {
    fn drop(&mut self) {
        self.inner.drain(..).for_each(|ptr| {
            #[allow(unsafe_code)]
            unsafe {
                drop(Box::from_raw(ptr));
            }
        })
    }
}

// FIXME: 以下のことをちゃんと確認した後、onnxruntime-rs側で`Session`が`Send`であると宣言する。
// https://github.com/VOICEVOX/voicevox_core/issues/307#issuecomment-1276184614
mod assert_send {
    use std::ops::{Deref, DerefMut};

    pub(crate) struct AssertSend<T>(T);

    impl<'s> From<ort::session::InMemorySession<'s>> for AssertSend<ort::session::InMemorySession<'s>> {
        fn from(session: ort::session::InMemorySession<'s>) -> Self {
            Self(session)
        }
    }

    impl<T> Deref for AssertSend<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<T> DerefMut for AssertSend<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    // SAFETY: `Session` is probably "send"able.
    #[allow(unsafe_code)]
    unsafe impl<T> Send for AssertSend<T> {}
}
