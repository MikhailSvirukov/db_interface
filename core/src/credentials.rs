use std::io;
use std::io::ErrorKind;
use std::str::FromStr;
use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};

#[derive(ToPrimitive, FromPrimitive, Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub enum AccessLevel {
    User = 0,
    Economist = 1,
    Manager = 2,
    Administrator = 3,
    Programmer = 4,
}

impl FromStr for AccessLevel {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "User" => Ok(AccessLevel::User),
            "Economist" => Ok(AccessLevel::Economist),
            "Manager" => Ok(AccessLevel::Manager),
            "Administrator" => Ok(AccessLevel::Administrator),
            "Programmer" => Ok(AccessLevel::Programmer),
            _ => Err(io::Error::new(ErrorKind::InvalidInput, "Invalid access level")),
        }
    }
}

#[derive(Debug,Serialize, Deserialize, Clone)]
pub struct Credentials {
    pub login: String,
    pub password: String,
    pub access_level: AccessLevel,
}

#[allow(dead_code)]
trait CredentialsProvider {
    fn generate_login() -> String;
    fn generate_password() -> String;
}

#[allow(dead_code, unused_variables)]
fn add_new_user_to_db(db_url: &str /* tmp */, access_level: AccessLevel) -> io::Result<()> {
    todo!()
}

