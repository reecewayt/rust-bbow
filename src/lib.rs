//! Big Bag Of Words
//!
//! The "Big Bag Of Words" is used in text analysis and
//! machine learning.  It reduces a text to a collection of
//! words, each with a count of the number of occurrences.
//!
//! This implementation uses zero-copy strings when
//! reasonably possible to improve performance and reduce
//! memory usage.
//!
//! Words are separated by whitespace, and consist of a
//! span of one or more consecutive letters (any Unicode
//! code point in the "letter" class) with no internal
//! punctuation: leading and trailing punctuation are
//! removed.
//!
//! For example, the text
//!
//! ```text
//! "It ain't over untïl it ain't, over."
//! ```
//!
//! contains the sequence of words `"It"`, `"over"`,
//! `"untïl"`, `"it"`, `"over"`.
//!
//! Words in the bag containing uppercase letters will be
//! represented by their lowercase equivalent.

use std::borrow::Cow;
use std::collections::BTreeMap;

/// Each key in this struct's map is a word in some
/// in-memory text document. The corresponding value is the
/// count of occurrences.
#[derive(Debug, Default, Clone)]
pub struct Bbow<'a>(BTreeMap<Cow<'a, str>, usize>);

fn is_word(word: &str) -> bool {
    !word.is_empty() && word.chars().all(|c| c.is_alphabetic())
}

fn has_uppercase(word: &str) -> bool {
    word.chars().any(char::is_uppercase)
}

impl<'a> Bbow<'a> {
    /// Make a new empty target words list.
    pub fn new() -> Self {
        Self::default()
    }

    /// Parse the `target` text and add the sequence of
    /// valid words contained in it to this BBOW.
    ///
    /// This is a "builder method": calls can be
    /// conveniently chained to build up a BBOW covering
    /// multiple texts.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bbow::Bbow;
    /// let bbow = Bbow::new().extend_from_text("Hello world.");
    /// assert_eq!(2, bbow.len());
    /// assert_eq!(1, bbow.match_count("hello"));
    /// ```
    pub fn extend_from_text(mut self, target: &'a str) -> Self {
        // Iterate over the words in the target text, adding them to the map.
        for words in target.split_whitespace() {
            // Trim leading and trailing non-alphabetic characters from the word.
            let word = words.trim_matches(|c: char| !c.is_alphabetic());
            if is_word(word) {
                // Convert to lowercase if the word contains uppercase letters.
                let cow_word = if has_uppercase(word) {
                    Cow::Owned(word.to_lowercase())
                } else {
                    Cow::Borrowed(word)
                };

                // From the documentation: Add the word to the map, incrementing the count if it already exists.
                self.0
                    .entry(cow_word)
                    .and_modify(|curr| *curr += 1)
                    .or_insert(1);
            }
        }
        self
    }

    /// Report the number of occurrences of the given
    /// `keyword` that are indexed by this BBOW. The keyword
    /// should be lowercase and not contain punctuation, as
    /// per the rules of BBOW: otherwise the keyword will
    /// not match and 0 will be returned.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use bbow::Bbow;
    /// let bbow = Bbow::new()
    ///     .extend_from_text("b b b-banana b");
    /// assert_eq!(3, bbow.match_count("b"));
    /// ```
    pub fn match_count(&self, keyword: &str) -> usize {
        // Check if keyword is valid
        if !is_word(keyword) || has_uppercase(keyword) {
            return 0;
        }
        // Gets keyword reference from map, if it exists
        // If it doesn't exist, returns 0
        // If it does exists, copied() converts the reference to a value
        self.0.get(keyword).copied().unwrap_or(0)
    }

    pub fn words(&'a self) -> impl Iterator<Item = &'a str> {
        self.0.keys().map(|w| w.as_ref())
    }

    /// Count the overall number of words contained in this BBOW:
    /// multiple occurrences are considered separate.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use bbow::Bbow;
    /// let bbow = Bbow::new()
    ///     .extend_from_text("Can't stop this! Stop!");
    /// assert_eq!(3, bbow.count());
    /// ```
    pub fn count(&self) -> usize {
        // Iterates over the map, summing the values of each key
        self.0.values().sum()
    }

    /// Count the number of unique words contained in this BBOW,
    /// not considering number of occurrences.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use bbow::Bbow;
    /// let bbow = Bbow::new()
    ///     .extend_from_text("Can't stop this! Stop!");
    /// assert_eq!(2, bbow.len());
    /// ```
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Is this BBOW empty?
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_is_empty() {
        let bbow = Bbow::new();
        assert!(bbow.is_empty());
        assert_eq!(bbow.len(), 0);
        assert_eq!(bbow.count(), 0);
    }

    // Test from documentation example
    #[test]
    fn test_b_banana() {
        let bbow = Bbow::new().extend_from_text("b b b-banana b");
        assert_eq!(0, bbow.match_count("b-banana")); // Should be zero since it has a hyphen
        assert_eq!(3, bbow.match_count("b"));
    }

    // Test from documentation example
    #[test]
    fn test_cant_stop() {
        let bbow = Bbow::new().extend_from_text("Can't stop this! Stop!");
        assert_eq!(2, bbow.len());
        assert_eq!(3, bbow.count());
    }

    #[test]
    fn test_case_sensitivity() {
        let bbow = Bbow::new().extend_from_text("TEST test TEst tESt");
        assert_eq!(4, bbow.match_count("test"));
        assert_eq!(1, bbow.len());
        assert_eq!(4, bbow.count());
    }

    #[test]
    fn test_invalid_keyword_param() {
        let bbow = Bbow::new().extend_from_text("Hello world.");
        assert_eq!(0, bbow.match_count("")); // empty string
        assert_eq!(0, bbow.match_count("hello.")); // with punctuation
        assert_eq!(0, bbow.match_count("Hello")); // with uppercase
        assert_eq!(0, bbow.match_count("hello world")); // multiple words
    }

    #[test]
    fn test_large_iterative_input() {
        let mut bbow = Bbow::new().extend_from_text("Lets iterate over this text.");

        // Verify initial count
        assert_eq!(5, bbow.count());

        //
        for i in 0..100 {
            let text = match i % 2 {
                0 => "Lets iterate over this text.",
                _ => "Lets iterate over that text.",
            };
            bbow = bbow.extend_from_text(text);
        }

        // Unique count should be: 5 + 1 = 6
        assert_eq!(6, bbow.len());
        // Total bag of words should be: 5 + (5 * 50) + (5 * 50) = 505
        assert_eq!(505, bbow.count());

        // "This" count should be 51
        assert_eq!(51, bbow.match_count("this"));
        // "That" count should be 50
        assert_eq!(50, bbow.match_count("that"));
        // "iterate" count should be 101
        assert_eq!(101, bbow.match_count("iterate"));
        //
    }

    #[test]
    fn test_diff_unicode_types() {
        let mut bbow = Bbow::new().extend_from_text("café café café cafe!");

        assert_eq!(2, bbow.len());
        assert_eq!(4, bbow.count());
        assert_eq!(3, bbow.match_count("café"));
        assert_eq!(1, bbow.match_count("cafe"));

        // Japanese characters
        // Source: google translate
        bbow = bbow.extend_from_text("日本語");
        assert_eq!(3, bbow.len());
        assert_eq!(5, bbow.count());
        assert_eq!(1, bbow.match_count("日本語"));

        // Arabic characters
        // Source: google translate
        bbow = bbow.extend_from_text("Hello in arabic: مرحبًا");
        assert_eq!(7, bbow.len());
        assert_eq!(9, bbow.count());
        assert_eq!(1, bbow.match_count("مرحبًا"));
    }
}
