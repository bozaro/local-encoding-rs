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
#[cfg(windows)]
pub mod windows;
pub mod posix;
use std::io::Result;

/// Converter between string and multibyte encoding.
pub trait Encoder {
    /// Convert from bytes to string.
    fn to_string(self: &Self, data: &[u8]) -> Result<String>;

    /// Convert from string to bytes.
    fn to_bytes(self: &Self, data: &str) -> Result<Vec<u8>>;
}

/// Text convertation encoding.
pub enum Encoding {
    /// Use CP_ACP codepage on Windows and UTF-8 on other systems.
    ANSI,
    /// Use CP_OEM codepage on Windows and UTF-8 on other systems.
    OEM,
}

#[cfg(windows)]
trait CodePage {
    fn codepage(self: &Self) -> u32;
}

#[cfg(windows)]
impl CodePage for Encoding {
    fn codepage(self: &Self) -> u32 {
        extern crate winapi;

        match self {
            &Encoding::ANSI => winapi::CP_ACP,
            &Encoding::OEM => winapi::CP_OEMCP,
        }
    }
}

#[cfg(windows)]
impl Encoder for Encoding {
    /// Convert from bytes to string.
    fn to_string(self: &Self, data: &[u8]) -> Result<String> {
        windows::EncoderCodePage(self.codepage()).to_string(data)

    }
    /// Convert from bytes to string.
    fn to_bytes(self: &Self, data: &str) -> Result<Vec<u8>> {
        windows::EncoderCodePage(self.codepage()).to_bytes(data)
    }
}

#[cfg(not(windows))]
impl Encoder for Encoding {
    /// Convert from bytes to string.
    fn to_string(self: &Self, data: &[u8]) -> Result<String> {
        posix::EncoderUtf8.to_string(data)

    }
    /// Convert from bytes to string.
    fn to_bytes(self: &Self, data: &str) -> Result<Vec<u8>> {
        posix::EncoderUtf8.to_bytes(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oem_to_string_test() {
        to_string_test(Encoding::OEM);
    }

    #[test]
    fn ansi_to_string_test() {
        to_string_test(Encoding::ANSI);
    }

    #[test]
    fn string_to_oem_test() {
        from_string_test(Encoding::OEM);
    }

    #[test]
    fn string_to_ansi_test() {
        from_string_test(Encoding::ANSI);
    }

    fn to_string_test(encoding: Encoding) {
        assert_eq!(encoding.to_string(b"Test").unwrap(), "Test");
    }

    fn from_string_test(encoding: Encoding) {
        assert_eq!(encoding.to_bytes("Test").unwrap(), b"Test");
    }
}
