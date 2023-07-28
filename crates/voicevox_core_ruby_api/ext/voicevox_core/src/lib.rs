use easy_ext::ext;
use magnus::{define_module, eval, function, prelude::*, Error, ExceptionClass, RClass, Value};
use std::{collections::HashMap, fmt::Display};

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("VoicevoxCore")?;
    module.const_set("VERSION", env!("CARGO_PKG_VERSION"))?;
    module.define_singleton_method("supported_devices", function!(supported_devices, 0))?;
    Ok(())
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
