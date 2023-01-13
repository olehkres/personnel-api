//! Personnel-api is a backend inside Personnel Manager crate group.
//!
//! If you just want to use Personnel Manager look for either personnel-cli or personnel-gui.

/// Working with data from SQLite database.
pub mod db;
/// Defined what person is.
pub mod person;
/// Making report.
pub mod report;
/// Making report from a new XML template.
pub mod xml;

use rusqlite::Connection;
use std::{collections::HashMap, fs::read_to_string, path::Path};

pub struct Template {
    xml_data: String,
    db: Connection,
}

impl Template {
    /**
    Creates template from XML and SQLite fiels.

    # Arguments
    * `xml_path` - path to XML file.
    * `db_path` - path to SQLite database.
    */
    pub fn new(xml_path: &Path, db_path: &Path) -> Self {
        Template {
            xml_data: read_to_string(xml_path).unwrap(),
            db: Connection::open(db_path).unwrap(),
        }
    }

    /**
    This function will create and save report.

    # Arguments
    * `params` - HashMap of key nd value according to XML report.
    * `output` - save path for output file.
    */
    pub fn create_report(&self, params: HashMap<String, String>, output: &Path) {
        let output_data = xml::parse_xml(&self.xml_data, &self.db, params);

        std::fs::write(output, output_data.unwrap()).unwrap();
    }

    /// This function will return parameters decleared in params section of document template
    pub fn get_params(&self) -> Vec<String> {
        xml::get_params(&self.xml_data).unwrap()
    }
}

#[cfg(test)]
mod tests {}
