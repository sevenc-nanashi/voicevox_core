mod open_jtalk;
mod result;
mod user_dict;
mod utils;
use crate::open_jtalk::OpenJtalk;
use crate::result::*;
use crate::user_dict::UserDict;

use magnus::{class, define_module, eval, function, method, prelude::*, Error, RClass, Value};
use std::collections::HashMap;

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("VoicevoxCore")?;
    module.const_set("VERSION", env!("CARGO_PKG_VERSION"))?;
    module.define_singleton_method("supported_devices", function!(supported_devices, 0))?;
    let open_jtalk = module.define_class("OpenJtalk", class::object())?;
    open_jtalk.define_method("initialize", function!(OpenJtalk::initialize, -1))?;
    open_jtalk.define_method("use_user_dict", method!(OpenJtalk::use_user_dict, 1))?;
    let user_dict = module.define_class("UserDict", class::object())?;
    user_dict.define_singleton_method("new", function!(UserDict::new, 0))?;
    user_dict.define_method("load", method!(UserDict::load, 1))?;
    user_dict.define_method("save", method!(UserDict::save, 1))?;
    user_dict.define_method("add_word", method!(UserDict::add_word, 1))?;
    Ok(())
}

fn supported_devices() -> Result<Value, Error> {
    let devices = voicevox_core::SupportedDevices::create().into_rb_result()?;

    let ruby_struct = RClass::from_value(eval("VoicevoxCore::SupportedDevices").into_rb_result()?)
        .expect("Failed to get VoicevoxCore::SupportedDevices");
    let mut map = HashMap::new();

    map.insert("cpu", devices.cpu);
    map.insert("cuda", devices.cuda);
    map.insert("dml", devices.dml);

    ruby_struct.new_instance((map,))
}
