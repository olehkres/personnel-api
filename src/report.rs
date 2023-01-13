use std::fs;
use std::{io, path::Path};

use crate::person::Person;

/**
 This function will make report from person list and document.

 # Arguments
 * `person_chain` - A vector filled with personnel from lowest to higest commander.
 * `content_template` - A path to text file with variables that
 will be replaced with data from `person_chain`.
 * `output` - A path to text file where result will be stored.
*/
pub fn make_report(
    person_chain: &[Person],
    content_template: &Path,
    reference_template: &Path,
    output: &Path,
) -> Result<(), io::Error> {
    // Here we making a content report.
    // Note: we will retur that variable as a result.
    let mut report = replace(
        &person_chain[0],
        fs::read_to_string(content_template).unwrap(),
    );

    // Set direct subordinate. Often we need data of commander subordinate to form
    // reference report from commander.
    let mut dsubordinate = &person_chain[0];

    let reference_data = fs::read(reference_template)?;
    // Note: we need to use template with multiple persons so it have to be unmutable.
    let reference_data = String::from_utf8(reference_data).unwrap();

    for person in &person_chain[1..] {
        // We will save reference report for that particular person here.
        // Fill gaps of commander.
        let mut temp_reference_data: String = replace(person, reference_data.clone());

        // Fill gaps of direct subordinate.
        temp_reference_data = replace_dsubordinate(dsubordinate, temp_reference_data);

        // Push our reference to report.
        report.push_str(temp_reference_data.as_str());
        // Update our direct subordinate for next itteration.
        dsubordinate = person;
    }

    fs::write(output, report)?;

    Ok(())
}

/**
This function will make content report from person and template.

# Arguments
* `person` - A HashMap with key an value to replace in report.
* `template` - A string in wich we will fill gaps with data from person.
Note: function will own template. If you want to use it more times pass clone.
*/
fn replace(person: &Person, mut template: String) -> String {
    for pair in person {
        let (field, value) = pair;
        template = template.replace(format!("{{{}}}", field).as_str(), value.as_str());
    }

    template
}

/**
This function will make content report from person and template with
direct subordinate modifier.

# Arguments
* `person` - A HashMap with key an value to replace in report.
* `template` - A string in wich we will fill gaps with data from person.
Note: function will own template. If you want to use it multiple times pass clone.
*/
fn replace_dsubordinate(person: &Person, mut template: String) -> String {
    for pair in person {
        let (field, value) = pair;
        template = template.replace(
            format!("{{{}_dsubordinate}}", field).as_str(),
            value.as_str(),
        );
    }

    template
}
