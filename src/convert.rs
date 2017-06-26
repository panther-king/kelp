//! Functions which convert strings.
use std::collections::HashMap;
use std::vec::Vec;

use conv_option::ConvOption;
use conv_table::HiraKana;

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
    let convert_ascii = option.convert_ascii();
    let convert_digit = option.convert_digit();
    let convert_kana = option.convert_kana();
    let conv_type = if convert_ascii {
        if convert_digit {
            if convert_kana {
                FullToHalf::All
            } else {
                FullToHalf::AsciiAndDigits
            }
        } else if convert_kana {
            FullToHalf::AsciiAndKana
        } else {
            FullToHalf::Ascii
        }
    } else if convert_digit {
        if convert_kana {
            FullToHalf::DigitsAndKana
        } else {
            FullToHalf::Digits
        }
    } else {
        FullToHalf::Kana
    };
    let table = conv_type.to_table();
    convert(text, table, option)
}

/// Convert strings refers conversion table and option settings
fn convert(text: &str, table: HashMap<u32, String>, option: ConvOption) -> String {
    let ignore: Vec<u32> = option.ignore_chars().into_iter().map(|c| *c as u32).collect();
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
    use conv_option::ConvOption;
    use conv_table::*;
    use super::*;

    #[test]
    fn test_hira2kata() {
        let before = HIRAGANA.join("");
        let after = FULL_KANA.join("");
        let option = ConvOption::build().finalize();
        assert_eq!(hira2kata(&before, option), after);
    }

    #[test]
    fn test_hira2kata_with_ignore() {
        let before = HIRAGANA.join("");
        let option = ConvOption::build().ignore(&before).finalize();
        assert_eq!(hira2kata(&before, option), before);
    }

    #[test]
    fn test_hira2hkata() {
        let before = HIRAGANA.join("");
        let after = HALF_KANA.join("");
        let option = ConvOption::build().finalize();
        assert_eq!(hira2hkata(&before, option), after);
    }

    #[test]
    fn test_hira2hkata_with_ignore() {
        let before = HIRAGANA.join("");
        let option = ConvOption::build().ignore(&before).finalize();
        assert_eq!(hira2hkata(&before, option), before);
    }

    #[test]
    fn test_kata2hira() {
        let before = FULL_KANA.join("");
        let after = HIRAGANA.join("");
        let option = ConvOption::build().finalize();
        assert_eq!(kata2hira(&before, option), after);
    }

    #[test]
    fn test_kata2hira_with_ignore() {
        let before = FULL_KANA.join("");
        let option = ConvOption::build().ignore(&before).finalize();
        assert_eq!(kata2hira(&before, option), before);
    }

    #[test]
    fn test_z2h_all() {
        let before = format!("{}{}{}",
                             FULL_ASCII.join(""),
                             FULL_DIGIT.join(""),
                             FULL_KANA.join(""));
        let after = format!("{}{}{}",
                            HALF_ASCII.join(""),
                            HALF_DIGIT.join(""),
                            HALF_KANA.join(""));
        let option = ConvOption::build().ascii(true).digit(true).kana(true).finalize();
        assert_eq!(z2h(&before, option), after);
    }

    #[test]
    fn test_z2h_ascii() {
        let before = format!("{}{}{}",
                             FULL_ASCII.join(""),
                             FULL_DIGIT.join(""),
                             FULL_KANA.join(""));
        let after = format!("{}{}{}",
                            HALF_ASCII.join(""),
                            FULL_DIGIT.join(""),
                            FULL_KANA.join(""));
        let option = ConvOption::build().ascii(true).digit(false).kana(false).finalize();
        assert_eq!(z2h(&before, option), after);
    }

    #[test]
    fn test_z2h_ascii_and_digits() {
        let before = format!("{}{}{}",
                             FULL_ASCII.join(""),
                             FULL_DIGIT.join(""),
                             FULL_KANA.join(""));
        let after = format!("{}{}{}",
                            HALF_ASCII.join(""),
                            HALF_DIGIT.join(""),
                            FULL_KANA.join(""));
        let option = ConvOption::build().ascii(true).digit(true).kana(false).finalize();
        assert_eq!(z2h(&before, option), after);
    }

    #[test]
    fn test_z2h_ascii_and_kana() {
        let before = format!("{}{}{}",
                             FULL_ASCII.join(""),
                             FULL_DIGIT.join(""),
                             FULL_KANA.join(""));
        let after = format!("{}{}{}",
                            HALF_ASCII.join(""),
                            FULL_DIGIT.join(""),
                            HALF_KANA.join(""));
        let option = ConvOption::build().ascii(true).digit(false).kana(true).finalize();
        assert_eq!(z2h(&before, option), after);
    }

    #[test]
    fn test_z2h_digits() {
        let before = format!("{}{}{}",
                             FULL_ASCII.join(""),
                             FULL_DIGIT.join(""),
                             FULL_KANA.join(""));
        let after = format!("{}{}{}",
                            FULL_ASCII.join(""),
                            HALF_DIGIT.join(""),
                            FULL_KANA.join(""));
        let option = ConvOption::build().ascii(false).digit(true).kana(false).finalize();
        assert_eq!(z2h(&before, option), after);
    }

    #[test]
    fn test_z2h_digits_and_kana() {
        let before = format!("{}{}{}",
                             FULL_ASCII.join(""),
                             FULL_DIGIT.join(""),
                             FULL_KANA.join(""));
        let after = format!("{}{}{}",
                            FULL_ASCII.join(""),
                            HALF_DIGIT.join(""),
                            HALF_KANA.join(""));
        let option = ConvOption::build().ascii(false).digit(true).kana(true).finalize();
        assert_eq!(z2h(&before, option), after);
    }

    #[test]
    fn test_z2h_kana() {
        let before = format!("{}{}{}",
                             FULL_ASCII.join(""),
                             FULL_DIGIT.join(""),
                             FULL_KANA.join(""));
        let after = format!("{}{}{}",
                            FULL_ASCII.join(""),
                            FULL_DIGIT.join(""),
                            HALF_KANA.join(""));
        let option = ConvOption::build().ascii(false).digit(false).kana(true).finalize();
        assert_eq!(z2h(&before, option), after);
    }
}