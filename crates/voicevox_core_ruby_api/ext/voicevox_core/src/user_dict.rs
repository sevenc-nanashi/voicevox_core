use crate::result::*;
use magnus::{
    block::Proc, eval, exception, scan_args::scan_args, Error, IntoValue, RClass, RHash, Symbol,
    TryConvert, Value, QNIL,
};
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

    pub fn get_word(&self, word_uuid: String) -> Result<Value, Error> {
        let mut dict = self.user_dict.lock().expect("Failed to lock user_dict");
        dict.get_word(word_uuid.parse().into_rb_result()?)
            .map_or_else(|_| Ok(QNIL.into_value()), to_ruby_user_dict_word)
    }
    pub fn add_word(&self, word: Value) -> Result<String, Error> {
        let word = to_rust_user_dict_word(word)?;
        let uuid = {
            let mut dict = self.user_dict.lock().expect("Failed to lock user_dict");
            dict.add_word(word).into_rb_result()?
        };
        Ok(uuid.to_string())
    }
    pub fn update_word(&self, word_uuid: String, new_word: Value) -> Result<(), Error> {
        let new_word = to_rust_user_dict_word(new_word)?;
        {
            let mut dict = self.user_dict.lock().expect("Failed to lock user_dict");
            dict.update_word(word_uuid.parse().into_rb_result()?, new_word)
                .into_rb_result()?;
        }
        Ok(())
    }
    pub fn remove_word(&self, word_uuid: String) -> Result<(), Error> {
        {
            let mut dict = self.user_dict.lock().expect("Failed to lock user_dict");
            dict.remove_word(word_uuid.parse().into_rb_result()?)
                .into_rb_result()?;
        }
        Ok(())
    }
    pub fn import(&self, other: &UserDict) -> Result<(), Error> {
        {
            let mut dict = self.user_dict.lock().expect("Failed to lock user_dict");
            let other = other.user_dict.lock().expect("Failed to lock user_dict");
            dict.import(&other).into_rb_result()?;
        }
        Ok(())
    }
    pub fn each(&self, args: &[Value]) -> Result<Value, Error> {
        let args = scan_args::<(), (), (), (), (), Option<Proc>>(args)?;
        let hash = RHash::new();
        {
            let dict = self.user_dict.lock().expect("Failed to lock user_dict");
            for (uuid, word) in dict.words() {
                hash.aset(uuid.to_string(), to_ruby_user_dict_word(word)?)?;
            }
        }

        let block = args.block;
        if let Some(block) = block {
            hash.funcall_with_block("each", [], block)
        } else {
            hash.funcall("each", [])
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, EnumString, strum::Display)]
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
impl IntoValue for UserDictWordType {
    fn into_value(self) -> Value {
        Symbol::new(self.to_string()).into_value()
    }
    fn into_value_with(self, handle: &magnus::Ruby) -> Value {
        handle.sym_new(self.to_string()).into_value()
    }
}
fn to_rust_user_dict_word(word: Value) -> Result<voicevox_core::UserDictWord, Error> {
    if !word.is_kind_of(eval::<RClass>("VoicevoxCore::UserDict::Word").unwrap()) {
        return Err(Error::Error(
            exception::type_error(),
            "VoicevoxCore::UserDict::Word 以外のオブジェクトを渡すことはできません".into(),
        ));
    }
    let surface: String = word.funcall("surface", []).into_rb_result()?;
    let word_type: UserDictWordType = word.funcall("word_type", [])?;
    voicevox_core::UserDictWord::new(
        &surface,
        word.funcall("pronunciation", [])?,
        word.funcall("accent_type", [])?,
        word_type.into(),
        word.funcall("priority", [])?,
    )
    .into_rb_result()
}
fn to_ruby_user_dict_word(word: &voicevox_core::UserDictWord) -> Result<Value, Error> {
    eval::<RClass>("VoicevoxCore::UserDict::Word")?.new_instance((
        word.surface().clone(),
        word.pronunciation().clone(),
        *word.priority(),
        *word.accent_type(),
        UserDictWordType::from(word.word_type().to_owned()),
    ))
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
impl From<voicevox_core::UserDictWordType> for UserDictWordType {
    fn from(val: voicevox_core::UserDictWordType) -> Self {
        match val {
            voicevox_core::UserDictWordType::ProperNoun => UserDictWordType::ProperNoun,
            voicevox_core::UserDictWordType::CommonNoun => UserDictWordType::CommonNoun,
            voicevox_core::UserDictWordType::Verb => UserDictWordType::Verb,
            voicevox_core::UserDictWordType::Adjective => UserDictWordType::Adjective,
            voicevox_core::UserDictWordType::Suffix => UserDictWordType::Suffix,
        }
    }
}
