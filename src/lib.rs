#[cfg(ffi_uniffi)]
uniffi::setup_scaffolding!();

#[cfg(target_env = "ohos")]
mod _napi_inner {
    use napi::bindgen_prelude::*;
    use napi::module_init;

    #[module_init]
    fn napi_register_module_v1_ohos_init() {
        let mut modules = napi::sys::napi_module {
            nm_version: 1,
            nm_filename: std::ptr::null_mut(),
            nm_flags: 0,
            nm_modname: "jk_core_question".as_ptr().cast(),
            nm_priv: std::ptr::null_mut(),
            nm_register_func: Some(napi_register_module_v1),
            reserved: [std::ptr::null_mut(); 4],
        };
        unsafe {
            napi::sys::napi_module_register(&mut modules);
        }
    }
}

#[cfg(not(ffi_napi))]
mod error {
    #[derive(Debug, uniffi::Error)]
    pub enum Error {
        Common { reason: String },
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Error::Common { reason } => write!(f, "{}", format!("{:?}", reason)),
            }
        }
    }

    impl std::error::Error for Error {}

    pub fn error(reason: &str) -> Error {
        Error::Common {
            reason: reason.into(),
        }
    }
}

#[cfg(ffi_napi)]
mod error {
    pub(crate) use napi::bindgen_prelude::{Error, Status};
    pub fn error(str: &str) -> Error {
        Error::new(Status::Unknown, str)
    }
}

mod db;

#[cfg(target_os = "android")]
mod _inner_android {
    static VM: once_cell::sync::OnceCell<jni::JavaVM> = once_cell::sync::OnceCell::new();

    // need call this function in java/kotlin first
    #[export_name = "Java_cn_mucang_android_jk_core_question_AsyncRt_initRuntime"]
    pub extern "system" fn java_init(
        env: jni::JNIEnv,
        _class: jni::objects::JClass
    ) {
        let vm = env.get_java_vm().unwrap();
        _ = VM.set(vm);

        // take tokio for example, and we need to call uniffi's callback in the tokio worker threads -
        tokio::runtime::Builder::new_multi_thread()
            .on_thread_start(|| {
                let vm = VM.get().expect("init java vm");
                vm.attach_current_thread_permanently().unwrap();
            })
            .build()
            .unwrap();
    }
}
