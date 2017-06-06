//! Options of converting
use std::vec::Vec;

/// Convert option
pub struct ConvOption<'a> {
    ascii: bool,
    digit: bool,
    kana: bool,
    ignore: &'a str,
}

impl<'a> ConvOption<'a> {
    /// Whether convert with ascii or not
    pub fn convert_ascii(&self) -> bool {
        self.ascii
    }

    /// Whether convert with digit or not
    pub fn convert_digit(&self) -> bool {
        self.digit
    }

    /// Whether convert with kana or not
    pub fn convert_kana(&self) -> bool {
        self.kana
    }

    /// Ignore characters when convert
    pub fn ignore_chars(&self) -> Option<Vec<char>> {
        if self.ignore == "" {
            return None;
        }

        Some(self.ignore.chars().collect())
    }
}

/// Builder of ConvOption
pub struct ConvOptionBuilder<'a> {
    ascii: bool,
    digit: bool,
    ignore: &'a str,
    kana: bool,
}

impl<'a> ConvOptionBuilder<'a> {
    pub fn new() -> Self {
        ConvOptionBuilder {
            ascii: false,
            digit: false,
            ignore: "",
            kana: true,
        }
    }

    /// Set a flag of ascii
    pub fn ascii(&self, ascii: bool) -> Self {
        ConvOptionBuilder { ascii: ascii, ..*self }
    }

    /// Build a ConvOption
    pub fn build(&self) -> ConvOption<'a> {
        ConvOption {
            ascii: self.ascii,
            digit: self.digit,
            ignore: self.ignore,
            kana: self.kana,
        }
    }

    /// Set a flag of digit
    pub fn digit(&self, digit: bool) -> Self {
        ConvOptionBuilder { digit: digit, ..*self }
    }

    /// Set ignore characters
    pub fn ignore(&self, ignore: &'a str) -> Self {
        ConvOptionBuilder { ignore: ignore, ..*self }
    }

    /// Set a flag of kana
    pub fn kana(&self, kana: bool) -> Self {
        ConvOptionBuilder { kana: kana, ..*self }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii() {
        let option = ConvOptionBuilder::new().build();
        assert_eq!(false, option.convert_ascii());

        let option = ConvOptionBuilder::new().ascii(true).build();
        assert_eq!(true, option.convert_ascii());
    }

    #[test]
    fn test_digit() {
        let option = ConvOptionBuilder::new().build();
        assert_eq!(false, option.convert_digit());

        let option = ConvOptionBuilder::new().digit(true).build();
        assert_eq!(true, option.convert_digit());
    }

    #[test]
    fn test_kana() {
        let option = ConvOptionBuilder::new().build();
        assert_eq!(true, option.convert_kana());

        let option = ConvOptionBuilder::new().kana(false).build();
        assert_eq!(false, option.convert_kana());
    }

    #[test]
    fn test_ignore() {
        let option = ConvOptionBuilder::new().build();
        assert!(option.ignore_chars().is_none());

        let option = ConvOptionBuilder::new().ignore("ignore").build();
        assert!(option.ignore_chars().is_some());
    }
}
