extern crate winapi;
extern crate kernel32;

use std::ptr;
use std::io::{Error, ErrorKind, Result};
use self::winapi::DWORD;

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
