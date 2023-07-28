use crate::result::*;
use magnus::{eval, exception, Error, RClass, Symbol, TryConvert, Value};
use std::sync::{Arc, Mutex};
use strum::EnumString;

#[magnus::wrap(class = "VoicevoxCore::UserDict", free_immediately, size)]
pub struct UserDict {
    pub user_dict: Arc<Mutex<voicevox_core::UserDict>>,
}

impl UserDict {
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            user_dict: Arc::new(Mutex::new(voicevox_core::UserDict::new())),
        })
    }

    pub fn load(&self, path: String) -> Result<(), Error> {
        {
            let mut dict = self.user_dict.lock().expect("Failed to lock user_dict");

            dict.load(&path).into_rb_result()?;
        }
        Ok(())
    }

    pub fn save(&self, path: String) -> Result<(), Error> {
        {
            let dict = self.user_dict.lock().expect("Failed to lock user_dict");

            dict.save(&path).into_rb_result()?;
        }
        Ok(())
    }

    pub fn add_word(&self, word: Value) -> Result<(), Error> {
        if !word
            .class()
            .equal(RClass::from_value(eval("VoicevoxCore::UserDict::Word").unwrap()).unwrap())?
        {
            return Err(Error::Error(
                exception::type_error(),
                "VoicevoxCore::UserDict::Word 以外のオブジェクトを渡すことはできません".into(),
            ));
        }
        let surface: String = word.funcall("surface", []).into_rb_result()?;
        let word_type: UserDictWordType = word.funcall("word_type", [])?;
        let word = voicevox_core::UserDictWord::new(
            &surface,
            word.funcall("pronunciation", [])?,
            word.funcall("accent_type", [])?,
            word_type.into(),
            word.funcall("priority", [])?,
        )
        .into_rb_result()?;
        {
            let mut dict = self.user_dict.lock().expect("Failed to lock user_dict");

            dict.add_word(word).into_rb_result()?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum UserDictWordType {
    ProperNoun,
    CommonNoun,
    Verb,
    Adjective,
    Suffix,
}

impl TryConvert for UserDictWordType {
    fn try_convert(value: Value) -> Result<Self, Error> {
        let value = value.try_convert::<Symbol>()?.to_string();
        value.parse().map_err(|_| {
            Error::Error(
                exception::type_error(),
                format!("単語の種類が不正です: {}", value).into(),
            )
        })
    }
}

impl From<UserDictWordType> for voicevox_core::UserDictWordType {
    fn from(val: UserDictWordType) -> Self {
        match val {
            UserDictWordType::ProperNoun => voicevox_core::UserDictWordType::ProperNoun,
            UserDictWordType::CommonNoun => voicevox_core::UserDictWordType::CommonNoun,
            UserDictWordType::Verb => voicevox_core::UserDictWordType::Verb,
            UserDictWordType::Adjective => voicevox_core::UserDictWordType::Adjective,
            UserDictWordType::Suffix => voicevox_core::UserDictWordType::Suffix,
        }
    }
}
