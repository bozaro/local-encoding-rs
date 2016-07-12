//! UTF-8 string converting for non-Windows systems.
use std::io::{Error, ErrorKind, Result};

/// Convert OEM 8-bit string to String.
///
/// On non-Windows systems convert UTF-8 to String.
pub fn oem_to_string(data: &[u8]) -> Result<String> {
    String::from_utf8(data.to_vec()).map_err(|e| Error::new(ErrorKind::InvalidInput, e))
}

/// Convert ANSI 8-bit string to String.
///
/// On non-Windows systems convert UTF-8 to String.
pub fn ansi_to_string(data: &[u8]) -> Result<String> {
    String::from_utf8(data.to_vec()).map_err(|e| Error::new(ErrorKind::InvalidInput, e))
}

/// Convert String to OEM 8-bit string.
///
/// On non-Windows systems convert String to UTF-8.
pub fn string_to_oem(data: &str) -> Result<Vec<u8>> {
    Ok(data.as_bytes().to_vec())
}

/// Convert String to ANSI 8-bit string.
///
/// On non-Windows systems convert String to UTF-8.
pub fn string_to_ansi(data: &str) -> Result<Vec<u8>> {
    Ok(data.as_bytes().to_vec())
}
