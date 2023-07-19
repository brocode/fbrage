use std::{io, string::FromUtf8Error};

use age::{DecryptError, EncryptError};

pub enum AppError {
    RuntimeError(String),
}

macro_rules! app_error_from {
    ($error: ty) => {
        impl From<$error> for AppError {
            fn from(err: $error) -> AppError {
                AppError::RuntimeError(err.to_string())
            }
        }
    };
}

impl From<AppError> for String {
    fn from(value: AppError) -> Self {
        match value {
            AppError::RuntimeError(s) => s,
        }
    }
}

app_error_from!(DecryptError);
app_error_from!(EncryptError);
app_error_from!(io::Error);
app_error_from!(FromUtf8Error);
app_error_from!(&str);
