//! Tables of convert.
use std::collections::HashMap;
use std::vec::Vec;

/// ASCII(full-width)
const FULL_ASCII: &str = "\
    ！＂＃＄％＆＇（）＊＋，－．／：；＜＝＞？＠\
    ＡＢＣＤＥＦＧＨＩＪＫＬＭＮＯＰＱＲＳＴＵＶＷＸＹＺ\
    ［＼］＾＿｀\
    ａｂｃｄｅｆｇｈｉｊｋｌｍｎｏｐｑｒｓｔｕｖｗｘｙｚ\
    ｛｜｝～　";

/// ASCII(half-width)
const HALF_ASCII: &str = "\
    !\"#$%&'()*+,-./:;<=>?@\
    ABCDEFGHIJKLMNOPQRSTUVWXYZ\
    [\\]^_`\
    abcdefghijklmnopqrstuvwxyz\
    {|}~ ";

/// Digits(full-width)
const FULL_DIGIT: &str = "０１２３４５６７８９";

/// Digits(half-width)
const HALF_DIGIT: &str = "0123456789";

/// Hiragana(only full-width)
const HIRAGANA: &str = "\
    ぁあぃいぅうぇえぉお\
    かがきぎくぐけげこご\
    さざしじすずせぜそぞ\
    ただちぢっつづてでとど\
    なにぬねの\
    はばぱひびぴふぶぷへべぺほぼぽ\
    まみむめも\
    ゃやゅゆょよ\
    らりるれろ\
    わをん\
    ーゎゐゑゕゖゔゝゞ・「」。、";

/// Katakana(full-width)
const FULL_KANA: &str = "\
    ァアィイゥウェエォオ\
    カガキギクグケゲコゴ\
    サザシジスズセゼソゾ\
    タダチヂッツヅテデトド\
    ナニヌネノ\
    ハバパヒビピフブプヘベペホボポ\
    マミムメモ\
    ャヤュユョヨ\
    ラリルレロ\
    ワヲン\
    ーヮヰヱヵヶヴヽヾ・「」。、";

/// Katakana(half-width)
const HALF_KANA: &str = "\
    ｧｱｨｲｩｳｪｴｫｵ\
    ｶｶﾞｷｷﾞｸｸﾞｹｹﾞｺｺﾞ\
    ｻｻﾞｼｼﾞｽｽﾞｾｾﾞｿｿﾞ\
    ﾀﾀﾞﾁﾁﾞｯﾂﾂﾞﾃﾃﾞﾄﾄﾞ\
    ﾅﾆﾇﾈﾉ\
    ﾊﾊﾞﾊﾟﾋﾋﾞﾋﾟﾌﾌﾞﾌﾟﾍﾍﾞﾍﾟﾎﾎﾞﾎﾟ\
    ﾏﾐﾑﾒﾓ\
    ﾔｬﾕｭﾖｮ\
    ﾗﾘﾙﾚﾛ\
    ﾜｦﾝ\
    ｰヮヰヱヵヶｳﾞヽヾ･｢｣｡､";

/// Katakana(full-width, no voiced consonant marks)
const FULL_KANA_SEION: &str = "\
    ァアィイゥウェエォオ\
    カキクケコ\
    サシスセソ\
    タチッツテト\
    ナニヌネノ\
    ハヒフヘホ\
    マミムメモ\
    ャヤュユョヨ\
    ラリルレロ\
    ワヲン\
    ーヮヰヱヵヶヽヾ・「」。、";

/// Katakana(half-width, no voiced consonant marks)
const HALF_KANA_SEION: &str = "\
    ｧｱｨｲｩｳｪｴｫｵ\
    ｶｷｸｹｺ\
    ｻｼｽｾｿ\
    ﾀﾁｯﾂﾃﾄ\
    ﾅﾆﾇﾈﾉ\
    ﾊﾋﾌﾍﾎ\
    ﾏﾐﾑﾒﾓ\
    ｬﾔｭﾕｮﾖ\
    ﾗﾘﾙﾚﾛ\
    ﾜｦﾝ\
    ｰヮヰヱヵヶヽヾ･｢｣｡､";

/// Roman letters
const HEPBURN: &str = "aiueoaiueon";

/// Hiragana corresponding to Roman letters
const HEPBURN_KANA: &str = "ぁぃぅぇぉあいうえおん";

pub trait ConvTable {
    /// Generate a convert table
    fn table(&self) -> HashMap<u32, String>;
}

/// Generate a convert table.
///
/// HashMap's key is a code-point of character.
/// HashMap's value is a string after conversion.
fn conv_table(key: &str, value: &str) -> HashMap<u32, String> {
    let keys: Vec<u32> = key.chars().map(|c| c as u32).collect();
    let values: Vec<String> = value.chars().map(|s| s.to_string()).collect();
    let mut table = HashMap::new();

    for (k, v) in keys.into_iter().zip(values.into_iter()) {
        table.insert(k, v);
    }

    table
}

/// Convert from full-width to half-width
pub enum FullToHalf {
    /// ASCII, digits and katakana
    All,
    /// Only ASCII
    Ascii,
    /// ASCII and digits
    AsciiAndDigits,
    /// ASCII and katakana
    AsciiAndKana,
    /// Only digits
    Digits,
    /// Digits and katakana
    DigitsAndKana,
    /// Only katakana
    Kana,
}

