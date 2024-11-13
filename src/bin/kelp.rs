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

use clap::Parser;
use kelp::h2z;
use kelp::hira2hkata;
use kelp::hira2kata;
use kelp::kata2hira;
use kelp::z2h;
use kelp::ConvOption;

/// A conversion tool of Japanese
#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    /// Specified conversion pattern
    #[arg(short, long)]
    conv: String,

    /// Convert with ascii if specified
    #[arg(short, long)]
    ascii: bool,

    /// Convert with digit if specified
    #[arg(short, long)]
    digit: bool,

    /// Convert with kana if specified
    #[arg(short, long)]
    kana: bool,

    /// Specified ignore charcters
    #[arg(short, long)]
    ignore: Option<String>,

    text: Option<String>,
}

fn main() {
    let args = Args::parse();
    let ignore: &'static str = match args.ignore.as_deref() {
        Some("") | None => "",
        Some(s) => Box::leak(s.to_string().into_boxed_str()),
    };
    let option = ConvOption {
        ascii: args.ascii,
        digit: args.digit,
        ignore: ignore,
        kana: args.kana,
    };
    let text = args.text.as_deref().unwrap_or("");

    let converted = match args.conv.as_str() {
        "h2z" => h2z(text, option),
        "h2hk" => hira2hkata(text, option),
        "h2k" => hira2kata(text, option),
        "k2h" => kata2hira(text, option),
        "z2h" => z2h(text, option),
        _ => text.to_string(),
    };

    println!("{}", converted);
}
