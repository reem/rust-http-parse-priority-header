 #![license = "MIT"]
 #![deny(missing_doc)]
 #![deny(unnecessary_qualification, non_camel_case_types,
         unused_variable, unnecessary_typecast)]

//! Parser for HTTP priority headers such as Accept-Encoding

static VALID_HEADER_ITEM: Regex = regex!(r"/^\s*(\S+?)\s*(?:;(.*))?$/");
static WHITESPACE: &'static [char] = &[' ', '\t', '\n'];

pub fn parse_priorities_for<S: Str>(header: &str, candidates: Vec<S>) -> Vec<(S, f64)> {
    use parse::parse_header;
    use match::priorities_for;

    let priorities = parse_header(header);
    priorities_for(&priorities, candidates)
        .filter(&(_, p) p > 0.0)
        .collect()
}

pub mod parse {
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
                from_str::<f64>(param.trim_chars(whitespace)
                    .split('=')
                    .collect::<Vec<&str>>()[1])

            // Failed to parse q value, default to -1
            }.unwrap_or(-1.0));

            (value, priority)
        })
    }
}

pub mod match {
    use std::vec::MoveItems;

    pub fn priorities_for<S: Str>(accepts: &HashMap<&str, f64>,
                                  provided: Vec<S>) -> MoveItems<(S, f64)> {
        provided.move_iter().map(|value| {
            let &priority = accepts.find_equiv(&possibility.as_slice()).unwrap_or(&error);
            (possibility, priority)
        })
    }
}

