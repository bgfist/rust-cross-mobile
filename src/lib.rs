#[cfg(ffi_uniffi)]
uniffi::setup_scaffolding!();

#[cfg(target_env = "ohos")]
mod _napi_inner {
    use std::ffi::c_char;
    use std::ffi::CString;
    use std::ptr;

    use napi::bindgen_prelude::*;
    use napi::module_init;

    extern "C" {
        pub fn napi_load_module(
            env: sys::napi_env,
            path: *const c_char,
            result: *mut sys::napi_value,
        ) -> sys::napi_status;
        pub fn napi_load_module_with_info(
            env: sys::napi_env,
            path: *const c_char,
            module_info: *const c_char,
            result: *mut sys::napi_value,
        ) -> sys::napi_status;
    }

    // FIXME: 两种加载模块的方法都没法加载华为的私有系统模块，只能加载openharmony的
    pub(crate) fn load_module<V: FromNapiValue>(env: Env, path: &str) -> Result<V> {
        let path = CString::new(path)?;
        let mut raw_value = ptr::null_mut();
        check_status!(unsafe { napi_load_module(env.raw(), path.as_ptr(), &mut raw_value) })?;
        unsafe { V::from_napi_value(env.raw(), raw_value) }
    }

    // TODO: 这个方法其实用不到
    pub(crate) fn load_sys_module<V: FromNapiValue>(env: Env, path: &str) -> Result<V> {
        let path = CString::new(path)?;
        let mut raw_value = ptr::null_mut();
        check_status!(unsafe {
            napi_load_module_with_info(env.raw(), path.as_ptr(), ptr::null(), &mut raw_value)
        })?;
        unsafe { V::from_napi_value(env.raw(), raw_value) }
    }

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

    /// 鸿蒙的openssl证书检测有问题，这里手动检测下
    #[module_init]
    fn probe_ssl_ca_cert() {
        // 鸿蒙的ca证书路径为/etc/ssl/certs/cacert.pem
        std::env::set_var(
            "SSL_CERT_FILE",
            std::path::Path::new("/etc/ssl/certs/cacert.pem"),
        );
        std::env::set_var("SSL_CERT_DIR", std::path::Path::new("/etc/ssl/certs"));
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
    pub extern "system" fn java_init(env: jni::JNIEnv, _class: jni::objects::JClass) {
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
