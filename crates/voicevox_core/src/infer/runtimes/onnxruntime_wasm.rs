use std::any::Any;
use std::mem::ManuallyDrop;
use std::sync::Arc;
use std::{fmt::Debug, vec};

use anyhow::anyhow;
use duplicate::duplicate_item;
use ndarray::{Array, Dimension};
use once_cell::sync::Lazy;

use crate::devices::SupportedDevices;

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
            cuda: false,
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
        todo!()
    }

    fn run(ctx: OnnxruntimeRunContext<'_>) -> anyhow::Result<Vec<OutputTensor>> {
        todo!()
    }
}

pub(crate) struct OnnxruntimeSession {}

impl Drop for OnnxruntimeSession {
    fn drop(&mut self) {
        todo!()
    }
}

pub(crate) struct OnnxruntimeRunContext<'sess> {
    session: &'sess mut OnnxruntimeSession,
}

impl<'sess> From<&'sess mut OnnxruntimeSession> for OnnxruntimeRunContext<'sess> {
    fn from(sess: &'sess mut OnnxruntimeSession) -> Self {
        todo!()
    }
}

impl PushInputTensor for OnnxruntimeRunContext<'_> {
    #[duplicate_item(
        method           T;
        [ push_int64 ]   [ i64 ];
        [ push_float32 ] [ f32 ];
    )]
    fn method(&mut self, tensor: Array<T, impl Dimension + 'static>) {
        todo!()
    }
}
