//! 8-bit string converters for Windows systems.
extern crate winapi;

use std::ptr;
use std::io::{Error, ErrorKind, Result};
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use self::winapi::shared::minwindef::{BOOL, DWORD};
use self::winapi::um::stringapiset::{MultiByteToWideChar, WideCharToMultiByte};
use self::winapi::um::winnt::LPSTR;
use super::Encoder;

#[cfg(test)]
use self::winapi::um::winnls::{CP_ACP, CP_UTF8};

/// Always use precomposed characters, that is, characters having a single character value for
/// a base or nonspacing character combination.
pub const MB_PRECOMPOSED: DWORD = 0x00000001;
/// Always use decomposed characters, that is, characters in which a base character and one or more
/// nonspacing characters each have distinct code point values.
pub const MB_COMPOSITE: DWORD = 0x00000002;
/// Use glyph characters instead of control characters.
pub const MB_USEGLYPHCHARS: DWORD = 0x00000004;
/// Fail if an invalid input character is encountered.
pub const MB_ERR_INVALID_CHARS: DWORD = 0x00000008;
/// Convert composite characters, consisting of a base character and a nonspacing character,
/// each with different character values.
pub const WC_COMPOSITECHECK: DWORD = 0x00000200;
/// Discard nonspacing characters during conversion.
pub const WC_DISCARDNS: DWORD = 0x00000010;
/// Default. Generate separate characters during conversion.
pub const WC_SEPCHARS: DWORD = 0x00000020;
/// Replace exceptions with the default character during conversion.
pub const WC_DEFAULTCHAR: DWORD = 0x00000040;
/// Fail if an invalid input character is encountered.
pub const WC_ERR_INVALID_CHARS: DWORD = 0x00000080;
/// Translate any Unicode characters that do not translate directly to multibyte equivalents to
/// the default character specified by lpDefaultChar.
pub const WC_NO_BEST_FIT_CHARS: DWORD = 0x00000400;

/// Encoding for use WinAPI calls: MultiByteToWideChar and WideCharToMultiByte.
pub struct EncoderCodePage(pub u32);

impl Encoder for EncoderCodePage {
    ///     Convert from bytes to string.
    fn to_string(self: &Self, data: &[u8]) -> Result<String> {
        multi_byte_to_wide_char(self.0, MB_ERR_INVALID_CHARS, data)
    }

    /// Convert from string to bytes.
    fn to_bytes(self: &Self, data: &str) -> Result<Vec<u8>> {
        string_to_multibyte(self.0, data, None)
    }
}

/// Convert String to 8-bit string.
///
/// * `codepage`     - Code page to use in performing the conversion. This parameter can be set to
///                    the value of any code page that is installed or available in the operating
///                    system.
/// * `data`         - Source string.
/// * `default_char` - Optional character for replace to use if a character cannot be represented
///                    in the specified code page.
///
/// Returns `Err` if an invalid input character is encountered and `default_char` is `None`.
pub fn string_to_multibyte(codepage: DWORD,
                           data: &str,
                           default_char: Option<u8>)
                           -> Result<Vec<u8>> {
    let wstr: Vec<u16> = OsStr::new(data).encode_wide().collect();
    wide_char_to_multi_byte(codepage,
                            WC_COMPOSITECHECK,
                            &wstr,
                            default_char,
                            default_char.is_none())
        .and_then(|(data, invalid)| if invalid {
            Err(Error::new(ErrorKind::InvalidInput,
                           "Can't convert some characters to multibyte charset"))
        } else {
            Ok(data)
        })
}

/// Wrapper for MultiByteToWideChar.
///
/// See https://msdn.microsoft.com/en-us/library/windows/desktop/dd319072(v=vs.85).aspx
/// for more details.
pub fn multi_byte_to_wide_char(codepage: DWORD,
                               flags: DWORD,
                               multi_byte_str: &[u8])
                               -> Result<String> {
    // Empty string
    if multi_byte_str.len() == 0 {
        return Ok(String::new());
    }
    unsafe {
        // Get length of UTF-16 string
        let len = MultiByteToWideChar(codepage,
                                                flags,
                                                multi_byte_str.as_ptr() as LPSTR,
                                                multi_byte_str.len() as i32,
                                                ptr::null_mut(),
                                                0);
        if len > 0 {
            // Convert to UTF-16
            let mut wstr: Vec<u16> = Vec::with_capacity(len as usize);
            wstr.set_len(len as usize);
            let len = MultiByteToWideChar(codepage,
                                                    flags,
                                                    multi_byte_str.as_ptr() as LPSTR,
                                                    multi_byte_str.len() as i32,
                                                    wstr.as_mut_ptr(),
                                                    len);
            if len > 0 {
                return String::from_utf16(&wstr[0..(len as usize)])
                    .map_err(|e| Error::new(ErrorKind::InvalidInput, e));
            }
        }
        Err(Error::last_os_error())
    }

}

