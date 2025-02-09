# Rust Big Bag of Words

Author: Reece Wayt  
Contact: reecwayt@pdx.edu  
Date: 2/9/2025 
GitHub Repo: https://github.com/reecewayt/rust-bbow

## Description
This is a rust library crate that implements methods for working with a "Big Bag of Words" (BBOW). Briefly, a bag of words is a way of reducing a stream of text, in a document or string, to a frequency of each word. For a more detailed description, I found this [article](https://builtin.com/machine-learning/bag-of-words) helpful. The underlying data structure this library is build on is [BTreeMap](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#).

## Methods
- `new()`: Creates a new empty BBOW instance.
- `extend_from_text(&str)`: Adds words from the provided text to the BBOW. Words are converted to lower case and any punctuation will be trimmed from string. Invalid words are ignored. Note that you can call this function for adding or chaining together texts into the data structure. 
- `match_count(&str)`: Returns the number of occurrences of a given keyword in the BBOW. The keyword must be lowercase and contain only alphabetic characters.
- `words()`: Returns an iterator over all unique words in the BBOW.
- `count()`: Returns the total number of words in the BBOW, counting multiple occurrences separately.
- `len()`: Returns the number of unique words in the BBOW.
- `is_empty()`: Returns true if the BBOW contains no words, false otherwise.

### Tests
The library included several tests to validate the methods but also provide example use cases of the methods. Below is an iterative example that chains together texts to create a "large-ish" bag of words. 

```rust
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
```

### Compiling and Testing
To compile the library: 
```rust
cargo build
```
To run the test cases : 
```rust
cargo test
```
### Using in your project
To use this library in your Rust project, add the following to your `cargo.toml`
```rust
[dependencies]
bbow = { git = "https://github.com/reecewayt/rust-bbow" }
```

### Sources
Development Tools:
- Rust Analyzer for syntax and error help
- GitHub Coplit in VS-Code for AI generative code completion, and pattern matching
- Claude AI (Anthropic) assisted with understanding Cow library and borrow/owned types in rust

**Note:** These tools were used to help with development but code and design decisions were made independently. AI tools were generally used to help with syntax and documentation. 
