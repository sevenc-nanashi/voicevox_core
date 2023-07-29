use crate::{result::*, ruby_async::future_into_rb};
use magnus::{wrap, Error};

#[wrap(class = "VoicevoxCore::VoiceModel", free_immediately, size)]
pub struct VoiceModel {
    voice_model: voicevox_core::VoiceModel,
}

impl VoiceModel {
    pub fn from_path(path: String) -> Result<Self, Error> {
        Ok(Self {
            voice_model: future_into_rb(async move {
                voicevox_core::VoiceModel::from_path(path)
                    .await
                    .into_rb_result()
            })?,
        })
    }
}
