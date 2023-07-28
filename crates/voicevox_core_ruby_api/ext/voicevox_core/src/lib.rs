use magnus::{define_module, function, prelude::*, Error};

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("VoicevoxCore")?;
    module.const_set("VERSION", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
