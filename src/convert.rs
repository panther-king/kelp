//! Functions which convert strings.
use std::collections::HashMap;
use std::vec::Vec;

use crate::conv_option::ConvOption;
use crate::conv_table::{FullToHalf, HalfToFull, HiraKana, MAP_KANA};

/// Convert from hiragana to full-witdh katakana
///
/// # Example
///
/// ```rust
/// use kelp::conv_option::ConvOption;
/// use kelp::hira2kata;
///
/// let option = ConvOption::build().finalize();
/// let converted = hira2kata("あいうえお", option);
/// assert_eq!("アイウエオ", converted);
///
/// let option = ConvOption::build().ignore("かこ").finalize();
/// let converted = hira2kata("かきくけこ", option);
/// assert_eq!("かキクケこ", converted);
/// ```
pub fn hira2kata(text: &str, option: ConvOption) -> String {
    let table = HiraKana::HiraToKana.to_table();
    convert(text, table, option)
}

/// Convert from hiragana to half-width katakana
///
/// # Example
///
/// ```rust
/// use kelp::conv_option::ConvOption;
/// use kelp::hira2hkata;
///
/// let option = ConvOption::build().finalize();
/// let converted = hira2hkata("あいうえお", option);
/// assert_eq!("ｱｲｳｴｵ", converted);
///
/// let option = ConvOption::build().ignore("がご").finalize();
/// let converted = hira2hkata("がぎぐげご", option);
/// assert_eq!("がｷﾞｸﾞｹﾞご", converted);
/// ```
pub fn hira2hkata(text: &str, option: ConvOption) -> String {
    let table = HiraKana::HiraToHalfKana.to_table();
    convert(text, table, option)
}

/// Convert from full-width katakana to hiragana
///
/// # Example
///
/// ```rust
/// use kelp::conv_option::ConvOption;
/// use kelp::kata2hira;
///
/// let option = ConvOption::build().finalize();
/// let converted = kata2hira("アイウエオ", option);
/// assert_eq!("あいうえお", converted);
///
/// let option = ConvOption::build().ignore("キクケ").finalize();
/// let converted = kata2hira("カキクケコ", option);
/// assert_eq!("かキクケこ", converted);
/// ```
pub fn kata2hira(text: &str, option: ConvOption) -> String {
    let table = HiraKana::KanaToHira.to_table();
    convert(text, table, option)
}

/// Convert from half-width to full-width
///
/// # Example
///
/// ```rust
/// use kelp::conv_option::ConvOption;
/// use kelp::h2z;
///
/// let option = ConvOption::build()
///     .ascii(true)
///     .digit(true)
///     .kana(true)
///     .finalize();
/// let converted = h2z("ABCｱｲｳ012", option);
/// assert_eq!("ＡＢＣアイウ０１２", converted);
///
/// let option = ConvOption::build()
///     .ascii(true)
///     .digit(true)
///     .kana(true)
///     .ignore("Aｱ0")
///     .finalize();
/// let converted = h2z("ABCｱｲｳ012", option);
/// assert_eq!("AＢＣｱイウ0１２", converted);
/// ```
pub fn h2z(text: &str, option: ConvOption) -> String {
    let ascii = option.convert_ascii();
    let digit = option.convert_digit();
    let kana = option.convert_kana();
    let conv_type = match (ascii, digit, kana) {
        (true, true, true) => HalfToFull::All,
        (true, true, false) => HalfToFull::AsciiAndDigits,
        (true, false, true) => HalfToFull::AsciiAndKana,
        (true, false, false) => HalfToFull::Ascii,
        (false, true, true) => HalfToFull::DigitsAndKana,
        (false, true, false) => HalfToFull::Digits,
        _ => HalfToFull::Kana,
    };
    let table = conv_type.to_table();

    if kana {
        convert(&before_convert(text, MAP_KANA.to_vec()), table, option)
    } else {
        convert(text, table, option)
    }
}

