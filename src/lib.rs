#![license = "MIT"]
#![feature(phase, globs)]
#![deny(missing_doc)]
#![deny(unnecessary_qualification, non_camel_case_types,
        unused_variable, unnecessary_typecast)]

//! Parser for HTTP priority headers such as Accept-Encoding

extern crate regex;
#[phase(plugin)] extern crate regex_macros;

use regex::Regex;

static VALID_HEADER_ITEM: Regex = regex!(r"^\s*(\S+?)\s*((;(.*)+))?$");
static WHITESPACE: &'static [char] = &[' ', '\t', '\n'];

/// Get the priorities for several candidates from an unparsed header.
///
/// `parse_priorities_for("a;q=0.7,b;q=0.3", vec!["a", "b"])` will give back
/// `vec![("a", 0.7), ("b", 0.3)]`
pub fn parse_priorities_for<S: Str>(header: S, candidates: Vec<S>) -> Vec<(S, f64)> {
    use parser::parse_header;
    use matcher::priorities_for;

    let priorities = parse_header(header.as_slice());
    priorities_for(&priorities, candidates).move_iter()
        .filter(|&(_, p)| p > 0.0)
        .collect()
}

/// Contains the parser for priority headers such as Accept-Encoding
pub mod parser {
    use super::{VALID_HEADER_ITEM, WHITESPACE};
    use std::collections::HashMap;

    /// Parse a priority header into a HashMap of values to their priorities.
    pub fn parse_header<'a>(header: &'a str) -> HashMap<&'a str, f64> {
        header
            .trim_chars(WHITESPACE)
            .split(',')
            .filter_map(parse_header_item).collect()
    }


    #[doc(hidden)]
    pub fn parse_header_item(header_item: &str) -> Option<(&str, f64)> {
        // Is this a valid header item? Extract its values.
        VALID_HEADER_ITEM.captures(header_item).map(|captures| {
            let value = captures.at(1);
            let params = captures.at(2);

            let priority = params.split(';').find(|&param| {
                // Is this param q?
                param.trim_chars(WHITESPACE)
                    .split('=')
                    .collect::<Vec<&str>>()[0] == "q"

            // No q found, so default to 1.0
            }).map_or(1.0, |param| {

                // Found q, parse its value
                from_str::<f64>(param.trim_chars(WHITESPACE)
                    .split('=')
                    .collect::<Vec<&str>>()[1])

            // Failed to parse q value, default to -1
            }.unwrap_or(-1.0));

            (value, priority)
        })
    }
}

/// Convenience functions for working with priorities.
pub mod matcher {
    use std::collections::HashMap;

    /// Get priorities for several values
    pub fn priorities_for<S: Str>(accepts: &HashMap<&str, f64>,
                                  provided: Vec<S>) -> Vec<(S, f64)> {
        let error = -1.0;
        provided.move_iter().map(|value| {
            let &priority = accepts.find_equiv(&value.as_slice()).unwrap_or(&error);
            (value, priority)
        }).collect()
    }
}

#[cfg(test)]
mod test {
    pub use super::matcher::*;
    pub use super::parser::*;
    pub use super::parse_priorities_for;
    pub use super::VALID_HEADER_ITEM;

    mod end_to_end {
        use super::*;

        #[test]
        fn test() {
            assert_eq!(parse_priorities_for("a;q=0.7,b;q=0.3", vec!["a", "b"]),
                       vec![("a", from_str("0.7").unwrap()), ("b", from_str("0.3").unwrap())]);
        }
    }

    mod parser {
        use super::*;

        #[test]
        fn test_header_item() {
            assert_eq!(parse_header_item("a;q=0.7"), Some(("a", from_str("0.7").unwrap())));
        }

    }

    mod matcher {
        use super::*;
    }

    #[test]
    fn test_valid_header_item() {
        assert!(VALID_HEADER_ITEM.is_match("a;q=0.7"));
    }
}