/// Wrapper for WideCharToMultiByte.
///
/// See https://msdn.microsoft.com/ru-ru/library/windows/desktop/dd374130(v=vs.85).aspx
/// for more details.
pub fn wide_char_to_multi_byte(codepage: DWORD,
                               flags: DWORD,
                               wide_char_str: &[u16],
                               default_char: Option<u8>,
                               use_default_char_flag: bool)
                               -> Result<(Vec<u8>, bool)> {
    // Empty string
    if wide_char_str.len() == 0 {
        return Ok((Vec::new(), false));
    }
    unsafe {
        // Get length of multibyte string
        let len = WideCharToMultiByte(codepage,
                                                flags,
                                                wide_char_str.as_ptr(),
                                                wide_char_str.len() as i32,
                                                ptr::null_mut(),
                                                0,
                                                ptr::null(),
                                                ptr::null_mut());

        if len > 0 {
            // Convert from UTF-16 to multibyte
            let mut astr: Vec<u8> = Vec::with_capacity(len as usize);
            astr.set_len(len as usize);
            let default_char_ref: [i8; 1] = match default_char {
                Some(c) => [c as i8],
                None => [0],
            };
            let mut use_char_ref: [BOOL; 1] = [0];
            let len = WideCharToMultiByte(codepage,
                                                    flags,
                                                    wide_char_str.as_ptr(),
                                                    wide_char_str.len() as i32,
                                                    astr.as_mut_ptr() as LPSTR,
                                                    len,
                                                    match default_char {
                                                        Some(_) => default_char_ref.as_ptr(),
                                                        None => ptr::null(),
                                                    },
                                                    match use_default_char_flag {
                                                        true => use_char_ref.as_mut_ptr(),
                                                        false => ptr::null_mut(),
                                                    });
            if (len as usize) == astr.len() {
                return Ok((astr, use_char_ref[0] != 0));
            }
            if len > 0 {
                return Ok((astr[0..(len as usize)].to_vec(), use_char_ref[0] != 0));
            }
        }
        Err(Error::last_os_error())
    }
}

#[test]
fn multi_byte_to_wide_char_empty() {
    assert_eq!(multi_byte_to_wide_char(CP_ACP, MB_ERR_INVALID_CHARS, b"").unwrap(),
               "");
}

#[test]
fn multi_byte_to_wide_char_ascii() {
    assert_eq!(multi_byte_to_wide_char(CP_ACP, MB_ERR_INVALID_CHARS, b"Test").unwrap(),
               "Test");
}

#[test]
fn multi_byte_to_wide_char_utf8() {
    assert_eq!(multi_byte_to_wide_char(CP_UTF8,
                                       MB_ERR_INVALID_CHARS,
                                       b"\xD0\xA2\xD0\xB5\xD1\x81\xD1\x82")
                   .unwrap(),
               "Тест");
}

#[test]
fn multi_byte_to_wide_char_invalid() {
    assert!(multi_byte_to_wide_char(CP_UTF8, MB_ERR_INVALID_CHARS, b"Test\xC0").is_err());
}

#[test]
fn wide_char_to_multi_byte_empty() {
    assert_eq!(wide_char_to_multi_byte(CP_UTF8, WC_ERR_INVALID_CHARS, &[], None, false)
                   .unwrap(),
               (b"".to_vec(), false));
}

#[test]
fn wide_char_to_multi_byte_ascii() {
    assert_eq!(wide_char_to_multi_byte(CP_ACP,
                                       WC_COMPOSITECHECK,
                                       &[0x0054, 0x0065, 0x0073, 0x0074],
                                       None,
                                       true)
                   .unwrap(),
               (b"Test".to_vec(), false));
}

#[test]
fn wide_char_to_multi_byte_utf8() {
    assert_eq!(wide_char_to_multi_byte(CP_UTF8,
                                       WC_ERR_INVALID_CHARS,
                                       &[0x6F22],
                                       None,
                                       false)
                   .unwrap(),
               (b"\xE6\xBC\xA2".to_vec(), false));
}

#[test]
fn wide_char_to_multi_byte_replace() {
    assert_eq!(wide_char_to_multi_byte(CP_ACP,
                                       WC_DEFAULTCHAR | WC_COMPOSITECHECK,
                                       &[0x0054, 0x0065, 0x0073, 0x0074, 0x6F22, 0x0029],
                                       Some(b':'),
                                       true)
                   .unwrap(),
               (b"Test:)".to_vec(), true));
}

#[test]
fn wide_char_to_multi_byte_invalid() {
    assert_eq!(wide_char_to_multi_byte(CP_ACP,
                                       WC_COMPOSITECHECK,
                                       &[0x6F22],
                                       Some(b':'),
                                       true)
                   .unwrap(),
               (b":".to_vec(), true));
    assert_eq!(wide_char_to_multi_byte(CP_ACP,
                                       WC_COMPOSITECHECK,
                                       &[0x0020],
                                       Some(b':'),
                                       true)
                   .unwrap(),
               (b" ".to_vec(), false));
}

#[cfg(test)]
mod tests {
    extern crate winapi;

    use super::*;
    use super::super::Encoder;

    #[test]
    fn cp1251_to_string_test() {
        assert_eq!(EncoderCodePage(1251).to_string(b"\xD2\xE5\xF1\xF2").unwrap(),
                   "Тест");
    }
    #[test]
    fn string_to_cp1251_test() {
        assert_eq!(EncoderCodePage(1251).to_bytes("Тест").unwrap(),
                   b"\xD2\xE5\xF1\xF2");
    }

    #[test]
    fn cp866_to_string_test() {
        assert_eq!(EncoderCodePage(866).to_string(b"\x92\xA5\xE1\xE2").unwrap(),
                   "Тест");
    }

    #[test]
    fn string_to_cp866_test() {
        assert_eq!(EncoderCodePage(866).to_bytes("Тест").unwrap(),
                   b"\x92\xA5\xE1\xE2");
    }
}
