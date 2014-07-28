 #![license = "MIT"]
 #![deny(missing_doc)]
 #![deny(unnecessary_qualification, non_camel_case_types,
         unused_variable, unnecessary_typecast)]

//! Parser for HTTP priority headers such as Accept-Encoding

static VALID_HEADER_ITEM: Regex = regex!(r"/^\s*(\S+?)\s*(?:;(.*))?$/");
static WHITESPACE: &'static [char] = &[' ', '\t', '\n'];

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

