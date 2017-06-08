//! Functions which convert strings.
use std::collections::HashMap;
use std::vec::Vec;

use conv_option::ConvOption;
use conv_table::{ConvTable, HiraKana, exclude_table};

/// Convert from hiragana to full-witdh katakana
pub fn hira2kata(text: &str, option: ConvOption) -> String {
    let table = match option.ignore_chars() {
        Some(chars) => exclude_table(chars, HiraKana::HiraToKana),
        None => HiraKana::HiraToKana.table(),
    };
    convert(text, table)
}

/// Convert from full-width katakana to hiragana
pub fn kata2hira(text: &str, option: ConvOption) -> String {
    let table = match option.ignore_chars() {
        Some(chars) => exclude_table(chars, HiraKana::KanaToHira),
        None => HiraKana::KanaToHira.table(),
    };
    convert(text, table)
}

/// Convert strings with conversion table
fn convert(text: &str, table: HashMap<u32, String>) -> String {
    let mut converted = Vec::new();

    for c in text.chars() {
        let ord = c as u32;
        match table.get(&ord) {
            Some(s) => converted.push(s.to_string()),
            None => converted.push(c.to_string()),
        }
    }

    converted.into_iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    use conv_option::ConvOptionBuilder;
    use super::*;

    #[test]
    fn test_hira2kata() {
        let option = ConvOptionBuilder::new().build();
        assert_eq!(hira2kata("あいうえお", option), "アイウエオ");
    }

    #[test]
    fn test_hira2kata_with_ignore() {
        let option = ConvOptionBuilder::new().ignore("きくけ").build();
        assert_eq!(hira2kata("かきくけこ", option), "カきくけコ");
    }

    #[test]
    fn test_kata2hira() {
        let option = ConvOptionBuilder::new().build();
        assert_eq!(kata2hira("アイウエオ", option), "あいうえお");
    }

    #[test]
    fn test_kata2hira_with_ignore() {
        let option = ConvOptionBuilder::new().ignore("サソ").build();
        assert_eq!(kata2hira("サシスセソ", option), "サしすせソ");
    }
}
