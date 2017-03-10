//! Shared parsing operations and values.

/// error returns an error for src being an invalid string of the target type.
pub fn error<T>(src: &str, target: &str) -> Result<T, String> {
    Err(format!("\"{}\" is not a valid \"{}\"", src, target))
}
