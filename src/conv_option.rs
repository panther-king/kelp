//! Options of converting
use std::vec::Vec;

/// Convert option
#[derive(Debug)]
pub struct ConvOption {
    ascii: bool,
    digit: bool,
    ignore: Vec<char>,
    kana: bool,
}

impl ConvOption {
    /// Returns a builder of `ConvOption`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use kelp::conv_option::ConvOption;
    ///
    /// let option = ConvOption::build().finalize();
    /// assert!(!option.convert_ascii());
    /// assert!(!option.convert_digit());
    /// assert!(!option.convert_kana());
    /// assert_eq!(0, option.ignore_chars().len());
    /// ```
    pub fn build() -> ConvOptionBuilder {
        ConvOptionBuilder::new()
    }

    /// Whether convert with ascii or not.
    ///
    /// # Example
    ///
    /// ```rust
    /// use kelp::conv_option::ConvOption;
    ///
    /// let option = ConvOption::build().ascii(true).finalize();
    /// assert!(option.convert_ascii());
    ///
    /// let option = ConvOption::build().ascii(false).finalize();
    /// assert!(!option.convert_ascii());
    /// ```
    pub fn convert_ascii(&self) -> bool {
        self.ascii
    }

    /// Whether convert with digit or not.
    ///
    /// # Example
    ///
    /// ```rust
    /// use kelp::conv_option::ConvOption;
    ///
    /// let option = ConvOption::build().digit(true).finalize();
    /// assert!(option.convert_digit());
    ///
    /// let option = ConvOption::build().digit(false).finalize();
    /// assert!(!option.convert_digit());
    /// ```
    pub fn convert_digit(&self) -> bool {
        self.digit
    }

    /// Whether convert with kana or not.
    ///
    /// # Example
    ///
    /// ```rust
    /// use kelp::conv_option::ConvOption;
    ///
    /// let option = ConvOption::build().kana(true).finalize();
    /// assert!(option.convert_kana());
    ///
    /// let option = ConvOption::build().kana(false).finalize();
    /// assert!(!option.convert_kana());
    /// ```
    pub fn convert_kana(&self) -> bool {
        self.kana
    }

    /// Ignore characters when convert.
    ///
    /// # Example
    ///
    /// ```rust
    /// use kelp::conv_option::ConvOption;
    ///
    /// let option = ConvOption::build().ignore("").finalize();
    /// assert_eq!(0, option.ignore_chars().len());
    ///
    /// let option = ConvOption::build().ignore("あいう").finalize();
    /// assert_eq!(3, option.ignore_chars().len());
    /// ```
    pub fn ignore_chars(&self) -> &Vec<char> {
        &self.ignore
    }
}

/// Builder of ConvOption
#[derive(Debug)]
pub struct ConvOptionBuilder {
    ascii: bool,
    digit: bool,
    ignore: Vec<char>,
    kana: bool,
}

impl ConvOptionBuilder {
    /// Returns a `ConvOptionBuilder` with default options.
    ///
    /// # Example
    ///
    /// ```rust
    /// use kelp::conv_option::ConvOptionBuilder;
    ///
    /// let option = ConvOptionBuilder::new().finalize();
    /// assert!(!option.convert_ascii());
    /// assert!(!option.convert_digit());
    /// assert!(!option.convert_kana());
    /// assert_eq!(0, option.ignore_chars().len());
    /// ```
    pub fn new() -> Self {
        ConvOptionBuilder {
            ascii: false,
            digit: false,
            ignore: vec![],
            kana: false,
        }
    }

    /// Set a flag of ascii.
    ///
    /// # Example
    ///
    /// ```rust
    /// use kelp::conv_option::ConvOptionBuilder;
    ///
    /// let option = ConvOptionBuilder::new().ascii(true).finalize();
    /// assert!(option.convert_ascii());
    /// ```
    pub fn ascii(mut self, ascii: bool) -> Self {
        self.ascii = ascii;
        self
    }

