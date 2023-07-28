use easy_ext::ext;
use magnus::{eval, Error, ExceptionClass};
use std::fmt::Display;

#[ext]
pub impl<T, E: Display> Result<T, E> {
    fn into_rb_result(self) -> Result<T, Error> {
        let err_class =
            ExceptionClass::from_value(eval("VoicevoxCore::VoicevoxError").unwrap()).unwrap();
        self.map_err(|e| Error::Error(err_class, format!("{}", e).into()))
    }
}
