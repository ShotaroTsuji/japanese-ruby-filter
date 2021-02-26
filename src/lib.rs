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
//! This crate supports only LaTeX-like notation in the current version.
//!
//! ### LaTeX-like notation
//!
//! LaTeX-like command notation is used for representing rubies.
//! The notation is based on the pxrubrica package, but it has a slight modification.
//! It starts with `\ruby` and base characters and ruby characters are given in braces:
//! ```ignore
//! \ruby{武|家|諸法度}{ぶ|け|しょはっと}
//! ```
//! The first argument is base characters. A sign `|` separates the groups of base characters.
//! The second argument is ruby characters. Similar to base characters, `|` is used as a separator.
//!
//! In the example above, a ruby character `ぶ` corresponeds to a base character `武`.
//! Ruby characters `しょはっと` correspond to `諸法度`.
//!
//! ## Filter for `pulldown-cmark`
//!
//! This crate provides a `pulldown-cmark` filter which parses LaTeX-like ruby notations and
//! converts them into HTML elements.
//! You can use this filter by adding `pulldown-cmark-filter` to features in `Cargo.toml`.
//! ```ignore
//! [dependencies]
//! japanese-ruby-filter = { version = "0.1.0", features = ["pulldown-cmark-filter"] }
//! ```
//!
//! An example code is given below:
//! ```
//! use pulldown_cmark::{Parser, Event, Tag};
//! use japanese_ruby_filter::pulldown_cmark_filter::RubyFilter;
//!
//! let s = "\\ruby{漢字}{かん|じ}";
//! let mut iter = RubyFilter::new(Parser::new(s));
//! assert_eq!(iter.next(), Some(Event::Start(Tag::Paragraph)));
//! assert_eq!(iter.next(), Some(Event::Html("<ruby>漢<rp>（</rp><rt>かん</rt><rp>）</rp>字<rp>（</rp><rt>じ</rt><rp>）</rp></ruby>".into())));
//! assert_eq!(iter.next(), Some(Event::End(Tag::Paragraph)));
//! assert_eq!(iter.next(), None);
//! ```

use std::borrow::Cow;

pub mod latex_like;
pub mod renderer;

#[cfg(feature = "pulldown-cmark-filter")]
pub mod pulldown_cmark_filter;

/// Plain text or ruby text
#[derive(Debug,Clone,PartialEq)]
pub enum Filtered<'a> {
    Plain(&'a str),
    Ruby(Ruby<'a>),
}

/// Pairs of base characters and ruby characters
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

    pub fn base(&self) -> &[Cow<'a, str>] {
        self.base.as_slice()
    }

    pub fn ruby(&self) -> &[Cow<'a, str>] {
        self.ruby.as_slice()
    }
}