/// Convert from full-width to half-width
///
/// # Example
///
/// ```rust
/// use kelp::conv_option::ConvOption;
/// use kelp::z2h;
///
/// let option = ConvOption::build()
///     .ascii(true)
///     .digit(true)
///     .kana(true)
///     .finalize();
/// let converted = z2h("ＡＢＣアイウ０１２", option);
/// assert_eq!("ABCｱｲｳ012", converted);
///
/// let option = ConvOption::build()
///     .ascii(true)
///     .digit(true)
///     .kana(true)
///     .ignore("Ａア０")
///     .finalize();
/// let converted = z2h("ＡＢＣアイウ０１２", option);
/// assert_eq!("ＡBCアｲｳ０12", converted);
/// ```
pub fn z2h(text: &str, option: ConvOption) -> String {
    let ascii = option.convert_ascii();
    let digit = option.convert_digit();
    let kana = option.convert_kana();
    let conv_type = match (ascii, digit, kana) {
        (true, true, true) => FullToHalf::All,
        (true, true, false) => FullToHalf::AsciiAndDigits,
        (true, false, true) => FullToHalf::AsciiAndKana,
        (true, false, false) => FullToHalf::Ascii,
        (false, true, true) => FullToHalf::DigitsAndKana,
        (false, true, false) => FullToHalf::Digits,
        _ => FullToHalf::Kana,
    };
    let table = conv_type.to_table();

    convert(text, table, option)
}

/// Replace strings before convert
fn before_convert(text: &str, convert: Vec<(&str, &str)>) -> String {
    let mut converted = text.to_string();
    for (before, after) in convert.into_iter() {
        converted = converted.replace(before, after);
    }
    converted
}

