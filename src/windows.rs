extern crate winapi;
extern crate kernel32;

use std::ptr;
use std::io::{Error, ErrorKind, Result};
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use self::winapi::{BOOL, DWORD};

pub const MB_PRECOMPOSED: DWORD = 0x00000001;
pub const MB_COMPOSITE: DWORD = 0x00000002;
pub const MB_USEGLYPHCHARS: DWORD = 0x00000004;
pub const MB_ERR_INVALID_CHARS: DWORD = 0x00000008;
pub const WC_COMPOSITECHECK: DWORD = 0x00000200;
pub const WC_DISCARDNS: DWORD = 0x00000010;
pub const WC_SEPCHARS: DWORD = 0x00000020;
pub const WC_DEFAULTCHAR: DWORD = 0x00000040;
pub const WC_ERR_INVALID_CHARS: DWORD = 0x00000080;
pub const WC_NO_BEST_FIT_CHARS: DWORD = 0x00000400;

pub fn oem_to_string(data: &[u8]) -> Result<String> {
    multi_byte_to_wide_char(winapi::CP_OEMCP, MB_ERR_INVALID_CHARS, data)
}

pub fn ansi_to_string(data: &[u8]) -> Result<String> {
    multi_byte_to_wide_char(winapi::CP_ACP, MB_ERR_INVALID_CHARS, data)
}

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

pub fn string_to_oem(data: &str) -> Result<Vec<u8>> {
    string_to_multibyte(winapi::CP_OEMCP, data, None)
}

pub fn string_to_ansi(data: &str) -> Result<Vec<u8>> {
    string_to_multibyte(winapi::CP_ACP, data, None)
}

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
        let len = kernel32::MultiByteToWideChar(codepage,
                                                flags,
                                                multi_byte_str.as_ptr() as winapi::LPSTR,
                                                multi_byte_str.len() as i32,
                                                ptr::null_mut(),
                                                0);
        if len > 0 {
            // Convert to UTF-16
            let mut wstr: Vec<u16> = Vec::with_capacity(len as usize);
            wstr.set_len(len as usize);
            let len = kernel32::MultiByteToWideChar(codepage,
                                                    flags,
                                                    multi_byte_str.as_ptr() as winapi::LPSTR,
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
        let len = kernel32::WideCharToMultiByte(codepage,
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
            let len = kernel32::WideCharToMultiByte(codepage,
                                                    flags,
                                                    wide_char_str.as_ptr(),
                                                    wide_char_str.len() as i32,
                                                    astr.as_mut_ptr() as winapi::LPSTR,
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
    assert_eq!(multi_byte_to_wide_char(winapi::CP_ACP, MB_ERR_INVALID_CHARS, b"").unwrap(),
               "");
}

#[test]
fn multi_byte_to_wide_char_ascii() {
    assert_eq!(multi_byte_to_wide_char(winapi::CP_ACP, MB_ERR_INVALID_CHARS, b"Test").unwrap(),
               "Test");
}

#[test]
fn multi_byte_to_wide_char_utf8() {
    assert_eq!(multi_byte_to_wide_char(winapi::CP_UTF8,
                                       MB_ERR_INVALID_CHARS,
                                       b"\xD0\xA2\xD0\xB5\xD1\x81\xD1\x82")
                   .unwrap(),
               "Тест");
}

#[test]
fn multi_byte_to_wide_char_invalid() {
    assert!(multi_byte_to_wide_char(winapi::CP_UTF8, MB_ERR_INVALID_CHARS, b"Test\xC0").is_err());
}

#[test]
fn wide_char_to_multi_byte_empty() {
    assert_eq!(wide_char_to_multi_byte(winapi::CP_UTF8, WC_ERR_INVALID_CHARS, &[], None, false)
                   .unwrap(),
               (b"".to_vec(), false));
}

#[test]
fn wide_char_to_multi_byte_ascii() {
    assert_eq!(wide_char_to_multi_byte(winapi::CP_ACP,
                                       WC_COMPOSITECHECK,
                                       &[0x0054, 0x0065, 0x0073, 0x0074],
                                       None,
                                       true)
                   .unwrap(),
               (b"Test".to_vec(), false));
}

#[test]
fn wide_char_to_multi_byte_utf8() {
    assert_eq!(wide_char_to_multi_byte(winapi::CP_UTF8,
                                       WC_ERR_INVALID_CHARS,
                                       &[0x6F22],
                                       None,
                                       false)
                   .unwrap(),
               (b"\xE6\xBC\xA2".to_vec(), false));
}

#[test]
fn wide_char_to_multi_byte_replace() {
    assert_eq!(wide_char_to_multi_byte(winapi::CP_ACP,
                                       WC_DEFAULTCHAR | WC_COMPOSITECHECK,
                                       &[0x0054, 0x0065, 0x0073, 0x0074, 0x6F22, 0x0029],
                                       Some(b':'),
                                       true)
                   .unwrap(),
               (b"Test:)".to_vec(), true));
}

#[test]
fn wide_char_to_multi_byte_invalid() {
    assert_eq!(wide_char_to_multi_byte(winapi::CP_ACP,
                                       WC_COMPOSITECHECK,
                                       &[0x6F22],
                                       Some(b':'),
                                       true)
                   .unwrap(),
               (b":".to_vec(), true));
    assert_eq!(wide_char_to_multi_byte(winapi::CP_ACP,
                                       WC_COMPOSITECHECK,
                                       &[0x0020],
                                       Some(b':'),
                                       true)
                   .unwrap(),
               (b" ".to_vec(), false));
}

#[test]
fn oem_to_string_test() {
    assert_eq!(oem_to_string(b"Test").unwrap(), "Test");
}

#[test]
fn ansi_to_string_test() {
    assert_eq!(ansi_to_string(b"Test").unwrap(), "Test");
}
