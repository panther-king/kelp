//! # kelp-cli
//!
//! A command-line tool with kelp crate.
//!
//! ## Examples
//!
//! Convert from half-width to full-width only ascii.
//!
//! ```sh
//! kelp-cli -a -c h2z ABC
//! #=> ＡＢＣ
//! ```
//!
//! Convert from full-width to half-width only digit.
//!
//! ```sh
//! kelp-cli -d -c z2h １２３
//! #=> 123
//! ```
//!
//! Convert from half-width to full-width only kana.
//!
//! ```sh
//! kelp-cli -k -c h2z ｱｲｳ
//! #=> アイウ
//! ```
//!
//! ## Options
//!
//! - -c, --conv
//!     - Specified conversion pattern
//!
//! Option | Pattern
//! ------ | -------
//! h2z | half-width to full-width
//! h2hk | half-width to half-width(katakana)
//! h2k | half-width to full-width(katakana)
//! k2h | full-width(katakana) to half-width(katakana)
//! z2h | full-width to half-width
//!
//! - -a, --ascii
//!     - Convert with ascii if specified
//! - -d, --digit
//!     - Convert with digit if specified
//! - -k, --kana
//!     - Convert with kana if specified
//! - -i, --ignore
//!     - Specified ignore characters
//!     - e.g. `-i A1ｱ`
//!
extern crate clap;
extern crate kelp;

use clap::{App, Arg};
use kelp::h2z;
use kelp::hira2hkata;
use kelp::hira2kata;
use kelp::kata2hira;
use kelp::z2h;
use kelp::conv_option::ConvOption;

fn main() {
    let matches = App::new("kelp")
        .version("0.1")
        .author("Taro Yamashita <taro.ymst@gmail.com>")
        .about("A conversion tool of Japanese")
        .arg(Arg::with_name("INPUT")
            .required(true)
            .help("A text that you want to convert"))
        .arg(Arg::with_name("convert")
            .short("c")
            .long("conv")
            .takes_value(true)
            .help("Specified conversion pattern"))
        .arg(Arg::with_name("ascii")
            .short("a")
            .long("ascii")
            .help("Convert with ascii if specified"))
        .arg(Arg::with_name("digit")
            .short("d")
            .long("digit")
            .help("Convert with digit if specified"))
        .arg(Arg::with_name("kana")
            .short("k")
            .long("kana")
            .help("Convert with kana if specified"))
        .arg(Arg::with_name("ignore")
            .short("i")
            .long("ignore")
            .takes_value(true)
            .help("Specified ignore characters"))
        .get_matches();

    let option = ConvOption::build()
        .ascii(matches.is_present("ascii"))
        .digit(matches.is_present("digit"))
        .kana(matches.is_present("digit"))
        .ignore(matches.value_of("ignore").unwrap_or(""))
        .finalize();

    let text = matches.value_of("INPUT").unwrap();

    let converted = match matches.value_of("convert") {
        Some(s) if s == "h2z" => h2z(text, option),
        Some(s) if s == "h2hk" => hira2hkata(text, option),
        Some(s) if s == "h2k" => hira2kata(text, option),
        Some(s) if s == "k2h" => kata2hira(text, option),
        Some(s) if s == "z2h" => z2h(text, option),
        _ => text.to_string(),
    };

    println!("{}", converted);
}
