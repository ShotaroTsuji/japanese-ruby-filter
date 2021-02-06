//! # japanese-ruby-filter
//!
//! This crate provides a text filter which processes markups for Japanese ruby.
//!
//! ## Structure of ruby text
//!
//! A text with ruby consists of base characters and ruby characters.
//! Base characters and ruby characters are divided into the same number of groups.
//! Ruby characters may be displayed on the top of corresponding group of base characters.
//!
//! ## Supported Notations
//!
//! ### LaTeX-like notation
//!
//! LaTeX-like command notation is used for representing rubies.
//! It starts with `\ruby` and base characters and ruby characters are given in braces:
//! ```ignore
//! \ruby{武|家|諸法度}{ぶ|け|しょはっと}
//! ```
//! The first argument is base characters. A sign `|` separates the groups of base characters.
//! The second argument is ruby characters. Similar to base characters, `|` is used as a separator.
//!
//! In the example above, a ruby character `ぶ` corresponeds to a base character `武`.
//! Ruby characters `しょはっと` correspond to `諸法度`.
