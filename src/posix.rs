use std::io::{Error, ErrorKind, Result};

pub fn oem_to_string(data: &[u8]) -> Result<String> {
    String::from_utf8(data.to_vec()).map_err(|e| Error::new(ErrorKind::InvalidInput, e))
}

pub fn ansi_to_string(data: &[u8]) -> Result<String> {
    String::from_utf8(data.to_vec()).map_err(|e| Error::new(ErrorKind::InvalidInput, e))
}

pub fn string_to_oem(data: &str) -> Result<Vec<u8>> {
    Ok(data.as_bytes().to_vec())
}

pub fn string_to_ansi(data: &str) -> Result<Vec<u8>> {
    Ok(data.as_bytes().to_vec())
}
