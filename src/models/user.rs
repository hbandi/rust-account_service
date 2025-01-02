use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub name: String,
    pub ssn_no: String,
    pub country: String,
    pub age: u8,
    pub address: String
}


impl User {
    pub fn new(name: &String, ssn_no: &String, country: &String, age: u8, address: &String) -> Self {
        User {
            name: name.to_string(),
            ssn_no: ssn_no.to_string(),
            country: country.to_string(),
            age,
            address: address.to_string()
        }
    }

    pub fn to_string(&self) -> String {
        format!("User: {:?}", self)
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} from {}", self.name, self.country)
    }
}