impl ConvTable for FullToHalf {
    /// Generate a convert table
    /// which convert from full-width to half-width
    fn table(&self) -> HashMap<u32, String> {
        match *self {
            FullToHalf::All => {
                conv_table(
                    &format!("{}{}{}", FULL_ASCII, FULL_DIGIT, FULL_KANA),
                    &format!("{}{}{}", HALF_ASCII, HALF_DIGIT, HALF_KANA)
                )
            },
            FullToHalf::Ascii => {
                conv_table(&FULL_ASCII, &HALF_ASCII)
            },
            FullToHalf::AsciiAndDigits => {
                conv_table(
                    &format!("{}{}", FULL_ASCII, FULL_DIGIT),
                    &format!("{}{}", HALF_ASCII, HALF_DIGIT)
                )
            },
            FullToHalf::AsciiAndKana => {
                conv_table(
                    &format!("{}{}", FULL_ASCII, FULL_KANA),
                    &format!("{}{}", HALF_ASCII, HALF_KANA)
                )
            },
            FullToHalf::Digits => {
                conv_table(&FULL_DIGIT, &HALF_DIGIT)
            },
            FullToHalf::DigitsAndKana => {
                conv_table(
                    &format!("{}{}", FULL_DIGIT, FULL_KANA),
                    &format!("{}{}", HALF_DIGIT, HALF_KANA)
                )
            },
            FullToHalf::Kana => {
                conv_table(&FULL_KANA, &HALF_KANA)
            },
        }
    }
}

/// Convert from half-width to full-width
pub enum HalfToFull {
    /// ASCII, digits and katakana
    All,
    /// Only ASCII
    Ascii,
    /// ASCII and digits
    AsciiAndDigits,
    /// ASCII and katakana
    AsciiAndKana,
    /// Only digits
    Digits,
    /// Digits and katakana
    DigitsAndKana,
    /// Only katakana
    Kana,
}

impl ConvTable for HalfToFull {
    /// Generate a convert table
    /// which convert from half-width to full-width
    fn table(&self) -> HashMap<u32, String> {
        match *self {
            HalfToFull::All => {
                conv_table(
                    &format!("{}{}{}", HALF_ASCII, HALF_DIGIT, HALF_KANA_SEION),
                    &format!("{}{}{}", FULL_ASCII, FULL_DIGIT, FULL_KANA_SEION)
                )
            },
            HalfToFull::Ascii => {
                conv_table(&HALF_ASCII, &FULL_ASCII)
            },
            HalfToFull::AsciiAndDigits => {
                conv_table(
                    &format!("{}{}", HALF_ASCII, HALF_DIGIT),
                    &format!("{}{}", FULL_ASCII, FULL_DIGIT)
                )
            },
            HalfToFull::AsciiAndKana => {
                conv_table(
                    &format!("{}{}", HALF_ASCII, HALF_KANA_SEION),
                    &format!("{}{}", FULL_ASCII, FULL_KANA_SEION)
                )
            },
            HalfToFull::Digits => {
                conv_table(&HALF_DIGIT, &FULL_DIGIT)
            },
            HalfToFull::DigitsAndKana => {
                conv_table(
                    &format!("{}{}", HALF_DIGIT, HALF_KANA_SEION),
                    &format!("{}{}", FULL_DIGIT, FULL_KANA_SEION)
                )
            },
            HalfToFull::Kana => {
                conv_table(&HALF_KANA_SEION, &FULL_KANA_SEION)
            }
        }
    }
}

/// Convert between hiragana and katakana
pub enum HiraKana {
    /// From hiragana to half-width katakana
    HiraToHalfKana,
    /// From hiragana to full-width katakana
    HiraToKana,
    /// 全角カタカナをひらがなへ変換
    /// From full-width katakana to hiragana
    KanaToHira,
}

impl ConvTable for HiraKana {
    /// Generate a convert table
    /// which convert between hiragana and katakana
    fn table(&self) -> HashMap<u32, String> {
        match *self {
            HiraKana::HiraToHalfKana => conv_table(&HIRAGANA, &HALF_KANA),
            HiraKana::HiraToKana => conv_table(&HIRAGANA, &FULL_KANA),
            HiraKana::KanaToHira => conv_table(&FULL_KANA, &HIRAGANA),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii() {
        assert_eq!(FULL_ASCII.chars().count(), HALF_ASCII.chars().count());
    }

    #[test]
    fn test_digit() {
        assert_eq!(FULL_DIGIT.chars().count(), HALF_DIGIT.chars().count());
    }

    #[test]
    fn test_hira_kana() {
        assert_eq!(HIRAGANA.chars().count(), FULL_KANA.chars().count());
    }

    #[test]
    fn test_seion() {
        assert_eq!(FULL_KANA_SEION.chars().count(), HALF_KANA_SEION.chars().count());
    }

    #[test]
    fn test_half_kana() {
        assert_eq!(HALF_KANA.chars().count(), 120);
    }
}
