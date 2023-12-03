use once_cell::sync::Lazy;
use tokio::runtime::{Runtime, Builder};

pub static RUNTIME: Lazy<Runtime> = Lazy::new(|| {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("error,voicevox_core=info,voicevox_core_c_api=info,onnxruntime=info")
        .try_init();

    Builder::new_current_thread().build().unwrap()
});
