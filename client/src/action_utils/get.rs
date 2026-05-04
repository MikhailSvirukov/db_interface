use crate::request;
use core_app::credentials::Credentials;
use core_app::types::{Accessories, Chain, Section, User};
use reqwest::blocking::Client;

fn process_get_list<T>(
    target: &mut Vec<T>,
    error_message: &mut Option<String>,
    credentials: Credentials,
    client: &mut Client,
    addr: &str,
    endpoint: &str,
) where
    T: serde::de::DeserializeOwned,
{
    match request::get_auth_request::<Vec<T>>(
        credentials,
        client,
        error_message,
        format!("/{endpoint}").as_str(),
        addr,
    ) {
        Ok(list) => {
            *target = list;
        }
        Err(e) => {
            *error_message = Some(e);
        }
    }
}

pub fn get_section(
    target: &mut Vec<Section>,
    error_message: &mut Option<String>,
    credentials: Credentials,
    client: &mut Client,
) {
    process_get_list(
        target,
        error_message,
        credentials,
        client,
        crate::ADDRESS,
        "sections",
    );
}

pub fn get_chains(
    target: &mut Vec<Chain>,
    error_message: &mut Option<String>,
    credentials: Credentials,
    client: &mut Client,
) {
    process_get_list(target, error_message, credentials, client, crate::ADDRESS, "chains");
}

pub fn get_users(
    target: &mut Vec<User>,
    error_message: &mut Option<String>,
    credentials: Credentials,
    client: &mut Client,
) {
    process_get_list(target, error_message, credentials, client, crate::ADDRESS, "users");
}

pub fn get_accessories(
    target: &mut Vec<Accessories>,
    error_message: &mut Option<String>,
    credentials: Credentials,
    client: &mut Client,
) {
    process_get_list(
        target,
        error_message,
        credentials,
        client,
        crate::ADDRESS,
        "accessories",
    );
}