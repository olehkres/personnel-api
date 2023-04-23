use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum PersonnelError {
    XmlError(minidom::Error),
    SqlError(rusqlite::Error),
    EmptySqlResult,
    EmptyXml,
    FromUtf8Error(FromUtf8Error),
}

impl std::fmt::Display for PersonnelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PersonnelError::XmlError(err) => write!(f, "invalid xml. {}", err),
            PersonnelError::SqlError(err) => write!(f, "invalid db. {}", err),
            PersonnelError::EmptySqlResult => write!(f, "invalid sql request. empty row."),
            PersonnelError::EmptyXml => write!(f, "invalid xml. there is no such object."),
            PersonnelError::FromUtf8Error(err) => write!(f, "parsing failed. {}", err),
        }
    }
}
