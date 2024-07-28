//! Substring utility functions.
//!
//! This module provides a convenient API for extracting substrings using a
//! [pattern](core::str::pattern::Pattern).
//!
//! # Examples
//! ```
//! use aidoku_helpers::substring::Substring;
//! assert_eq!(
//!     r#"background-image: url("paper.gif");"#.substring_after(r#"(""#),
//!     Some(r#"paper.gif");"#),
//! );
//! assert_eq!(
//!     r#"background-image: url("paper.gif");"#.substring_before(r#"(""#),
//!     Some(r#"background-image: url"#),
//! );
//! assert_eq!(
//!     "Baker Betty Botter Bought some Butter".substring_before_last('B'),
//!     Some("Baker Betty Botter Bought some "),
//! );
//! assert_eq!(
//!     "Baker Betty Botter Bought some Butter".substring_after_last('B'),
//!     Some("utter"),
//! );
//! ```
use core::str::pattern::{Pattern, ReverseSearcher, Searcher};

pub trait Substring {
    /// Returns a substring before the first occurence of pattern.
    fn substring_before<P: Pattern>(&self, pat: P) -> Option<&str>;

    /// Returns a substring before the last occurence of pattern.
    fn substring_before_last<P: Pattern>(&self, pat: P) -> Option<&str>
    where
        for<'a> P::Searcher<'a>: ReverseSearcher<'a>;

    /// Returns a substring after the first occurence of pattern.
    fn substring_after<P: Pattern>(&self, pat: P) -> Option<&str>;

    /// Returns a substring after the last occurence of pattern.
    fn substring_after_last<'a, P>(&'a self, pat: P) -> Option<&'a str>
    where
        P: Pattern,
        <P as Pattern>::Searcher<'a>: ReverseSearcher<'a>;
}

impl Substring for str {
    #[inline]
    fn substring_before<P: Pattern>(&self, pat: P) -> Option<&str> {
        match self.find(pat) {
            Some(i) => Some(&self[..i]),
            None => None,
        }
    }

    #[inline]
    fn substring_before_last<P: Pattern>(&self, pat: P) -> Option<&str>
    where
        for<'a> P::Searcher<'a>: ReverseSearcher<'a>,
    {
        self.rfind(pat).map(|idx| &self[..idx])
    }

    #[inline]
    fn substring_after<P: Pattern>(&self, pat: P) -> Option<&str> {
        match pat.into_searcher(self).next_match() {
            Some((_, end)) => Some(&self[end..]),
            None => None,
        }
    }

    #[inline]
    fn substring_after_last<'a, P>(&'a self, pat: P) -> Option<&'a str>
    where
        P: Pattern,
        <P as Pattern>::Searcher<'a>: ReverseSearcher<'a>,
    {
        match pat.into_searcher(self).next_match_back() {
            Some((_, end)) => Some(&self[end..]),
            None => None,
        }
    }
}
