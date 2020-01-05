kelp
====

[![Build Status](https://travis-ci.org/panther-king/kelp.svg?branch=master)](https://travis-ci.org/panther-king/kelp) [![Crates.io](https://img.shields.io/crates/v/kelp.svg)](https://crates.io/crates/kelp) [![Crates.io](https://img.shields.io/crates/d/kelp.svg)](https://crates.io/crates/kelp) [![license](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/panther-king/kelp/blob/master/LICENSE)

Convert tool for Japanese.

This is a porting
from [jaconv(python)](https://github.com/ikegami-yukino/jaconv)
written in Rust.

About
=====

- Convert characters between full-width and half-width
    - From full-width to half-width
    - From half-width to full-width
- Convert characters between hiragana and katakana
    - From hiragana to katakana
    - From katakana to hiragana

Usage
=====

Add `kelp` as a dependency in your `Cargo.toml`

```toml
[dependencies]
kelp = "0.2"
```

Quick Example
=============

First, you should build `ConvOption`.

`ConvOption` has flags of conversion method.

After building `ConvOption`, you can convert characters with functions
of `kelp`.

```rust
extern crate kelp;

use kelp::*;
use kelp::conv_option::ConvOption;

fn main() {
    // All flags are disabled in default
    let option = ConvOption::build()
        .enable_ascii() // Convert ascii
        .enable_digit() // Convert digit
        .enable_kana()  // Convert kana
        .finalize();    // Returns ConvOption with specified flags

    // From hiragana to katakana(full-width)
    println!("{}", hira2kata("あいうえお", option)); // アイウエオ

    // From hiragana to katakana(half-width)
    println!("{}", hira2hkata("あいうえお", option)); // ｱｲｳｴｵ

    // From katakana(full-width) to hiragana
    println!("{}", kata2hira("アイウエオ", option)); // あいうえお

    // From half-width to full-width
    println!("{}", h2z("abc123ｱｲｳ", option)); // ＡＢＣ１２３アイウ

    // From full-width to half-width
    println!("{}", z2h("ＡＢＣ１２３アイウ", option)); // ABC123ｱｲｳ
}
```
