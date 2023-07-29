use magnus::rb_sys::{AsRawValue, FromRawValue};
use magnus::{eval, value::ReprValue, Error, Ruby, Value, QNIL};
use once_cell::sync::Lazy;
use rb_sys::rb_thread_call_without_gvl;
use std::any::Any;
use std::future::Future;
use std::time::Duration;

static RUNTIME: Lazy<tokio::runtime::Runtime> =
    Lazy::new(|| tokio::runtime::Runtime::new().expect("Failed to create tokio runtime"));
unsafe extern "C" fn test(v: *mut std::os::raw::c_void) -> *mut std::os::raw::c_void {
    let f = v as *mut Box<dyn Future<Output = Result<Box<dyn Any>, Error>>>;
    let result = RUNTIME.block_on(f.as_mut().unwrap());
    v
}

unsafe extern "C" fn terminate_runtime(v: *mut std::os::raw::c_void) {
    RUNTIME.shutdown_timeout(Duration::ZERO);
}

pub fn future_into_rb<R>(f: impl Future<Output = Result<R, Error>>) -> Result<R, Error> {
    let fiber_scheduler: Value = eval("Fiber.scheduler").unwrap();
    if fiber_scheduler.equal(magnus::value::qnil())? {
        return RUNTIME.block_on(f);
    }

    unsafe {
        rb_sys::rb_thread_blocking_region();
        rb_thread_call_without_gvl(
            Some(test),
            &f as *const _ as *mut std::os::raw::c_void,
            Some(terminate_runtime),
            std::ptr::null_mut(),
        );
    };
}
