#[cfg(ffi_uniffi)]
uniffi::setup_scaffolding!();

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
