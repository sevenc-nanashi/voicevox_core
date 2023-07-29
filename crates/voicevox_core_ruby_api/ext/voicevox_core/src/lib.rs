mod open_jtalk;
mod result;
mod user_dict;
use crate::open_jtalk::OpenJtalk;
use crate::result::*;
use crate::user_dict::UserDict;

use magnus::{
    class, define_module, eval, function, method, prelude::*, Error, RClass, RHash, Value,
};

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("VoicevoxCore")?;
    module.const_set("VERSION", env!("CARGO_PKG_VERSION"))?;
    module.define_singleton_method("supported_devices", function!(supported_devices, 0))?;
    module.define_singleton_method(
        "_validate_pronunciation",
        function!(_validate_pronunciation, 1),
    )?;
    module.define_singleton_method("_to_zenkaku", function!(_to_zenkaku, 1))?;
    let open_jtalk = module.define_class("OpenJtalk", class::object())?;
    open_jtalk.define_method("initialize", function!(OpenJtalk::initialize, -1))?;
    open_jtalk.define_method("use_user_dict", method!(OpenJtalk::use_user_dict, 1))?;
    let user_dict = module.define_class("UserDict", class::object())?;
    user_dict.define_singleton_method("new", function!(UserDict::new, 0))?;
    user_dict.define_method("load", method!(UserDict::load, 1))?;
    user_dict.define_method("save", method!(UserDict::save, 1))?;
    user_dict.define_method("add_word", method!(UserDict::add_word, 1))?;
    user_dict.define_method("update_word", method!(UserDict::update_word, 2))?;
    user_dict.define_method("remove_word", method!(UserDict::remove_word, 1))?;
    user_dict.define_method("get_word", method!(UserDict::get_word, 1))?;
    user_dict.define_method("each", method!(UserDict::each, -1))?;
    user_dict.include_module(eval("Enumerable").unwrap())?;
    user_dict.define_alias("[]", "get_word")?;
    user_dict.define_alias("[]=", "update_word")?;
    Ok(())
}

fn supported_devices() -> Result<Value, Error> {
    let devices = voicevox_core::SupportedDevices::create().into_rb_result()?;

    let ruby_struct = eval::<RClass>("VoicevoxCore::SupportedDevices").into_rb_result()?;
    let map = RHash::new();

    map.aset("cpu", *devices.cpu())?;
    map.aset("cuda", *devices.cuda())?;
    map.aset("dml", *devices.dml())?;

    ruby_struct.new_instance((map,))
}

fn _validate_pronunciation(pronunciation: String) -> Result<(), Error> {
    voicevox_core::validate_pronunciation(&pronunciation).into_rb_result()
}

fn _to_zenkaku(s: String) -> Result<String, Error> {
    Ok(voicevox_core::to_zenkaku(&s))
}
