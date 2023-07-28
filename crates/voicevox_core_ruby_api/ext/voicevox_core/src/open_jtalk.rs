use crate::result::*;
use crate::user_dict::UserDict;
use magnus::{scan_args::scan_args, Error, Value};

#[magnus::wrap(class = "VoicevoxCore::OpenJtalk", free_immediately, size)]
pub struct OpenJtalk {
    open_jtalk: voicevox_core::OpenJtalk,
}

impl OpenJtalk {
    pub fn initialize(args: &[Value]) -> Result<Self, Error> {
        let args = scan_args::<(), (Option<Value>,), (), (), (), ()>(args)?;

        let (dict_dir,) = args.optional;
        let open_jtalk = if let Some(dict_dir) = dict_dir {
            let dict_dir: String = dict_dir.funcall("to_s", []).into_rb_result()?;
            voicevox_core::OpenJtalk::new_with_initialize(dict_dir).into_rb_result()?
        } else {
            voicevox_core::OpenJtalk::new_without_dic()
        };

        Ok(Self { open_jtalk })
    }

    pub fn use_user_dict(&self, user_dict: &UserDict) -> Result<(), Error> {
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
