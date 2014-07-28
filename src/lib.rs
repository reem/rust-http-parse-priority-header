#![license = "MIT"]
#![feature(phase)]
#![deny(missing_doc)]
#![deny(unnecessary_qualification, non_camel_case_types,
        unused_variable, unnecessary_typecast)]

//! Parser for HTTP priority headers such as Accept-Encoding

extern crate regex;
#[phase(plugin, link)] extern crate regex_macros;

use regex::Regex;

static VALID_HEADER_ITEM: Regex = regex!(r"/^\s*(\S+?)\s*(?:;(.*))?$/");
static WHITESPACE: &'static [char] = &[' ', '\t', '\n'];

pub fn parse_priorities_for<S: Str>(header: &str, candidates: Vec<S>) -> Vec<(S, f64)> {
    use parser::parse_header;
    use matcher::priorities_for;

    let priorities = parse_header(header);
    priorities_for(&priorities, candidates).move_iter()
        .filter(|&(_, p)| p > 0.0)
        .collect()
}

pub mod parser {
    use super::{VALID_HEADER_ITEM, WHITESPACE};
    use std::collections::HashMap;

    pub fn parse_header<'a>(header: &'a str) -> HashMap<&'a str, f64> {
        header
            .trim_chars(WHITESPACE)
            .split(',')
            .filter_map(parse_header_item).collect()
    }


    fn parse_header_item(header_item: &str) -> Option<(&str, f64)> {
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

pub mod matcher {
    use std::collections::HashMap;
    use std::vec::MoveItems;
    use std::iter::Map;

    pub fn priorities_for<S: Str>(accepts: &HashMap<&str, f64>,
                                  provided: Vec<S>) -> Vec<(S, f64)> {
        let error = -1.0;
        provided.move_iter().map(|value| {
            let &priority = accepts.find_equiv(&value.as_slice()).unwrap_or(&error);
            (value, priority)
        }).collect()
    }
}

