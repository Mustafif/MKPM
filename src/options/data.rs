use crate::DATA;
use serde::{Deserialize, Serialize};
use std::fs;
/// This struct is the core of the package manager and is
/// what interacts with the data.json to search, get, applications/books
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    /// Name of application
    pub name: String,
    /// App Type (Whether Book/Application)
    pub app_type: AppType,
    /// ID used to retrieve applications
    pub id: String,
    /// Version of book (default 1.0) or application
    pub version: String,
    /// Url to get book/application
    pub url: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Used to determine what type of application the data is
pub enum AppType {
    /// All books are under here
    Book,
    /// Any program, desktop or other sorts are here
    Application,
    /// Any undetermined, or not ready applications go under here
    Unknown,
}
impl Data {
    pub fn load_data() -> Result<Vec<Self>, ()> {
        /// The point of this function is to load a Vector of the Data struct
        /// from the data.json file
        let s = match fs::read_to_string(DATA) {
            Ok(file) => file,
            Err(_e) => "Error in reading file to string".to_owned(),
        };
        /// Note that most temporary strings, and vecs will be s & v
        let v: Vec<Self> = serde_json::from_str(&s).unwrap();
        Ok(v)
    }
}

impl AppType {
    pub fn is_what(&self) -> Result<String, ()> {
        match self {
            self::AppType::Book => Ok("Book".to_owned()),
            self::AppType::Application => Ok("Application".to_owned()),
            self::AppType::Unknown => Ok("Unknown".to_owned()),
        }
    }
    fn set(app_type: String) -> Self {
        match app_type.as_str() {
            "Book" => Self::Book,
            "Application" => Self::Application,
            _ => Self::Unknown,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_is_what() {
        let data = Data {
            name: "".to_owned(),
            url: "".to_owned(),
            version: "".to_owned(),
            id: "".to_owned(),
            app_type: AppType::set("Book".to_owned()),
        };
        assert_eq!(data.app_type.is_what().unwrap(), "Book".to_owned())
    }
}
