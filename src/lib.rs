use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref MATCH_START: Regex = Regex::new(r"^[{](\w+)[}]").unwrap();
    static ref MATCH_OTHER: Regex = Regex::new(r"[^{][{](\w+)[}]").unwrap();
}

pub fn render(text: &str, values: &HashMap<String, String>) -> Result<String, String> {
    let mut text_replaced = match MATCH_START.captures(text) {
        None => text.to_string(),
        Some(capture) => match capture.get(1) {
            None => panic!("at the disco"),
            Some(key) => match values.get(key.as_str()) {
                None => return Err(key.as_str().to_string()),
                Some(value) => Regex::new(format!("^[{{]{}[}}]", key.as_str()).as_str())
                    .unwrap()
                    .replace(text, value)
                    .to_string(),
            },
        },
    };

    loop {
        match MATCH_OTHER.captures(&text_replaced) {
            None => break,
            Some(capture) => match capture.get(1) {
                None => panic!("at the disco"),
                Some(key) => match values.get(key.as_str()) {
                    None => return Err(key.as_str().to_string()),
                    Some(value) => {
                        text_replaced =
                            Regex::new(format!("([^{{])[{{]{}[}}]", key.as_str()).as_str())
                                .unwrap()
                                .replace(&text_replaced, format!("${{1}}{}", value))
                                .to_string()
                    }
                },
            },
        }
    }

    Ok(text_replaced)
}
