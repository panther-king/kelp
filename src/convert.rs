//! Functions which convert strings.
use std::collections::HashMap;
use std::vec::Vec;

use crate::conv_table::{Method, Target, MAP_KANA};
use crate::ConvOption;

/// Convert from hiragana to full-witdh katakana
///
/// # Example
///
/// ```rust
/// use kelp::ConvOption;
/// use kelp::hira2kata;
///
/// let option = ConvOption {
///     ..Default::default()
/// };
/// let converted = hira2kata("あいうえお", option);
/// assert_eq!("アイウエオ", converted);
///
/// let option = ConvOption {
///     ignore: "かこ",
///     ..Default::default()
/// };
/// let converted = hira2kata("かきくけこ", option);
/// assert_eq!("かキクケこ", converted);
/// ```
pub fn hira2kata(text: &str, option: ConvOption) -> String {
    let method = Method::HiraToKana;
    convert(text, method.table(), &option.ignore)
}

/// Convert from hiragana to half-width katakana
///
/// # Example
///
/// ```rust
/// use kelp::ConvOption;
/// use kelp::hira2hkata;
///
/// let option = ConvOption {
///     ..Default::default()
/// };
/// let converted = hira2hkata("あいうえお", option);
/// assert_eq!("ｱｲｳｴｵ", converted);
///
/// let option = ConvOption {
///     ignore: "がご",
///     ..Default::default()
/// };
/// let converted = hira2hkata("がぎぐげご", option);
/// assert_eq!("がｷﾞｸﾞｹﾞご", converted);
/// ```
pub fn hira2hkata(text: &str, option: ConvOption) -> String {
    let method = Method::HiraToHalfKana;
    convert(text, method.table(), &option.ignore)
}

/// Convert from full-width katakana to hiragana
///
/// # Example
///
/// ```rust
/// use kelp::ConvOption;
/// use kelp::kata2hira;
///
/// let option = ConvOption {
///     ..Default::default()
/// };
/// let converted = kata2hira("アイウエオ", option);
/// assert_eq!("あいうえお", converted);
///
/// let option = ConvOption {
///     ignore: "キクケ",
///     ..Default::default()
/// };
/// let converted = kata2hira("カキクケコ", option);
/// assert_eq!("かキクケこ", converted);
/// ```
pub fn kata2hira(text: &str, option: ConvOption) -> String {
    let method = Method::KanaToHira;
    convert(text, method.table(), &option.ignore)
}

/// Convert from half-width to full-width
///
/// # Example
///
/// ```rust
/// use kelp::ConvOption;
/// use kelp::h2z;
///
/// let option = ConvOption {
///     ascii: true,
///     digit: true,
///     kana: true,
///     ..Default::default()
/// };
/// let converted = h2z("ABCｱｲｳ012", option);
/// assert_eq!("ＡＢＣアイウ０１２", converted);
///
/// let option = ConvOption {
///     ascii: true,
///     digit: true,
///     kana: true,
///     ignore: "Aｱ0",
/// };
/// let converted = h2z("ABCｱｲｳ012", option);
/// assert_eq!("AＢＣｱイウ0１２", converted);
/// ```
pub fn h2z(text: &str, option: ConvOption) -> String {
    let method = Method::HalfToFull(Target::from(&option));
    if option.kana {
        convert(
            &before_convert(text, MAP_KANA.to_vec()),
            method.table(),
            &option.ignore,
        )
    } else {
        convert(text, method.table(), &option.ignore)
    }
}

/// Convert from full-width to half-width
///
/// # Example
///
/// ```rust
/// use kelp::ConvOption;
/// use kelp::z2h;
///
/// let option = ConvOption {
///     ascii: true,
///     digit: true,
///     kana: true,
///     ..Default::default()
/// };
/// let converted = z2h("ＡＢＣアイウ０１２", option);
/// assert_eq!("ABCｱｲｳ012", converted);
///
/// let option = ConvOption {
///     ascii: true,
///     digit: true,
///     ignore: "Ａア０",
///     kana: true,
/// };
/// let converted = z2h("ＡＢＣアイウ０１２", option);
/// assert_eq!("ＡBCアｲｳ０12", converted);
/// ```
pub fn z2h(text: &str, option: ConvOption) -> String {
    let method = Method::FullToHalf(Target::from(&option));
    convert(text, method.table(), &option.ignore)
}

