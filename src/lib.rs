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