    /// Set a flag of digit.
    ///
    /// # Example
    ///
    /// ```rust
    /// use kelp::conv_option::ConvOptionBuilder;
    ///
    /// let option = ConvOptionBuilder::new().digit(true).finalize();
    /// assert!(option.convert_digit());
    /// ```
    pub fn digit(mut self, digit: bool) -> Self {
        self.digit = digit;
        self
    }

    /// Disabled convert with ascii.
    ///
    /// # Example
    ///
    /// ```rust
    /// use kelp::conv_option::ConvOptionBuilder;
    ///
    /// let option = ConvOptionBuilder::new().disable_ascii().finalize();
    /// assert!(!option.convert_ascii());
    /// ```
    pub fn disable_ascii(self) -> Self {
        self.ascii(false)
    }

    /// Disabled convert with digit.
    ///
    /// # Example
    ///
    /// ```rust
    /// use kelp::conv_option::ConvOptionBuilder;
    ///
    /// let option = ConvOptionBuilder::new().disable_digit().finalize();
    /// assert!(!option.convert_digit());
    /// ```
    pub fn disable_digit(self) -> Self {
        self.digit(false)
    }

    /// Disabled convert with kana.
    ///
    /// # Example
    ///
    /// ```rust
    /// use kelp::conv_option::ConvOptionBuilder;
    ///
    /// let option = ConvOptionBuilder::new().disable_kana().finalize();
    /// assert!(!option.convert_kana());
    /// ```
    pub fn disable_kana(self) -> Self {
        self.kana(false)
    }

    /// Enabled convert with ascii.
    ///
    /// # Example
    ///
    /// ```rust
    /// use kelp::conv_option::ConvOptionBuilder;
    ///
    /// let option = ConvOptionBuilder::new().enable_ascii().finalize();
    /// assert!(option.convert_ascii());
    /// ```
    pub fn enable_ascii(self) -> Self {
        self.ascii(true)
    }

    /// Enabled convert with digit.
    ///
    /// # Example
    ///
    /// ```rust
    /// use kelp::conv_option::ConvOptionBuilder;
    ///
    /// let option = ConvOptionBuilder::new().enable_digit().finalize();
    /// assert!(option.convert_digit());
    /// ```
    pub fn enable_digit(self) -> Self {
        self.digit(true)
    }

    /// Enabled convert with kana.
    ///
    /// # Example
    ///
    /// ```rust
    /// use kelp::conv_option::ConvOptionBuilder;
    ///
    /// let option = ConvOptionBuilder::new().enable_kana().finalize();
    /// assert!(option.convert_kana());
    /// ```
    pub fn enable_kana(self) -> Self {
        self.kana(true)
    }

    /// Build a `ConvOption`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use kelp::conv_option::ConvOptionBuilder;
    ///
    /// let option = ConvOptionBuilder::new()
    ///     .ascii(true)
    ///     .digit(true)
    ///     .ignore("あいう")
    ///     .kana(true)
    ///     .finalize();
    /// assert!(option.convert_ascii());
    /// assert!(option.convert_digit());
    /// assert_eq!(3, option.ignore_chars().len());
    /// assert!(option.convert_kana());
    /// ```
    pub fn finalize(self) -> ConvOption {
        ConvOption {
            ascii: self.ascii,
            digit: self.digit,
            ignore: self.ignore.clone(),
            kana: self.kana,
        }
    }

    /// Set ignore characters.
    ///
    /// # Example
    ///
    /// ```rust
    /// use kelp::conv_option::ConvOptionBuilder;
    ///
    /// let option = ConvOptionBuilder::new().ignore("あいう").finalize();
    /// assert_eq!(3, option.ignore_chars().len());
    /// ```
    pub fn ignore(mut self, ignore: &str) -> Self {
        self.ignore = ignore.chars().collect();
        self
    }

    /// Set a flag of kana.
    ///
    /// # Example
    ///
    /// ```rust
    /// use kelp::conv_option::ConvOptionBuilder;
    ///
    /// let option = ConvOptionBuilder::new().kana(false).finalize();
    /// assert!(!option.convert_kana());
    /// ```
    pub fn kana(mut self, kana: bool) -> Self {
        self.kana = kana;
        self
    }
}
