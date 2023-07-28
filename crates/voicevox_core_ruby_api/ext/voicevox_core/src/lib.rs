use easy_ext::ext;
use magnus::{
    class, define_module, eval, exception, function, prelude::*, scan_args::scan_args, Error,
    ExceptionClass, RClass, Symbol, TryConvert, Value,
};
use std::{
    collections::HashMap,
    fmt::Display,
    sync::{Arc, Mutex},
};
use strum::EnumString;

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("VoicevoxCore")?;
    module.const_set("VERSION", env!("CARGO_PKG_VERSION"))?;
    module.define_singleton_method("supported_devices", function!(supported_devices, 0))?;
    let open_jtalk = module.define_class("OpenJtalk", class::object())?;
    open_jtalk.define_method("initialize", function!(OpenJtalk::initialize, -1))?;
    open_jtalk.define_method("use_user_dict", function!(OpenJtalk::use_user_dict, 2))?;
    let user_dict = module.define_class("UserDict", class::object())?;
    user_dict.define_singleton_method("initialize", function!(UserDict::initialize, 0))?;
    user_dict.define_method("load", function!(UserDict::load, 2))?;
    Ok(())
}

#[magnus::wrap(class = "VoicevoxCore::OpenJtalk", free_immediately, size)]
struct OpenJtalk {
    open_jtalk: Arc<voicevox_core::OpenJtalk>,
}

impl OpenJtalk {
    fn initialize(args: &[Value]) -> Result<Self, Error> {
        let args = scan_args::<(), (Option<Value>,), (), (), (), ()>(args)?;

        let (dict_dir,) = args.optional;
        let open_jtalk = if let Some(dict_dir) = dict_dir {
            let dict_dir: String = dict_dir.funcall("to_s", []).into_rb_result()?;
            voicevox_core::OpenJtalk::new_with_initialize(dict_dir).into_rb_result()?
        } else {
            voicevox_core::OpenJtalk::new_without_dic()
        };

        Ok(Self {
            open_jtalk: Arc::new(open_jtalk),
        })
    }

    fn use_user_dict(&self, user_dict: &UserDict) -> Result<(), Error> {
        {
            let dict = user_dict
                .user_dict
                .lock()
                .expect("Failed to lock user_dict");
            self.open_jtalk.use_user_dict(&dict).into_rb_result()?;
        }
        Ok(())
    }
}

#[magnus::wrap(class = "VoicevoxCore::UserDict", free_immediately, size)]
struct UserDict {
    user_dict: Arc<Mutex<voicevox_core::UserDict>>,
}

impl UserDict {
    fn initialize() -> Result<Self, Error> {
        Ok(Self {
            user_dict: Arc::new(Mutex::new(voicevox_core::UserDict::new())),
        })
    }

    fn load(&self, path: String) -> Result<(), Error> {
        {
            let mut dict = self.user_dict.lock().expect("Failed to lock user_dict");

            dict.load(&path).into_rb_result()?;
        }
        Ok(())
    }

    fn save(&self, path: String) -> Result<(), Error> {
        {
            let dict = self.user_dict.lock().expect("Failed to lock user_dict");

            dict.save(&path).into_rb_result()?;
        }
        Ok(())
    }

    fn add_word(&self, word: String, yomi: String) -> Result<(), Error> {
        {
            let mut dict = self.user_dict.lock().expect("Failed to lock user_dict");

            dict.add_word(&word, &yomi).into_rb_result()?;
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

fn supported_devices() -> Result<Value, Error> {
    let ruby_struct = RClass::from_value(eval("VoicevoxCore::SupportedDevices").into_rb_result()?)
        .expect("Failed to get VoicevoxCore::SupportedDevices");

    let devices = voicevox_core::SupportedDevices::create().into_rb_result()?;

    let mut map = HashMap::new();

    map.insert("cpu", devices.cpu);
    map.insert("cuda", devices.cuda);
    map.insert("dml", devices.dml);

    ruby_struct.new_instance((map,))
}

#[ext]
impl<T, E: Display> Result<T, E> {
    fn into_rb_result(self) -> Result<T, Error> {
        let err_class = ExceptionClass::from_value(eval("VoicevoxCore::VoicevoxError")?).unwrap();
        self.map_err(|e| Error::Error(err_class, format!("{}", e).into()))
    }
}
