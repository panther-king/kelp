//! Tables for conversion
use std::collections::HashMap;
use std::vec::Vec;

use self::Method::*;
use self::Target::*;
use crate::ConvOption;

/// ASCII(full-width)
pub(crate) const FULL_ASCII: [&str; 85] = [
    "！", "＂", "＃", "＄", "％", "＆", "＇", "（", "）", "＊", "＋", "，", "－", "．", "／", "：",
    "；", "＜", "＝", "＞", "？", "＠", "Ａ", "Ｂ", "Ｃ", "Ｄ", "Ｅ", "Ｆ", "Ｇ", "Ｈ", "Ｉ", "Ｊ",
    "Ｋ", "Ｌ", "Ｍ", "Ｎ", "Ｏ", "Ｐ", "Ｑ", "Ｒ", "Ｓ", "Ｔ", "Ｕ", "Ｖ", "Ｗ", "Ｘ", "Ｙ", "Ｚ",
    "［", "＼", "］", "＾", "＿", "｀", "ａ", "ｂ", "ｃ", "ｄ", "ｅ", "ｆ", "ｇ", "ｈ", "ｉ", "ｊ",
    "ｋ", "ｌ", "ｍ", "ｎ", "ｏ", "ｐ", "ｑ", "ｒ", "ｓ", "ｔ", "ｕ", "ｖ", "ｗ", "ｘ", "ｙ", "ｚ",
    "｛", "｜", "｝", "～", "　",
];

