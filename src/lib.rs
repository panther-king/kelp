//! # kelp
//!
//! This is a porting from [jaconv(Python)](https://github.com/ikegami-yukino/jaconv).
mod conv_table;
mod convert;

pub use convert::h2z;
pub use convert::hira2hkata;
pub use convert::hira2kata;
pub use convert::kata2hira;
pub use convert::z2h;

/// Convert options
#[derive(Debug, Default)]
pub struct ConvOption {
    pub ascii: bool,
    pub digit: bool,
    pub ignore: String,
    pub kana: bool,
}
