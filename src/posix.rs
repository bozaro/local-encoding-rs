//! UTF-8 string converting for non-Windows systems.
use super::Encoder;
use std::io::{Error, ErrorKind, Result};

/// Convert UTF-8 bytes to String.
pub struct EncoderUtf8;

impl Encoder for EncoderUtf8 {
    /// Convert UTF-8 to String.
    fn to_string(self: &Self, data: &[u8]) -> Result<String> {
        String::from_utf8(data.to_vec()).map_err(|e| Error::new(ErrorKind::InvalidInput, e))
    }

    /// Convert String to UTF-8.
    fn to_bytes(self: &Self, data: &str) -> Result<Vec<u8>> {
        Ok(data.as_bytes().to_vec())
    }
}
