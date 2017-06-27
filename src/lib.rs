//! # kelp
//!
//! This is a porting from [jaconv(Python)](https://github.com/ikegami-yukino/jaconv).
pub mod conv_option;

mod conv_table;
mod convert;

pub use convert::hira2kata;
pub use convert::hira2hkata;
pub use convert::kata2hira;
pub use convert::h2z;
pub use convert::z2h;
