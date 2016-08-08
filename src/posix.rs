//! UTF-8 string converting for non-Windows systems.
use std::io::{Error, ErrorKind, Result};
use super::Encoding;

/// Convert ANSI 8-bit string to String.
pub struct ANSI;
/// Convert OEM 8-bit string to String.
pub struct OEM;

impl Encoding for OEM {
    /// Convert OEM 8-bit string to String.
    ///
    /// On non-Windows systems convert UTF-8 to String.
    fn to_string(data: &[u8]) -> Result<String> {
        String::from_utf8(data.to_vec()).map_err(|e| Error::new(ErrorKind::InvalidInput, e))
    }

    /// Convert String to OEM 8-bit string.
    ///
    /// On non-Windows systems convert String to UTF-8.
    fn to_bytes(data: &str) -> Result<Vec<u8>> {
        Ok(data.as_bytes().to_vec())
    }
}

impl Encoding for ANSI {
    /// Convert ANSI 8-bit string to String.
    ///
    /// On non-Windows systems convert UTF-8 to String.
    fn to_string(data: &[u8]) -> Result<String> {
        String::from_utf8(data.to_vec()).map_err(|e| Error::new(ErrorKind::InvalidInput, e))
    }

    /// Convert String to ANSI 8-bit string.
    ///
    /// On non-Windows systems convert String to UTF-8.
    fn to_bytes(data: &str) -> Result<Vec<u8>> {
        Ok(data.as_bytes().to_vec())
    }
}
