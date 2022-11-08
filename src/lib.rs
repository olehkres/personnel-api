//! Personnel-api is a backend inside Personnel Manager crate group.
//!
//! If you just want to use Personnel Manager look for either personnel-cli or personnel-gui.

/// Working with data from SQLite database.
pub mod db;
/// Defined what person is.
pub mod person;
/// Making report.
pub mod report;

#[cfg(test)]
mod tests {}
