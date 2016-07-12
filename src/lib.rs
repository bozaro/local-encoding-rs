#[cfg(windows)]
pub mod windows;
#[cfg(windows)]
pub use windows::{ansi_to_string, oem_to_string, string_to_ansi, string_to_oem};
#[cfg(not(windows))]
pub mod posix;
#[cfg(not(windows))]
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
