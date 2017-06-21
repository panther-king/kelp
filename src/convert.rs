//! Functions which convert strings.
use std::collections::HashMap;
use std::vec::Vec;

use conv_option::ConvOption;
use conv_table::HiraKana;

/// Convert from hiragana to full-witdh katakana
pub fn hira2kata(text: &str, option: ConvOption) -> String {
    convert(text, HiraKana::HiraToKana.to_table(), option)
}

/// Convert from full-width katakana to hiragana
pub fn kata2hira(text: &str, option: ConvOption) -> String {
    convert(text, HiraKana::KanaToHira.to_table(), option)
}

/// Convert strings refers conversion table and option settings
fn convert(text: &str, table: HashMap<u32, String>, option: ConvOption) -> String {
    let ignore: Vec<u32> = option.ignore_chars().into_iter().map(|c| c as u32).collect();
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
