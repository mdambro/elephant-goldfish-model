use std::fmt::Display;

use application::ports::RepositoryError;

/// Converts any storage failure into the port's error, without leaking storage types.
pub(crate) fn storage_error(error: impl Display) -> RepositoryError {
    RepositoryError(error.to_string())
}
