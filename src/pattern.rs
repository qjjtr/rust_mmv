use regex::Regex;
use std::collections::HashMap;

fn prepare_pattern_for_regex(pattern: &String) -> String {
    let mut regex_typed_pattern = String::from("^");
    for char in pattern.chars() {
        if char == '*' {
            regex_typed_pattern.push_str("(.");
            regex_typed_pattern.push(char);
            regex_typed_pattern.push(')');
        } else {
            if char == '.' {
                regex_typed_pattern.push('\\');
            }
            regex_typed_pattern.push(char);
        }
    }
    regex_typed_pattern.push('$');
    regex_typed_pattern
}

pub fn is_matched_by_pattern(name: &String, pattern: &String) -> bool {
    Regex::new(&prepare_pattern_for_regex(pattern))
        .unwrap()
        .is_match(name)
}

pub fn convert(
    matched_names: Vec<String>,
    pattern_from: &String,
    pattern_to: &String,
) -> HashMap<String, String> {
    let mut converted_names = HashMap::new();
    let filename = Regex::new(&prepare_pattern_for_regex(pattern_from)).unwrap();
    for name in matched_names {
        let captures = filename.captures(&name).unwrap();
        let mut new_name = pattern_to.clone();
        for i in 1..captures.len() {
            new_name = new_name.replace(&(String::from("#") + &i.to_string()), &captures[i]);
        }
        converted_names.insert(name, new_name);
    }
    converted_names
}

#[test]
fn test_prepare_pattern_for_regex() {
    assert_eq!(
        prepare_pattern_for_regex(&String::from("*")),
        String::from("^(.*)$")
    );
    assert_eq!(
        prepare_pattern_for_regex(&String::from("abc*de * f1234 **")),
        String::from("^abc(.*)de (.*) f1234 (.*)(.*)$")
    );
    assert_eq!(
        prepare_pattern_for_regex(&String::from("constanta))")),
        String::from("^constanta))$")
    );
}

#[test]
fn test_is_matched_by_pattern_simple() {
    assert!(is_matched_by_pattern(
        &String::from("rEal name"),
        &String::from("rEal name")
    ));
    assert!(is_matched_by_pattern(&String::from(""), &String::from("")));
    assert!(is_matched_by_pattern(
        &String::from("hello/aboba"),
        &String::from("hello/a*")
    ));

    assert!(!is_matched_by_pattern(
        &String::from("BIG"),
        &String::from("big")
    ));
    assert!(!is_matched_by_pattern(
        &String::from("pig"),
        &String::from("big")
    ));
    assert!(!is_matched_by_pattern(
        &String::from("apigs"),
        &String::from("pig")
    ));
}

#[test]
fn test_is_matched_by_pattern_hard() {
    assert!(is_matched_by_pattern(
        &String::from("aNyThInG"),
        &String::from("*")
    ));
    assert!(is_matched_by_pattern(
        &String::from("aNyThInG"),
        &String::from("a*")
    ));
    assert!(is_matched_by_pattern(
        &String::from("aNyThInG"),
        &String::from("aNy*G")
    ));
    assert!(is_matched_by_pattern(
        &String::from("aNyThInG"),
        &String::from("aNyThInG*")
    ));

    assert!(is_matched_by_pattern(
        &String::from("aNyThInG"),
        &String::from("a*T*")
    ));
    assert!(is_matched_by_pattern(
        &String::from("aNyThInG"),
        &String::from("*Ny*T*")
    ));

    assert!(!is_matched_by_pattern(
        &String::from("bad_thing"),
        &String::from("any*")
    ));
    assert!(!is_matched_by_pattern(
        &String::from("bad_thing"),
        &String::from("bad_*n")
    ));
    assert!(!is_matched_by_pattern(
        &String::from("bad_thing"),
        &String::from("ad_*ng")
    ));
    assert!(!is_matched_by_pattern(
        &String::from("bad_thing"),
        &String::from("bad_*_*")
    ));
}

#[test]
fn test_convert() {
    let names = vec![
        String::from("hello/aboba"),
        String::from("hello/arbus"),
        String::from("hello/akula"),
    ];
    let mut result = HashMap::new();
    for name in &names {
        result.insert(name.clone(), String::from("alo/") + &name[7..].to_string());
    }
    assert_eq!(
        convert(names, &String::from("hello/a*"), &String::from("alo/#1")),
        result
    );

    result.clear();
    result.insert(String::from("abcdefg"), String::from("bc_efg"));
    assert_eq!(
        convert(
            vec![String::from("abcdefg")],
            &String::from("a*d*"),
            &String::from("#1_#2")
        ),
        result
    );

    result.clear();
    result.insert(String::from("hello"), String::from("hell"));
    assert_eq!(
        convert(
            vec![String::from("hello")],
            &String::from("hell*"),
            &String::from("hell")
        ),
        result
    );

    result.clear();
    result.insert(String::from("from"), String::from("to"));
    assert_eq!(
        convert(
            vec![String::from("from")],
            &String::from("from"),
            &String::from("to")
        ),
        result
    );
}
