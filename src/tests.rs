use super::*;

use std::fs::read_to_string;

#[test]
fn get_params() {
    let template = r#"<?xml version="1.0" encoding="UTF-8"?>
                <root xmlns="pm">
                <params>
                    <param name="param1" />
                    <param name="param2" default="x" />
                </params>
                </root>
            "#;

    let params = PersonnelManager::get_params(template).unwrap();

    let result = HashMap::from([
        ("param1".to_string(), String::new()),
        ("param2".to_string(), "x".to_string()),
    ]);

    assert_eq!(params, result)
}

#[test]
fn make_request() {
    let personnel = PersonnelManager::new(Path::new("./resources/test.sqlite")).unwrap();

    let responce = personnel
        .make_request("SELECT name FROM person WHERE id = ?", [0])
        .unwrap();

    assert_eq!(responce["name"], "Someone")
}

#[test]
fn make_simple_report() {
    let personnel = PersonnelManager::new(Path::new("resources/test.sqlite")).unwrap();

    let template = read_to_string("resources/test-template.xml").unwrap();

    let report = personnel
        .make_report([("id".to_string(), "0".to_string())].into(), template)
        .unwrap();

    let report_example = read_to_string("resources/test-report.html").unwrap();

    assert_eq!(report, report_example)
}
