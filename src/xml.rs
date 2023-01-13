use std::collections::HashMap;

use minidom::Element;
use rusqlite::Connection;

use crate::db::make_onerow_request;

/// Place to incapsulate all variable releted stuff in XML documents.
struct XmlVariables {
    content: HashMap<String, String>,
}

impl XmlVariables {
    fn new() -> Self {
        XmlVariables {
            content: HashMap::<String, String>::new(),
        }
    }

    /**
    This function take key and value and push it to variables list.

    # Arguments
    * `str` - string to find matches.

    # Examples
    * Simplistic.
        ```
        let mut variables = XmlVariables::new();
        variables.insert("name", "value");

        assert_eq!(variables.replace("{name}"), "value");
        ```
    <br/>
    * You can use set value of existing value to new one.
        ```
        let mut variables = XmlVariables::new();
        variables.insert("name1", "value");
        variables.insert("name2", "{name1}");

        assert_eq!(variables.replace("{name2}"), "value");
        ```

    NOTE: This function can cost really bad if you pass not `mut String`.
    */
    fn insert(&mut self, k: impl Into<String>, v: impl Into<String>) {
        self.content.insert(k.into(), self.replace(v.into()));
    }

    /**
    This function take string and match all variables. If find any it fill replace it in it.

    # Arguments
    * `str` - string to find matches.

    NOTE: This function can cost really bad if you pass not `mut String`.
    */
    fn replace(&self, str: impl Into<String>) -> String {
        let mut str = str.into();

        for (field, value) in &self.content {
            str = str.replace(format!("{{{}}}", field).as_str(), value.as_str());
        }

        str
    }

    /// Reroute to HashMap extend.
    fn extend(&mut self, iter: HashMap<String, String>) {
        self.content.extend(iter);
    }
}

/**
 * This function will return parameters decleared in params section of document template.
 *
 * # Arguments
 * `xml_data` - a string containing XML document.
 */
pub fn get_params(xml_data: impl Into<String>) -> Result<Vec<String>, String> {
    let mut params_list: Vec<String> = vec![];

    let Ok(root) = xml_data.into().parse::<Element>() else {
        return Err("Can't parse root".to_string());
    };

    for param in root.get_child("params", "pm").unwrap().children() {
        let Some(param) = param.attr("name") else {
            // It's possible to comeback from this error. So we just print error and go on.
            println!("Error! Param does not have a name attribute!");
            continue;
        };
        params_list.push(param.into());
    }

    Ok(params_list)
}

pub fn parse_xml(
    xml_data: impl Into<String>,
    conn: &Connection,
    params: HashMap<String, String>,
) -> Result<String, String> {
    let mut output = String::new();

    let mut variables = XmlVariables::new();
    variables.extend(params);

    let Ok(root) = xml_data.into().parse::<Element>() else {
        return Err("Can't parse XML file!".to_string());
    };

    for child in root.children() {
        match child.name() {
            "block" => 'block: loop {
                for block_child in child.children() {
                    match block_child.name() {
                        "sql" => {
                            if handle_sql(block_child, &mut variables, conn).is_err() {
                                break 'block;
                            }
                        }
                        "html" => {
                            output.push_str(&variables.replace(block_child.text()));
                        }
                        _ => (),
                    }
                }
                if let (Some(k), Some(v)) = (child.attr("set"), child.attr("value")) {
                    variables.insert(k, v);
                }

                if child.attr("loop_type").is_none() {
                    break;
                }
            },
            "variable" => {
                if let (Some(k), Some(v)) = (child.attr("name"), child.attr("value")) {
                    variables.insert(k, v);
                }
            }
            _ => (),
        }
    }

    Ok(output)
}

fn handle_sql(
    sql_block: &Element,
    variables: &mut XmlVariables,
    conn: &Connection,
) -> Result<(), ()> {
    for request in sql_block.children() {
        let Some(name)= request.attr("name") else {
                                    println!("Error! Request tag does not have name to save.");
                                    continue;
                                };

        let Some(sql_statement)= request.attr("request") else {
                                    println!("Error! Request tag does not have SQL statement.");
                                    continue;
                                };

        if let Ok(person) = make_onerow_request(conn, &variables.replace(sql_statement), []) {
            for (k, v) in person {
                variables.insert(format!("{}.{}", name, k), v);
            }
        } else {
            return Err(());
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn variable() {
        let mut variables = XmlVariables::new();
        variables.insert("name", "value");

        assert_eq!(variables.replace("{name}"), "value");
    }

    #[test]
    fn variable_recursive() {
        let mut variables = XmlVariables::new();
        variables.insert("name1", "value");
        variables.insert("name2", "{name1}");

        assert_eq!(variables.replace("{name2}"), "value");
    }

    #[test]
    fn xml_test() {
        parse_xml(
            include_str!("../../templates/content-template.xml"),
            &rusqlite::Connection::open(
                "/home/donetchan/Programming/rust/personnel-manager/personnel.sqlite",
            )
            .unwrap(),
            HashMap::new(),
        )
        .unwrap();
    }
}
