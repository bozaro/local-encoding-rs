local-encoding
====

[![Join the chat at https://gitter.im/bozaro/local-encoding-rs](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/bozaro/local-encoding-rs?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)
[![Build Status](https://travis-ci.org/bozaro/local-encoding-rs.svg?branch=master)](https://travis-ci.org/bozaro/local-encoding-rs)
[![Crates.io](https://img.shields.io/crates/v/local-encoding.svg)](https://crates.io/crates/local-encoding)

This repository contains rust library for encoding/decoding string with local charset. It usefull for work with ANSI strings on Windows.

Unfortunately Windows widly use 8-bit character encoding instead UTF-8. This causes a lot of pain.

For example, in Russian version:

 * CP-1251 (ANSI codepage) used for 8-bit files;
 * CP-866 (OEM codepage) used for console output.

To convert between 8-bit and Unicode used Windows have function: MultiByteToWideChar and WideCharToMultiByte.

This library provide simple function to convert between 8-bit and Unicode characters on Windows.

UTF-8 used as 8-bit codepage for non-Windows system.

Rustdoc: https://bozaro.github.io/local-encoding-rs/local_encoding/

## Usage

Put this in your `Cargo.toml`:

```toml
[dependencies]
local-encoding = "*"
```

For example:
```rust
extern crate local_encoding;

use local_encoding::{Encoding, Encoder};

fn main()
{
	println!("Unicode string: {}", Encoding::ANSI.to_string(b"ANSI string").unwrap());
	println!("Unicode string: {}", Encoding::OEM.to_string(b"OEM string").unwrap());
}
```
