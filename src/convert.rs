//! Functions which convert strings.
use std::collections::HashMap;
use std::vec::Vec;

use conv_option::ConvOption;
use conv_table::HiraKana;

/// Convert from hiragana to full-witdh katakana
///
/// ```
/// use kelp::conv_option::ConvOptionBuilder;
/// use kelp::hira2kata;
///
/// let option = ConvOptionBuilder::new().build();
/// let converted = hira2kata("あいうえお", option);
/// assert_eq!("アイウエオ", converted);
///
/// let option = ConvOptionBuilder::new().ignore("かこ").build();
/// let converted = hira2kata("かきくけこ", option);
/// assert_eq!("かキクケこ", converted);
/// ```
pub fn hira2kata(text: &str, option: ConvOption) -> String {
    let table = HiraKana::HiraToKana.to_table();
    convert(text, table, option)
}

/// Convert from hiragana to half-width katakana
///
/// ```
/// use kelp::conv_option::ConvOptionBuilder;
/// use kelp::hira2hkata;
///
/// let option = ConvOptionBuilder::new().build();
/// let converted = hira2hkata("あいうえお", option);
/// assert_eq!("ｱｲｳｴｵ", converted);
///
/// let option = ConvOptionBuilder::new().ignore("がご").build();
/// let converted = hira2hkata("がぎぐげご", option);
/// assert_eq!("がｷﾞｸﾞｹﾞご", converted);
/// ```
pub fn hira2hkata(text: &str, option: ConvOption) -> String {
    let table = HiraKana::HiraToHalfKana.to_table();
    convert(text, table, option)
}

/// Convert from full-width katakana to hiragana
///
/// ```
/// use kelp::conv_option::ConvOptionBuilder;
/// use kelp::kata2hira;
///
/// let option = ConvOptionBuilder::new().build();
/// let converted = kata2hira("アイウエオ", option);
/// assert_eq!("あいうえお", converted);
///
/// let option = ConvOptionBuilder::new().ignore("キクケ").build();
/// let converted = kata2hira("カキクケコ", option);
/// assert_eq!("かキクケこ", converted);
/// ```
pub fn kata2hira(text: &str, option: ConvOption) -> String {
    let table = HiraKana::KanaToHira.to_table();
    convert(text, table, option)
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

    fn half_kana() -> &'static str {
        "ｧｱｨｲｩｳｪｴｫｵｶｶﾞｷｷﾞｸｸﾞｹｹﾞｺｺﾞｻｻﾞｼｼﾞｽｽﾞｾｾﾞｿｿﾞ\
         ﾀﾀﾞﾁﾁﾞｯﾂﾂﾞﾃﾃﾞﾄﾄﾞﾅﾆﾇﾈﾉﾊﾊﾞﾊﾟﾋﾋﾞﾋﾟﾌﾌﾞﾌﾟﾍﾍﾞ\
         ﾍﾟﾎﾎﾞﾎﾟﾏﾐﾑﾒﾓｬﾔｭﾕｮﾖﾗﾘﾙﾚﾛﾜｦﾝｰヮヰヱヵヶｳﾞ\
         ヽヾ･｢｣｡､"
    }

    fn hiragana() -> &'static str {
        "ぁあぃいぅうぇえぉおかがきぎくぐけげこごさざしじすずせぜそぞ\
         ただちぢっつづてでとどなにぬねのはばぱひびぴふぶぷへべぺほぼ\
         ぽまみむめもゃやゅゆょよらりるれろわをんーゎゐゑゕゖゔゝゞ・\
         「」。、"
    }

    fn full_kana() -> &'static str {
        "ァアィイゥウェエォオカガキギクグケゲコゴサザシジスズセゼソゾ\
         タダチヂッツヅテデトドナニヌネノハバパヒビピフブプヘベペホボ\
         ポマミムメモャヤュユョヨラリルレロワヲンーヮヰヱヵヶヴヽヾ・\
         「」。、"
    }

    #[test]
    fn test_hira2kata() {
        let before = hiragana();
        let after = full_kana();
        let option = ConvOptionBuilder::new().build();
        assert_eq!(hira2kata(before, option), after);
    }

    #[test]
    fn test_hira2kata_with_ignore() {
        let before = hiragana();
        let option = ConvOptionBuilder::new().ignore(before).build();
        assert_eq!(hira2kata(before, option), before);
    }

    #[test]
    fn test_hira2hkata() {
        let before = hiragana();
        let after = half_kana();
        let option = ConvOptionBuilder::new().build();
        assert_eq!(hira2hkata(before, option), after);
    }

    #[test]
    fn test_hira2hkata_with_ignore() {
        let before = hiragana();
        let option = ConvOptionBuilder::new().ignore(before).build();
        assert_eq!(hira2hkata(before, option), before);
    }

    #[test]
    fn test_kata2hira() {
        let before = full_kana();
        let after = hiragana();
        let option = ConvOptionBuilder::new().build();
        assert_eq!(kata2hira(before, option), after);
    }

    #[test]
    fn test_kata2hira_with_ignore() {
        let before = full_kana();
        let option = ConvOptionBuilder::new().ignore(before).build();
        assert_eq!(kata2hira(before, option), before);
    }
}