/// ASCII(half-width)
pub(crate) const HALF_ASCII: [&str; 85] = [
    "!", "\"", "#", "$", "%", "&", "'", "(", ")", "*", "+", ",", "-", ".", "/", ":", ";", "<", "=",
    ">", "?", "@", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P",
    "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "[", "\\", "]", "^", "_", "`", "a", "b", "c",
    "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v",
    "w", "x", "y", "z", "{", "|", "}", "~", " ",
];

/// Digits(full-width)
pub(crate) const FULL_DIGIT: [&str; 10] =
    ["０", "１", "２", "３", "４", "５", "６", "７", "８", "９"];

/// Digits(half-width)
pub(crate) const HALF_DIGIT: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

/// Hiragana(only full-width)
pub(crate) const HIRAGANA: [&str; 94] = [
    "ぁ", "あ", "ぃ", "い", "ぅ", "う", "ぇ", "え", "ぉ", "お", "か", "が", "き", "ぎ", "く", "ぐ",
    "け", "げ", "こ", "ご", "さ", "ざ", "し", "じ", "す", "ず", "せ", "ぜ", "そ", "ぞ", "た", "だ",
    "ち", "ぢ", "っ", "つ", "づ", "て", "で", "と", "ど", "な", "に", "ぬ", "ね", "の", "は", "ば",
    "ぱ", "ひ", "び", "ぴ", "ふ", "ぶ", "ぷ", "へ", "べ", "ぺ", "ほ", "ぼ", "ぽ", "ま", "み", "む",
    "め", "も", "ゃ", "や", "ゅ", "ゆ", "ょ", "よ", "ら", "り", "る", "れ", "ろ", "わ", "を", "ん",
    "ー", "ゎ", "ゐ", "ゑ", "ゕ", "ゖ", "ゔ", "ゝ", "ゞ", "・", "「", "」", "。", "、",
];

/// Katakana(full-width)
pub(crate) const FULL_KANA: [&str; 94] = [
    "ァ", "ア", "ィ", "イ", "ゥ", "ウ", "ェ", "エ", "ォ", "オ", "カ", "ガ", "キ", "ギ", "ク", "グ",
    "ケ", "ゲ", "コ", "ゴ", "サ", "ザ", "シ", "ジ", "ス", "ズ", "セ", "ゼ", "ソ", "ゾ", "タ", "ダ",
    "チ", "ヂ", "ッ", "ツ", "ヅ", "テ", "デ", "ト", "ド", "ナ", "ニ", "ヌ", "ネ", "ノ", "ハ", "バ",
    "パ", "ヒ", "ビ", "ピ", "フ", "ブ", "プ", "ヘ", "ベ", "ペ", "ホ", "ボ", "ポ", "マ", "ミ", "ム",
    "メ", "モ", "ャ", "ヤ", "ュ", "ユ", "ョ", "ヨ", "ラ", "リ", "ル", "レ", "ロ", "ワ", "ヲ", "ン",
    "ー", "ヮ", "ヰ", "ヱ", "ヵ", "ヶ", "ヴ", "ヽ", "ヾ", "・", "「", "」", "。", "、",
];

/// Katakana(half-width)
pub(crate) const HALF_KANA: [&str; 94] = [
    "ｧ", "ｱ", "ｨ", "ｲ", "ｩ", "ｳ", "ｪ", "ｴ", "ｫ", "ｵ", "ｶ", "ｶﾞ", "ｷ", "ｷﾞ", "ｸ", "ｸﾞ", "ｹ", "ｹﾞ",
    "ｺ", "ｺﾞ", "ｻ", "ｻﾞ", "ｼ", "ｼﾞ", "ｽ", "ｽﾞ", "ｾ", "ｾﾞ", "ｿ", "ｿﾞ", "ﾀ", "ﾀﾞ", "ﾁ", "ﾁﾞ", "ｯ",
    "ﾂ", "ﾂﾞ", "ﾃ", "ﾃﾞ", "ﾄ", "ﾄﾞ", "ﾅ", "ﾆ", "ﾇ", "ﾈ", "ﾉ", "ﾊ", "ﾊﾞ", "ﾊﾟ", "ﾋ", "ﾋﾞ", "ﾋﾟ",
    "ﾌ", "ﾌﾞ", "ﾌﾟ", "ﾍ", "ﾍﾞ", "ﾍﾟ", "ﾎ", "ﾎﾞ", "ﾎﾟ", "ﾏ", "ﾐ", "ﾑ", "ﾒ", "ﾓ", "ｬ", "ﾔ", "ｭ", "ﾕ",
    "ｮ", "ﾖ", "ﾗ", "ﾘ", "ﾙ", "ﾚ", "ﾛ", "ﾜ", "ｦ", "ﾝ", "ｰ", "ヮ", "ヰ", "ヱ", "ヵ", "ヶ", "ｳﾞ",
    "ヽ", "ヾ", "･", "｢", "｣", "｡", "､",
];

/// Katakana(full-width, no voiced consonant marks)
pub(crate) const FULL_KANA_SEION: [&str; 68] = [
    "ァ", "ア", "ィ", "イ", "ゥ", "ウ", "ェ", "エ", "ォ", "オ", "カ", "キ", "ク", "ケ", "コ", "サ",
    "シ", "ス", "セ", "ソ", "タ", "チ", "ッ", "ツ", "テ", "ト", "ナ", "ニ", "ヌ", "ネ", "ノ", "ハ",
    "ヒ", "フ", "ヘ", "ホ", "マ", "ミ", "ム", "メ", "モ", "ャ", "ヤ", "ュ", "ユ", "ョ", "ヨ", "ラ",
    "リ", "ル", "レ", "ロ", "ワ", "ヲ", "ン", "ー", "ヮ", "ヰ", "ヱ", "ヵ", "ヶ", "ヽ", "ヾ", "・",
    "「", "」", "。", "、",
];

/// Katakana(half-width, no voiced consonant marks)
pub(crate) const HALF_KANA_SEION: [&str; 68] = [
    "ｧ", "ｱ", "ｨ", "ｲ", "ｩ", "ｳ", "ｪ", "ｴ", "ｫ", "ｵ", "ｶ", "ｷ", "ｸ", "ｹ", "ｺ", "ｻ", "ｼ", "ｽ", "ｾ",
    "ｿ", "ﾀ", "ﾁ", "ｯ", "ﾂ", "ﾃ", "ﾄ", "ﾅ", "ﾆ", "ﾇ", "ﾈ", "ﾉ", "ﾊ", "ﾋ", "ﾌ", "ﾍ", "ﾎ", "ﾏ", "ﾐ",
    "ﾑ", "ﾒ", "ﾓ", "ｬ", "ﾔ", "ｭ", "ﾕ", "ｮ", "ﾖ", "ﾗ", "ﾘ", "ﾙ", "ﾚ", "ﾛ", "ﾜ", "ｦ", "ﾝ", "ｰ", "ヮ",
    "ヰ", "ヱ", "ヵ", "ヶ", "ヽ", "ヾ", "･", "｢", "｣", "｡", "､",
];

/// A table that convert from katakana(half-width) to katakana(full-width)
/// with voiced consonant marks
pub(crate) const MAP_KANA: [(&str, &str); 26] = [
    ("ｶﾞ", "ガ"),
    ("ｷﾞ", "ギ"),
    ("ｸﾞ", "グ"),
    ("ｹﾞ", "ゲ"),
    ("ｺﾞ", "ゴ"),
    ("ｻﾞ", "ザ"),
    ("ｼﾞ", "ジ"),
    ("ｽﾞ", "ズ"),
    ("ｾﾞ", "ゼ"),
    ("ｿﾞ", "ゾ"),
    ("ﾀﾞ", "ダ"),
    ("ﾁﾞ", "ヂ"),
    ("ﾂﾞ", "ヅ"),
    ("ﾃﾞ", "デ"),
    ("ﾄﾞ", "ド"),
    ("ﾊﾞ", "バ"),
    ("ﾋﾞ", "ビ"),
    ("ﾌﾞ", "ブ"),
    ("ﾍﾞ", "ベ"),
    ("ﾎﾞ", "ボ"),
    ("ﾊﾟ", "パ"),
    ("ﾋﾟ", "ピ"),
    ("ﾌﾟ", "プ"),
    ("ﾍﾟ", "ペ"),
    ("ﾎﾟ", "ポ"),
    ("ｳﾞ", "ヴ"),
];

#[derive(Debug)]
pub(crate) enum Method {
    /// From full-width to half-width
    FullToHalf(Target),
    /// From half-width to full-width
    HalfToFull(Target),
    /// From hiragana to katakana(half-width)
    HiraToHalfKana,
    /// From hiragana to katakana(full-width)
    HiraToKana,
    /// From katakana(full-width) to hiragana
    KanaToHira,
}

impl Method {
    pub fn table(&self) -> HashMap<u32, String> {
        let pair = match self {
            FullToHalf(target) => match target {
                All => (
                    [&FULL_ASCII[..], &FULL_DIGIT[..], &FULL_KANA[..]].concat(),
                    [&HALF_ASCII[..], &HALF_DIGIT[..], &HALF_KANA[..]].concat(),
                ),
                Ascii => ([&FULL_ASCII[..]].concat(), [&HALF_ASCII[..]].concat()),
                AsciiAndDigits => (
                    [&FULL_ASCII[..], &FULL_DIGIT[..]].concat(),
                    [&HALF_ASCII[..], &HALF_DIGIT[..]].concat(),
                ),
                AsciiAndKana => (
                    [&FULL_ASCII[..], &FULL_KANA[..]].concat(),
                    [&HALF_ASCII[..], &HALF_KANA[..]].concat(),
                ),
                Digits => ([&FULL_DIGIT[..]].concat(), [&HALF_DIGIT[..]].concat()),
                DigitsAndKana => (
                    [&FULL_DIGIT[..], &FULL_KANA[..]].concat(),
                    [&HALF_DIGIT[..], &HALF_KANA[..]].concat(),
                ),
                Kana => ([&FULL_KANA[..]].concat(), [&HALF_KANA[..]].concat()),
            },
            HalfToFull(target) => match target {
                All => (
                    [&HALF_ASCII[..], &HALF_DIGIT[..], &HALF_KANA_SEION[..]].concat(),
                    [&FULL_ASCII[..], &FULL_DIGIT[..], &FULL_KANA_SEION[..]].concat(),
                ),
                Ascii => ([&HALF_ASCII[..]].concat(), [&FULL_ASCII[..]].concat()),
                AsciiAndDigits => (
                    [&HALF_ASCII[..], &HALF_DIGIT[..]].concat(),
                    [&FULL_ASCII[..], &FULL_DIGIT[..]].concat(),
                ),
                AsciiAndKana => (
                    [&HALF_ASCII[..], &HALF_KANA_SEION[..]].concat(),
                    [&FULL_ASCII[..], &FULL_KANA_SEION[..]].concat(),
                ),
                Digits => ([&HALF_DIGIT[..]].concat(), [&FULL_DIGIT[..]].concat()),
                DigitsAndKana => (
                    [&HALF_DIGIT[..], &HALF_KANA_SEION[..]].concat(),
                    [&FULL_DIGIT[..], &FULL_KANA_SEION[..]].concat(),
                ),
                Kana => (
                    [&HALF_KANA_SEION[..]].concat(),
                    [&FULL_KANA_SEION[..]].concat(),
                ),
            },
            HiraToHalfKana => ([&HIRAGANA[..]].concat(), [&HALF_KANA[..]].concat()),
            HiraToKana => ([&HIRAGANA[..]].concat(), [&FULL_KANA[..]].concat()),
            KanaToHira => ([&FULL_KANA[..]].concat(), [&HIRAGANA[..]].concat()),
        };

        assert!(pair.0.len() == pair.1.len());
        let keys = pair
            .0
            .join("")
            .chars()
            .map(|c| c as u32)
            .collect::<Vec<u32>>();
        keys.into_iter()
            .zip(pair.1.into_iter())
            .map(|(k, v)| (k, v.to_string()))
            .collect()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Target {
    /// Ascii, digits and katakana
    All,
    /// Only ascii
    Ascii,
    /// Ascii and digits
    AsciiAndDigits,
    /// Ascii and katakana
    AsciiAndKana,
    /// Only digits
    Digits,
    /// Digits and katakana
    DigitsAndKana,
    /// Only katakana
    Kana,
}

impl From<&ConvOption<'_>> for Target {
    fn from(option: &ConvOption) -> Target {
        match (option.ascii, option.digit, option.kana) {
            (true, true, true) => All,
            (true, true, false) => AsciiAndDigits,
            (true, false, true) => AsciiAndKana,
            (true, false, false) => Ascii,
            (false, true, true) => DigitsAndKana,
            (false, true, false) => Digits,
            _ => Kana,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ConvOption;

    #[test]
    fn test_target_all() {
        let option = ConvOption {
            ascii: true,
            digit: true,
            kana: true,
            ..Default::default()
        };
        assert_eq!(Target::from(&option), Target::All);
    }

    #[test]
    fn test_target_ascii_only() {
        let option = ConvOption {
            ascii: true,
            ..Default::default()
        };
        assert_eq!(Target::from(&option), Target::Ascii);
    }

    #[test]
    fn test_target_ascii_and_digits() {
        let option = ConvOption {
            ascii: true,
            digit: true,
            ..Default::default()
        };
        assert_eq!(Target::from(&option), Target::AsciiAndDigits);
    }

    #[test]
    fn test_target_ascii_and_kana() {
        let option = ConvOption {
            ascii: true,
            kana: true,
            ..Default::default()
        };
        assert_eq!(Target::from(&option), Target::AsciiAndKana);
    }

    #[test]
    fn test_target_digits_only() {
        let option = ConvOption {
            digit: true,
            ..Default::default()
        };
        assert_eq!(Target::from(&option), Target::Digits);
    }

    #[test]
    fn test_target_digits_and_kana() {
        let option = ConvOption {
            digit: true,
            kana: true,
            ..Default::default()
        };
        assert_eq!(Target::from(&option), Target::DigitsAndKana);
    }

    #[test]
    fn test_target_kana_only() {
        let option = ConvOption {
            ..Default::default()
        };
        assert_eq!(Target::from(&option), Target::Kana);
    }

    #[test]
    fn test_full_to_half_all() {
        let table = Method::FullToHalf(Target::All).table();
        assert_eq!(189, table.len());
        assert_eq!(table.get(&65313).unwrap(), "A");
        assert_eq!(table.get(&65296).unwrap(), "0");
        assert_eq!(table.get(&12460).unwrap(), "ｶﾞ");
    }

    #[test]
    fn test_full_to_half_ascii() {
        let table = Method::FullToHalf(Target::Ascii).table();
        assert_eq!(85, table.len());
        assert_eq!(table.get(&65314).unwrap(), "B");
        assert_eq!(table.get(&65297), None);
        assert_eq!(table.get(&12461), None);
    }

    #[test]
    fn test_full_to_half_ascii_and_digits() {
        let table = Method::FullToHalf(Target::AsciiAndDigits).table();
        assert_eq!(95, table.len());
        assert_eq!(table.get(&65315).unwrap(), "C");
        assert_eq!(table.get(&65298).unwrap(), "2");
        assert_eq!(table.get(&12462), None);
    }

    #[test]
    fn test_full_to_half_ascii_and_kana() {
        let table = Method::FullToHalf(Target::AsciiAndKana).table();
        assert_eq!(179, table.len());
        assert_eq!(table.get(&65316).unwrap(), "D");
        assert_eq!(table.get(&65299), None);
        assert_eq!(table.get(&12463).unwrap(), "ｸ");
    }

    #[test]
    fn test_full_to_half_digits() {
        let table = Method::FullToHalf(Target::Digits).table();
        assert_eq!(10, table.len());
        assert_eq!(table.get(&65317), None);
        assert_eq!(table.get(&65300).unwrap(), "4");
        assert_eq!(table.get(&12464), None)
    }

    #[test]
    fn test_full_to_half_digits_and_kana() {
        let table = Method::FullToHalf(Target::DigitsAndKana).table();
        assert_eq!(104, table.len());
        assert_eq!(table.get(&65318), None);
        assert_eq!(table.get(&65301).unwrap(), "5");
        assert_eq!(table.get(&12465).unwrap(), "ｹ");
    }

    #[test]
    fn test_full_to_half_kana() {
        let table = Method::FullToHalf(Target::Kana).table();
        assert_eq!(94, table.len());
        assert_eq!(table.get(&65319), None);
        assert_eq!(table.get(&65302), None);
        assert_eq!(table.get(&12466).unwrap(), "ｹﾞ");
    }

    #[test]
    fn test_half_to_full_all() {
        let table = Method::HalfToFull(Target::All).table();
        assert_eq!(163, table.len());
        assert_eq!(table.get(&97).unwrap(), "ａ");
        assert_eq!(table.get(&48).unwrap(), "０");
        assert_eq!(table.get(&65393).unwrap(), "ア");
    }

    #[test]
    fn test_half_to_full_ascii() {
        let table = Method::HalfToFull(Target::Ascii).table();
        assert_eq!(85, table.len());
        assert_eq!(table.get(&98).unwrap(), "ｂ");
        assert_eq!(table.get(&49), None);
        assert_eq!(table.get(&65394), None);
    }

    #[test]
    fn test_half_to_full_ascii_and_digits() {
        let table = Method::HalfToFull(Target::AsciiAndDigits).table();
        assert_eq!(95, table.len());
        assert_eq!(table.get(&99).unwrap(), "ｃ");
        assert_eq!(table.get(&50).unwrap(), "２");
        assert_eq!(table.get(&65395), None);
    }

    #[test]
    fn test_half_to_full_ascii_and_kana() {
        let table = Method::HalfToFull(Target::AsciiAndKana).table();
        assert_eq!(153, table.len());
        assert_eq!(table.get(&100).unwrap(), "ｄ");
        assert_eq!(table.get(&51), None);
        assert_eq!(table.get(&65396).unwrap(), "エ");
    }

    #[test]
    fn test_half_to_full_digits() {
        let table = Method::HalfToFull(Target::Digits).table();
        assert_eq!(10, table.len());
        assert_eq!(table.get(&101), None);
        assert_eq!(table.get(&52).unwrap(), "４");
        assert_eq!(table.get(&65397), None);
    }

    #[test]
    fn test_half_to_full_digits_and_kana() {
        let table = Method::HalfToFull(Target::DigitsAndKana).table();
        assert_eq!(78, table.len());
        assert_eq!(table.get(&102), None);
        assert_eq!(table.get(&53).unwrap(), "５");
        assert_eq!(table.get(&65398).unwrap(), "カ");
    }

    #[test]
    fn test_half_to_full_kana() {
        let table = Method::HalfToFull(Target::Kana).table();
        assert_eq!(68, table.len());
        assert_eq!(table.get(&103), None);
        assert_eq!(table.get(&54), None);
        assert_eq!(table.get(&65399).unwrap(), "キ");
    }

    #[test]
    fn test_hiara_kana_hira_to_half_kana() {
        let table = Method::HiraToHalfKana.table();
        assert_eq!(94, table.len());
        assert_eq!(table.get(&12354).unwrap(), "ｱ");
    }

    #[test]
    fn test_hira_kana_hira_to_kana() {
        let table = Method::HiraToKana.table();
        assert_eq!(94, table.len());
        assert_eq!(table.get(&12355).unwrap(), "ィ");
    }

    #[test]
    fn test_hira_kana_kana_to_hira() {
        let table = Method::KanaToHira.table();
        assert_eq!(94, table.len());
        assert_eq!(table.get(&12531).unwrap(), "ん");
    }
}
