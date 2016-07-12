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
pub use windows::{ansi_to_string, oem_to_string, string_to_ansi, string_to_oem};
pub mod posix;
#[cfg(any(not(windows), feature="doc"))]
pub use posix::{ansi_to_string, oem_to_string, string_to_ansi, string_to_oem};

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Result;
    #[test]
    fn oem_to_string_test() {
        to_string_test(oem_to_string);
    }
    #[test]
    fn ansi_to_string_test() {
        to_string_test(ansi_to_string);
    }
    #[test]
    fn string_to_oem_test() {
        from_string_test(string_to_oem);
    }
    #[test]
    fn string_to_ansi_test() {
        from_string_test(string_to_ansi);
    }
    fn to_string_test<F: FnOnce(&[u8]) -> Result<String>>(f: F) {
        assert_eq!(f(b"Test").unwrap(), "Test");
    }
    fn from_string_test<F: FnOnce(&str) -> Result<Vec<u8>>>(f: F) {
        assert_eq!(&f("Test").unwrap(), b"Test");
    }
}
