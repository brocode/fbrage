use std::{error::Error, fmt, io, string::FromUtf8Error};

use age::{DecryptError, EncryptError};

#[derive(Debug)]
pub enum AppError {
    WrappedError(Box<dyn Error>),
    FbrageError(String),
}

macro_rules! app_error_from {
    ($error: ty) => {
        impl From<$error> for AppError {
            fn from(err: $error) -> AppError {
                AppError::WrappedError(Box::new(err))
            }
        }
    };
}

impl From<&str> for AppError {
    fn from(err: &str) -> AppError {
        AppError::FbrageError(err.to_string())
    }
}

impl From<AppError> for String {
    fn from(value: AppError) -> Self {
        match value {
            AppError::WrappedError(s) => s.to_string(),
            AppError::FbrageError(s) => s,
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::WrappedError(inner) => Some(&**inner),
            _ => None,
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::WrappedError(err) => write!(f, "Wrapped Error: {}", err),
            AppError::FbrageError(msg) => write!(f, "Fbrage Error: {}", msg),
        }
    }
}

app_error_from!(DecryptError);
app_error_from!(EncryptError);
app_error_from!(io::Error);
app_error_from!(FromUtf8Error);
