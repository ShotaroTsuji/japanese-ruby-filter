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

use std::borrow::Cow;

pub mod latex_like;

#[cfg(feature = "pulldown-cmark-filter")]
pub mod pulldown_cmark_filter;

#[derive(Debug,Clone,PartialEq)]
pub enum Filtered<'a> {
    Plain(&'a str),
    Ruby(Ruby<'a>),
}

#[derive(Debug,Clone,PartialEq)]
pub struct Ruby<'a> {
    base: Vec<Cow<'a, str>>,
    ruby: Vec<Cow<'a, str>>,
}

impl<'a> Ruby<'a> {
    pub fn from_str_vecs(base: Vec<&'a str>, ruby: Vec<&'a str>) -> Ruby<'a> {
        Ruby {
            base: base.into_iter().map(|s| Cow::Borrowed(s)).collect(),
            ruby: ruby.into_iter().map(|s| Cow::Borrowed(s)).collect(),
        }
    }
}
