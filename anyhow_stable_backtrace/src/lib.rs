// This is a wrapper for `anyhow` that can capture backtraces on stable Rust.
//
// This crate is meant to be a drop-in replacement for `anyhow`,
// but it only provides a subset of `anyhow`'s functionality.
//
// This crate does not honor `RUST_BACKTRACE` and `RUST_LIB_BACKTRACE`.

use core::{
    fmt::{Debug, Formatter, Result as FmtResult},
    result::Result as CoreResult,
};
use std::error::Error as StdError;

use backtrace::Backtrace;
use derive_more::Display;
use original_anyhow::Error as AnyhowError;

pub use original_anyhow::anyhow as original_anyhow;

pub type Result<T, E = Error> = CoreResult<T, E>;

#[derive(Display)]
#[display(fmt = "{}", anyhow_error)]
pub struct Error {
    anyhow_error: AnyhowError,
    backtrace: Backtrace,
}

impl Error {
    #[must_use]
    pub fn from_anyhow_error(anyhow_error: AnyhowError) -> Self {
        Self {
            anyhow_error,
            backtrace: Backtrace::new(),
        }
    }
}

impl<E: StdError + Send + Sync + 'static> From<E> for Error {
    fn from(error: E) -> Self {
        Self::from_anyhow_error(error.into())
    }
}

impl Debug for Error {
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        // Use the alternate `Display` format for `self.anyhow_error` in case someone decides to
        // build this on nightly.
        write!(
            formatter,
            "{:#}\n\nstack backtrace:\n\n{:?}",
            self.anyhow_error, self.backtrace,
        )
    }
}

#[macro_export(local_inner_macros)]
macro_rules! anyhow {
    ($($token: tt)+) => {
        $crate::Error::from_anyhow_error(original_anyhow!($($token)+))
    };
}

#[macro_export(local_inner_macros)]
macro_rules! bail {
    ($($token: tt)+) => {
        return Err(anyhow!($($token)+));
    };
}

#[macro_export(local_inner_macros)]
macro_rules! ensure {
    ($condition: expr, $($token: tt)+) => {
        if !$condition {
            return Err(anyhow!($($token)+));
        }
    };
}
