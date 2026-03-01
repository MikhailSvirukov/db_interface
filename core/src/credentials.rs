use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::io;
use std::io::ErrorKind;
use std::str::FromStr;

#[derive(
    Default, ToPrimitive, FromPrimitive, Debug, Serialize, Deserialize, Eq, PartialEq, Clone,
)]
pub enum AccessLevel {
    #[default]
    None = 0, //shouldn't be user for nothing but default
    User = 1,
    Economist = 2,
    Manager = 3,
    Administrator = 4,
    Programmer = 5,
}

impl Display for AccessLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            AccessLevel::User => "Пользователь".to_string(),
            AccessLevel::Economist => "Экономист".to_string(),
            AccessLevel::Manager => "Менеджер".to_string(),
            AccessLevel::Administrator => "Администратор".to_string(),
            AccessLevel::Programmer => "Программист".to_string(),
            AccessLevel::None => "Нет".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl FromStr for AccessLevel {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Пользователь" => Ok(AccessLevel::User),
            "Экономист" => Ok(AccessLevel::Economist),
            "Менеджер" => Ok(AccessLevel::Manager),
            "Администратор" => Ok(AccessLevel::Administrator),
            "Программист" => Ok(AccessLevel::Programmer),
            _ => Err(io::Error::new(
                ErrorKind::InvalidInput,
                "Invalid access level",
            )),
        }
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
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