/// Convert strings refers conversion table and option settings
fn convert(text: &str, table: HashMap<u32, String>, option: ConvOption) -> String {
    let ignore = option
        .ignore_chars()
        .into_iter()
        .map(|c| *c as u32)
        .collect::<Vec<u32>>();
    let mut converted = Vec::new();

    for c in text.chars() {
        let ord = c as u32;
        match table.get(&ord) {
            Some(s) if !ignore.contains(&ord) => converted.push(s.to_string()),
            _ => converted.push(c.to_string()),
        }
    }

    converted.join("").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::conv_option::ConvOption;
    use crate::conv_table::*;

    macro_rules! strings {
        ($($x:expr), *) => {{
            let mut concat = String::new();
            $(
                concat = format!("{}{}", concat, $x.join(""));
            )*
            concat
        }};
    }

    #[test]
    fn test_hira2kata() {
        let before = strings!(HIRAGANA);
        let after = strings!(FULL_KANA);
        let option = ConvOption::build().finalize();
        assert_eq!(hira2kata(&before, option), after);
    }

    #[test]
    fn test_hira2kata_with_ignore() {
        let before = strings!(HIRAGANA);
        let option = ConvOption::build().ignore(&before).finalize();
        assert_eq!(hira2kata(&before, option), before);
    }

    #[test]
    fn test_hira2hkata() {
        let before = strings!(HIRAGANA);
        let after = strings!(HALF_KANA);
        let option = ConvOption::build().finalize();
        assert_eq!(hira2hkata(&before, option), after);
    }

    #[test]
    fn test_hira2hkata_with_ignore() {
        let before = strings!(HIRAGANA);
        let option = ConvOption::build().ignore(&before).finalize();
        assert_eq!(hira2hkata(&before, option), before);
    }

    #[test]
    fn test_kata2hira() {
        let before = strings!(FULL_KANA);
        let after = strings!(HIRAGANA);
        let option = ConvOption::build().finalize();
        assert_eq!(kata2hira(&before, option), after);
    }

    #[test]
    fn test_kata2hira_with_ignore() {
        let before = strings!(FULL_KANA);
        let option = ConvOption::build().ignore(&before).finalize();
        assert_eq!(kata2hira(&before, option), before);
    }

    #[test]
    fn test_h2z_all() {
        let before = strings!(HALF_ASCII, HALF_DIGIT, HALF_KANA);
        let after = strings!(FULL_ASCII, FULL_DIGIT, FULL_KANA);
        let option = ConvOption::build()
            .ascii(true)
            .digit(true)
            .kana(true)
            .finalize();
        assert_eq!(h2z(&before, option), after);
    }

    #[test]
    fn test_h2z_ascii() {
        let before = strings!(HALF_ASCII, HALF_DIGIT, HALF_KANA);
        let after = strings!(FULL_ASCII, HALF_DIGIT, HALF_KANA);
        let option = ConvOption::build()
            .ascii(true)
            .digit(false)
            .kana(false)
            .finalize();
        assert_eq!(h2z(&before, option), after);
    }

    #[test]
    fn test_h2z_ascii_and_digits() {
        let before = strings!(HALF_ASCII, HALF_DIGIT, HALF_KANA);
        let after = strings!(FULL_ASCII, FULL_DIGIT, HALF_KANA);
        let option = ConvOption::build()
            .ascii(true)
            .digit(true)
            .kana(false)
            .finalize();
        assert_eq!(h2z(&before, option), after);
    }

    #[test]
    fn test_h2z_ascii_and_kana() {
        let before = strings!(HALF_ASCII, HALF_DIGIT, HALF_KANA);
        let after = strings!(FULL_ASCII, HALF_DIGIT, FULL_KANA);
        let option = ConvOption::build()
            .ascii(true)
            .digit(false)
            .kana(true)
            .finalize();
        assert_eq!(h2z(&before, option), after);
    }

    #[test]
    fn test_h2z_digits() {
        let before = strings!(HALF_ASCII, HALF_DIGIT, HALF_KANA);
        let after = strings!(HALF_ASCII, FULL_DIGIT, HALF_KANA);
        let option = ConvOption::build()
            .ascii(false)
            .digit(true)
            .kana(false)
            .finalize();
        assert_eq!(h2z(&before, option), after);
    }

    #[test]
    fn test_h2z_digits_and_kana() {
        let before = strings!(HALF_ASCII, HALF_DIGIT, HALF_KANA);
        let after = strings!(HALF_ASCII, FULL_DIGIT, FULL_KANA);
        let option = ConvOption::build()
            .ascii(false)
            .digit(true)
            .kana(true)
            .finalize();
        assert_eq!(h2z(&before, option), after);
    }

    #[test]
    fn test_h2z_kana() {
        let before = strings!(HALF_ASCII, HALF_DIGIT, HALF_KANA);
        let after = strings!(HALF_ASCII, HALF_DIGIT, FULL_KANA);
        let option = ConvOption::build()
            .ascii(false)
            .digit(false)
            .kana(true)
            .finalize();
        assert_eq!(h2z(&before, option), after);
    }

    #[test]
    fn test_z2h_all() {
        let before = strings!(FULL_ASCII, FULL_DIGIT, FULL_KANA);
        let after = strings!(HALF_ASCII, HALF_DIGIT, HALF_KANA);
        let option = ConvOption::build()
            .ascii(true)
            .digit(true)
            .kana(true)
            .finalize();
        assert_eq!(z2h(&before, option), after);
    }

    #[test]
    fn test_z2h_ascii() {
        let before = strings!(FULL_ASCII, FULL_DIGIT, FULL_KANA);
        let after = strings!(HALF_ASCII, FULL_DIGIT, FULL_KANA);
        let option = ConvOption::build()
            .ascii(true)
            .digit(false)
            .kana(false)
            .finalize();
        assert_eq!(z2h(&before, option), after);
    }

    #[test]
    fn test_z2h_ascii_and_digits() {
        let before = strings!(FULL_ASCII, FULL_DIGIT, FULL_KANA);
        let after = strings!(HALF_ASCII, HALF_DIGIT, FULL_KANA);
        let option = ConvOption::build()
            .ascii(true)
            .digit(true)
            .kana(false)
            .finalize();
        assert_eq!(z2h(&before, option), after);
    }

    #[test]
    fn test_z2h_ascii_and_kana() {
        let before = strings!(FULL_ASCII, FULL_DIGIT, FULL_KANA);
        let after = strings!(HALF_ASCII, FULL_DIGIT, HALF_KANA);
        let option = ConvOption::build()
            .ascii(true)
            .digit(false)
            .kana(true)
            .finalize();
        assert_eq!(z2h(&before, option), after);
    }

    #[test]
    fn test_z2h_digits() {
        let before = strings!(FULL_ASCII, FULL_DIGIT, FULL_KANA);
        let after = strings!(FULL_ASCII, HALF_DIGIT, FULL_KANA);
        let option = ConvOption::build()
            .ascii(false)
            .digit(true)
            .kana(false)
            .finalize();
        assert_eq!(z2h(&before, option), after);
    }

    #[test]
    fn test_z2h_digits_and_kana() {
        let before = strings!(FULL_ASCII, FULL_DIGIT, FULL_KANA);
        let after = strings!(FULL_ASCII, HALF_DIGIT, HALF_KANA);
        let option = ConvOption::build()
            .ascii(false)
            .digit(true)
            .kana(true)
            .finalize();
        assert_eq!(z2h(&before, option), after);
    }

    #[test]
    fn test_z2h_kana() {
        let before = strings!(FULL_ASCII, FULL_DIGIT, FULL_KANA);
        let after = strings!(FULL_ASCII, FULL_DIGIT, HALF_KANA);
        let option = ConvOption::build()
            .ascii(false)
            .digit(false)
            .kana(true)
            .finalize();
        assert_eq!(z2h(&before, option), after);
    }
}
