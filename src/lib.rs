use std::{collections::HashMap, path::Path};

use minidom::Element;
use rusqlite::{types::ValueRef, Connection};

mod utils;
use utils::XmlVariables;

mod error;
pub use error::PersonnelError;

#[cfg(test)]
mod tests;

type Result<T> = std::result::Result<T, PersonnelError>;

/**
Parsonnel Manager instance.
*/
pub struct PersonnelManager {
    db: Connection,
}

impl PersonnelManager {
    /**
    Creates new personnel manager instance.

    # Arguments
    * `db_path` - path to SQLite database.
    */
    pub fn new(db_path: &Path) -> Result<PersonnelManager> {
        match Connection::open(db_path) {
            Ok(conn) => Ok(PersonnelManager { db: conn }),
            Err(err) => Err(PersonnelError::SqlError(err)),
        }
    }

    /**
    This function will return parameters decleared in params section of document template.

    # Arguments
    * `template` - a string containing XML document.
    */
    pub fn get_params(template: impl Into<String>) -> Result<HashMap<String, String>> {
        let mut params: HashMap<String, String> = HashMap::new();

        let root = match template.into().parse::<Element>() {
            Ok(root) => root,
            Err(err) => return Err(PersonnelError::XmlError(err)),
        };

        let obj_params = match root.get_child("params", "pm") {
            Some(params) => params,
            None => return Err(PersonnelError::EmptyXml),
        };

        for param in obj_params.children() {
            let Some(key) = param.attr("name") else {
                // It's possible to comeback from this error. So we just print error and go on.
                println!("error! param does not have a name attribute!");
                continue;
            };

            // If there is default value insert with it if no then with empty string.
            if let Some(value) = param.attr("default") {
                params.insert(key.into(), value.into());
            } else {
                params.insert(key.into(), String::new());
            };
        }

        Ok(params)
    }

    /**
    This function will get one row from database.

    # Arguments
    * `request` - string slice wich is valid SQL request.
    * `params` - array of params for request.
    */
    pub fn make_request<P>(&self, request: &str, params: P) -> Result<HashMap<String, String>>
    where
        P: rusqlite::Params,
    {
        let mut stmt = match self.db.prepare(request) {
            Ok(stmt) => stmt,
            Err(err) => return Err(PersonnelError::SqlError(err)),
        };

        let column_names: Vec<String> = stmt.column_names().into_iter().map(String::from).collect();

        let mut rows = match stmt.query(params) {
            Ok(rows) => rows,
            Err(err) => return Err(PersonnelError::SqlError(err)),
        };

        match rows.next() {
            Ok(row) => {
                let Some(row) = row else {
                    return Err(PersonnelError::EmptySqlResult)
                };

                let mut result = HashMap::<String, String>::new();

                for column in &column_names {
                    let cell = match row.get_ref(column.as_str()) {
                        Ok(cell) => cell,
                        Err(err) => return Err(PersonnelError::SqlError(err)),
                    };

                    let value = match cell {
                        ValueRef::Integer(i) => i.to_string(),
                        ValueRef::Real(f) => f.to_string(),
                        ValueRef::Text(t) => match String::from_utf8(t.to_vec()) {
                            Ok(text) => text,
                            Err(err) => return Err(PersonnelError::FromUtf8Error(err)),
                        },
                        ValueRef::Blob(b) => match String::from_utf8(b.to_vec()) {
                            Ok(blob) => blob,
                            Err(err) => return Err(PersonnelError::FromUtf8Error(err)),
                        },

                        ValueRef::Null => String::new(),
                    };

                    result.insert(column.clone(), value);
                }

                Ok(result)
            }
            // If we get no rows return error.
            Err(err) => Err(PersonnelError::SqlError(err)),
        }
    }

    /**
    This function will try to make report and return result.

    Note. You might want to look at `get_params` before using it.

    # Arguments
    * `params` - params to use in make process.
    * `tempalte` - XML Personnel Manager template.
     */
    pub fn make_report(
        &self,
        params: HashMap<String, String>,
        template: impl Into<String>,
    ) -> Result<String> {
        let mut output = String::new();

        let mut variables = XmlVariables::new();
        variables.extend(params);

        let root = match template.into().parse::<Element>() {
            Ok(r) => r,
            Err(e) => return Err(PersonnelError::XmlError(e)),
        };

        for child in root.children() {
            match child.name() {
                "block" => 'block: loop {
                    for block_child in child.children() {
                        match block_child.name() {
                            "sql" => {
                                if !Self::handle_sql(self, block_child, &mut variables) {
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

    fn handle_sql(&self, sql_block: &Element, variables: &mut XmlVariables) -> bool {
        for request in sql_block.children() {
            let Some(name)= request.attr("name") else {
                                    println!("Error! Request tag does not have name to save.");
                                    continue;
                                };

            let Some(sql_statement)= request.attr("request") else {
                                    println!("Error! Request tag does not have SQL statement.");
                                    continue;
                                };

            if let Ok(person) = Self::make_request(self, &variables.replace(sql_statement), []) {
                for (k, v) in person {
                    variables.insert(format!("{}.{}", name, k), v);
                }
            } else {
                return false;
            }
        }

        true
    }
}
