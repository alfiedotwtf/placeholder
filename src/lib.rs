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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string() {
        let before = String::from("");
        let after = String::from("");
        let values = HashMap::new();

        assert!(render(&before, &values) == Ok(after));
    }

    #[test]
    fn no_substitution() {
        let before = String::from("Hello world");
        let after = String::from("Hello world");
        let values = HashMap::new();

        assert!(render(&before, &values) == Ok(after));
    }

    #[test]
    fn ignore_escaped() {
        let before = String::from("Hello {{middle} w{{orld");
        let after = String::from("Hello {{middle} w{{orld");
        let values = HashMap::new();

        assert!(render(&before, &values) == Ok(after));
    }

    #[test]
    fn missing_start_value() {
        let before = String::from("{start} world");
        let values = HashMap::new();

        assert!(render(&before, &values) == Err(String::from("start")));
    }

    #[test]
    fn missing_middle_value() {
        let before = String::from("Hello {middle} world");
        let values = HashMap::new();

        assert!(render(&before, &values) == Err(String::from("middle")));
    }
    #[test]
    fn missing_end_value() {
        let before = String::from("Hello {end}");
        let values = HashMap::new();

        assert!(render(&before, &values) == Err(String::from("end")));
    }

    #[test]
    fn missing_one_value() {
        let before = String::from("{start} {middle} world");

        let mut values = HashMap::new();
        values.insert(String::from("start"), String::from("Hello"));

        assert!(render(&before, &values) == Err(String::from("middle")));
    }

    #[test]
    fn missing_one_value_again() {
        let before = String::from("{start} {middle} world");

        let mut values = HashMap::new();
        values.insert(String::from("middle"), String::from("beautiful"));

        assert!(render(&before, &values) == Err(String::from("start")));
    }

    #[test]
    fn start() {
        let before = String::from("{start} world");
        let after = String::from("Hello world");

        let mut values = HashMap::new();
        values.insert(String::from("start"), String::from("Hello"));

        assert!(render(&before, &values) == Ok(after));
    }

    #[test]
    fn end() {
        let before = String::from("Hello {end}");
        let after = String::from("Hello world");

        let mut values = HashMap::new();
        values.insert(String::from("end"), String::from("world"));

        assert!(render(&before, &values) == Ok(after));
    }

    #[test]
    fn middle() {
        let before = String::from("Hello {middle} world");
        let after = String::from("Hello beautiful world");

        let mut values = HashMap::new();
        values.insert(String::from("middle"), String::from("beautiful"));

        assert!(render(&before, &values) == Ok(after));
    }

    #[test]
    fn hello_beautiful_world() {
        let before = String::from("{start} {middle} {end}");
        let after = String::from("Hello beautiful world");

        let mut values = HashMap::new();
        values.insert(String::from("start"), String::from("Hello"));
        values.insert(String::from("middle"), String::from("beautiful"));
        values.insert(String::from("end"), String::from("world"));

        assert!(render(&before, &values) == Ok(after));
    }

    #[test]
    fn multi_line_hello() {
        let before = String::from("{start} is a\n{middle} test to see\nif the regex {end}");
        let after = String::from("This is a\nmulti-line test to see\nif the regex works");

        let mut values = HashMap::new();
        values.insert(String::from("start"), String::from("This"));
        values.insert(String::from("middle"), String::from("multi-line"));
        values.insert(String::from("end"), String::from("works"));

        assert!(render(&before, &values) == Ok(after));
    }

    #[test]
    fn a_longer_test() {
        let before =
            String::from(format!("{}\n{}\n{}\n{}\n{}",
            "No society can surely {fourth}e flourishing {first} happy, {third} which {second} far greater part {third} {second}",
            "mem{fourth}ers are poor {first} misera{fourth}le. It is but equity, besides, that they who feed,",
            "clothe, {first} lodge {second} whole body {third} {second} people, should have such a share {third} {second}",
            "produce {third} their own la{fourth}our as to be themselves tolera{fourth}ly well fed, clothed, {first}",
            "lodged."));

        let after =
            String::from(format!("{}\n{}\n{}\n{}\n{}",
            "No society can surely be flourishing and happy, of which the far greater part of the",
            "members are poor and miserable. It is but equity, besides, that they who feed,",
            "clothe, and lodge the whole body of the people, should have such a share of the",
            "produce of their own labour as to be themselves tolerably well fed, clothed, and",
            "lodged."));

        let mut values = HashMap::new();
        values.insert(String::from("first"), String::from("and"));
        values.insert(String::from("second"), String::from("the"));
        values.insert(String::from("third"), String::from("of"));
        values.insert(String::from("fourth"), String::from("b"));
        values.insert(String::from("fifth"), String::from("these"));
        values.insert(String::from("sixth"), String::from("last"));
        values.insert(String::from("seventh"), String::from("ones"));
        values.insert(String::from("eighth"), String::from("do"));
        values.insert(String::from("ninth"), String::from("not"));
        values.insert(String::from("tenth"), String::from("exist"));

        assert!(render(&before, &values) == Ok(after));
    }
}