/// Replace strings before convert
fn before_convert(text: &str, convert: Vec<(&str, &str)>) -> String {
    let mut converted = text.to_string();
    convert
        .iter()
        .for_each(|(b, a)| converted = converted.replace(b, a));
    converted
}

/// Convert strings refers conversion table and option settings
fn convert(text: &str, table: HashMap<u32, String>, ignore: &str) -> String {
    let ignore = ignore.chars().map(|c| c as u32).collect::<Vec<_>>();

    text.chars()
        .map(|c| {
            let ord = c as u32;
            match table.get(&ord) {
                Some(s) if !ignore.contains(&ord) => s.to_string(),
                _ => c.to_string(),
            }
        })
        .collect::<Vec<String>>()
        .join("")
}

#[cfg(test)]
mod tests {
    use super::*;
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
        let option = ConvOption {
            ..Default::default()
        };
        assert_eq!(hira2kata(&before, option), after);
    }

    #[test]
    fn test_hira2kata_with_ignore() {
        let before = strings!(HIRAGANA);
        let option = ConvOption {
            ignore: &before,
            ..Default::default()
        };
        assert_eq!(hira2kata(&before, option), before);
    }

    #[test]
    fn test_hira2hkata() {
        let before = strings!(HIRAGANA);
        let after = strings!(HALF_KANA);
        let option = ConvOption {
            ..Default::default()
        };
        assert_eq!(hira2hkata(&before, option), after);
    }

    #[test]
    fn test_hira2hkata_with_ignore() {
        let before = strings!(HIRAGANA);
        let option = ConvOption {
            ignore: &before,
            ..Default::default()
        };
        assert_eq!(hira2hkata(&before, option), before);
    }

    #[test]
    fn test_kata2hira() {
        let before = strings!(FULL_KANA);
        let after = strings!(HIRAGANA);
        let option = ConvOption {
            ..Default::default()
        };
        assert_eq!(kata2hira(&before, option), after);
    }

    #[test]
    fn test_kata2hira_with_ignore() {
        let before = strings!(FULL_KANA);
        let option = ConvOption {
            ignore: &before,
            ..Default::default()
        };
        assert_eq!(kata2hira(&before, option), before);
    }

    #[test]
    fn test_h2z_all() {
        let before = strings!(HALF_ASCII, HALF_DIGIT, HALF_KANA);
        let after = strings!(FULL_ASCII, FULL_DIGIT, FULL_KANA);
        let option = ConvOption {
            ascii: true,
            digit: true,
            kana: true,
            ..Default::default()
        };
        assert_eq!(h2z(&before, option), after);
    }

    #[test]
    fn test_h2z_ascii() {
        let before = strings!(HALF_ASCII, HALF_DIGIT, HALF_KANA);
        let after = strings!(FULL_ASCII, HALF_DIGIT, HALF_KANA);
        let option = ConvOption {
            ascii: true,
            ..Default::default()
        };
        assert_eq!(h2z(&before, option), after);
    }

    #[test]
    fn test_h2z_ascii_and_digits() {
        let before = strings!(HALF_ASCII, HALF_DIGIT, HALF_KANA);
        let after = strings!(FULL_ASCII, FULL_DIGIT, HALF_KANA);
        let option = ConvOption {
            ascii: true,
            digit: true,
            ..Default::default()
        };
        assert_eq!(h2z(&before, option), after);
    }

    #[test]
    fn test_h2z_ascii_and_kana() {
        let before = strings!(HALF_ASCII, HALF_DIGIT, HALF_KANA);
        let after = strings!(FULL_ASCII, HALF_DIGIT, FULL_KANA);
        let option = ConvOption {
            ascii: true,
            kana: true,
            ..Default::default()
        };
        assert_eq!(h2z(&before, option), after);
    }

    #[test]
    fn test_h2z_digits() {
        let before = strings!(HALF_ASCII, HALF_DIGIT, HALF_KANA);
        let after = strings!(HALF_ASCII, FULL_DIGIT, HALF_KANA);
        let option = ConvOption {
            digit: true,
            ..Default::default()
        };
        assert_eq!(h2z(&before, option), after);
    }

    #[test]
    fn test_h2z_digits_and_kana() {
        let before = strings!(HALF_ASCII, HALF_DIGIT, HALF_KANA);
        let after = strings!(HALF_ASCII, FULL_DIGIT, FULL_KANA);
        let option = ConvOption {
            digit: true,
            kana: true,
            ..Default::default()
        };
        assert_eq!(h2z(&before, option), after);
    }

    #[test]
    fn test_h2z_kana() {
        let before = strings!(HALF_ASCII, HALF_DIGIT, HALF_KANA);
        let after = strings!(HALF_ASCII, HALF_DIGIT, FULL_KANA);
        let option = ConvOption {
            kana: true,
            ..Default::default()
        };
        assert_eq!(h2z(&before, option), after);
    }

    #[test]
    fn test_z2h_all() {
        let before = strings!(FULL_ASCII, FULL_DIGIT, FULL_KANA);
        let after = strings!(HALF_ASCII, HALF_DIGIT, HALF_KANA);
        let option = ConvOption {
            ascii: true,
            digit: true,
            kana: true,
            ..Default::default()
        };
        assert_eq!(z2h(&before, option), after);
    }

    #[test]
    fn test_z2h_ascii() {
        let before = strings!(FULL_ASCII, FULL_DIGIT, FULL_KANA);
        let after = strings!(HALF_ASCII, FULL_DIGIT, FULL_KANA);
        let option = ConvOption {
            ascii: true,
            ..Default::default()
        };
        assert_eq!(z2h(&before, option), after);
    }

    #[test]
    fn test_z2h_ascii_and_digits() {
        let before = strings!(FULL_ASCII, FULL_DIGIT, FULL_KANA);
        let after = strings!(HALF_ASCII, HALF_DIGIT, FULL_KANA);
        let option = ConvOption {
            ascii: true,
            digit: true,
            ..Default::default()
        };
        assert_eq!(z2h(&before, option), after);
    }

    #[test]
    fn test_z2h_ascii_and_kana() {
        let before = strings!(FULL_ASCII, FULL_DIGIT, FULL_KANA);
        let after = strings!(HALF_ASCII, FULL_DIGIT, HALF_KANA);
        let option = ConvOption {
            ascii: true,
            kana: true,
            ..Default::default()
        };
        assert_eq!(z2h(&before, option), after);
    }

    #[test]
    fn test_z2h_digits() {
        let before = strings!(FULL_ASCII, FULL_DIGIT, FULL_KANA);
        let after = strings!(FULL_ASCII, HALF_DIGIT, FULL_KANA);
        let option = ConvOption {
            digit: true,
            ..Default::default()
        };
        assert_eq!(z2h(&before, option), after);
    }

    #[test]
    fn test_z2h_digits_and_kana() {
        let before = strings!(FULL_ASCII, FULL_DIGIT, FULL_KANA);
        let after = strings!(FULL_ASCII, HALF_DIGIT, HALF_KANA);
        let option = ConvOption {
            digit: true,
            kana: true,
            ..Default::default()
        };
        assert_eq!(z2h(&before, option), after);
    }

    #[test]
    fn test_z2h_kana() {
        let before = strings!(FULL_ASCII, FULL_DIGIT, FULL_KANA);
        let after = strings!(FULL_ASCII, FULL_DIGIT, HALF_KANA);
        let option = ConvOption {
            kana: true,
            ..Default::default()
        };
        assert_eq!(z2h(&before, option), after);
    }
}
