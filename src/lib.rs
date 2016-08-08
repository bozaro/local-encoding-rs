//! Rust library for encoding/decoding string with local charset. It usefull for work with ANSI
//! strings on Windows.
//!
//! Unfortunately Windows widly use 8-bit character encoding instead UTF-8.
//! This causes a lot of pain.
//!
//! For example, in Russian version:
//!
//!  * CP-1251 (ANSI codepage) used for 8-bit files;
//!  * CP-866 (OEM codepage) used for console output.
//!
//! To convert between 8-bit and Unicode used Windows have function: MultiByteToWideChar and
//! WideCharToMultiByte.
//!
//! This library provide simple function to convert between 8-bit and Unicode characters on Windows.
//!
//! UTF-8 used as 8-bit codepage for non-Windows system.

#![warn(missing_docs)]
#[cfg(any(windows, feature="doc"))]
pub mod windows;
#[cfg(windows)]
pub use windows::{ANSI, OEM};
pub mod posix;
#[cfg(any(not(windows), feature="doc"))]
pub use posix::{ANSI, OEM};
use std::io::Result;

/// Converter between string and multibyte encoding.
pub trait Encoding {
    /// Convert from bytes to string.
    fn to_string(data: &[u8]) -> Result<String>;

    /// Convert from string to bytes.
    fn to_bytes(data: &str) -> Result<Vec<u8>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oem_to_string_test() {
        to_string_test::<OEM>();
    }
    #[test]
    fn ansi_to_string_test() {
        to_string_test::<ANSI>();
    }
    #[test]
    fn string_to_oem_test() {
        from_string_test::<OEM>();
    }
    #[test]
    fn string_to_ansi_test() {
        from_string_test::<ANSI>();
    }
    fn to_string_test<E: Encoding>() {
        assert_eq!(E::to_string(b"Test").unwrap(), "Test");
    }
    fn from_string_test<E: Encoding>() {
        assert_eq!(E::to_bytes("Test").unwrap(), b"Test");
    }
}
