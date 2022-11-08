use std::{collections::HashMap, path::Path};

use crate::person::Person;
use rusqlite::{types::ValueRef, Connection, Params, Result};

/**
This function will make a personchain started from lowest person and to the
highest commander.

# Arguments
* `id` - id in database corresponding to person.
* `db_path` - path to SQLite database with personnel schema.
 */
pub fn make_personchain(id: u32, db_path: &Path) -> Result<Vec<Person>> {
    let conn = Connection::open(db_path)?;

    let mut personchain: Vec<Person> = Vec::new();
    personchain.push(get_person_by_id(id, &conn)?);

    loop {
        // Get person suprime and add it to chain until we get it up to the end.
        match get_person_sup_by_id(
            personchain.last().unwrap()["person_id"]
                .parse::<u32>()
                .unwrap(),
            &conn,
        ) {
            Ok(person) => personchain.push(person),
            Err(_) => break,
        };
    }

    Ok(personchain)
}

/**
This function will get one row from database.

# Arguments
* `conn` - SQLite database connection defined in rusqlite library.
* `request` - string slice wich is valid SQL request.
* `params` - array of params for request.
 */
fn make_onerow_request<P>(
    conn: &Connection,
    request: &str,
    params: P,
) -> Result<HashMap<String, String>>
where
    P: Params,
{
    let mut stmt = conn.prepare(request)?;

    let column_names: Vec<String> = stmt.column_names().into_iter().map(String::from).collect();

    let mut rows = stmt.query(params)?;

    while let Some(row) = rows.next()? {
        let mut person = HashMap::<String, String>::new();

        for column in &column_names {
            let cell = match row.get_ref_unwrap(column.as_str()) {
                ValueRef::Integer(i) => i.to_string(),
                ValueRef::Real(f) => f.to_string(),
                ValueRef::Text(t) => String::from_utf8(t.to_vec()).unwrap(),
                ValueRef::Blob(b) => String::from_utf8(b.to_vec()).unwrap(),
                ValueRef::Null => String::new(),
            };

            person.insert(column.clone(), cell);
        }

        return Ok(person);
    }

    // If we get no rows return error.
    Err(rusqlite::Error::InvalidQuery)
}

/**
This function will get and return commander of person by id.

#Arguments
 * `id` - id in database corespoding to person of wich commander we want to know.
 * `conn` - SQLite database connection defined in rusqlite library.
 */
fn get_person_sup_by_id(id: u32, conn: &Connection) -> Result<Person> {
    make_onerow_request(
        conn,
        "SELECT * \
         FROM personnel \
        WHERE person_position_id = (\
            SELECT position_sup_id \
             FROM positions \
            WHERE position_id = (\
                SELECT person_position_id \
                 FROM person \
                WHERE person_id = ?));",
        [id],
    )
}

/**
This function will get and return person by id.

#Arguments
 * `id` - id in database corespoding to person.
 * `conn` - SQLite database connection defined in rusqlite library.
 */
fn get_person_by_id(id: u32, conn: &Connection) -> Result<Person> {
    make_onerow_request(conn, "SELECT * FROM personnel WHERE person_id = ?", [id])
}
