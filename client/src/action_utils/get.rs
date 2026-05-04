use crate::{AuthRequest, ADDRESS};
use core_app::credentials::Credentials;
use core_app::types::{Accessories, Chain, Section, User};
use reqwest::blocking::Client;
use serde::de::DeserializeOwned;

fn process_get_list<T: DeserializeOwned>(
    target: &mut Vec<T>,
    error_message: &mut Option<String>,
    credentials: Credentials,
    client: &mut Client,
    addr: &str,
    endpoint: &str,
) {
    let url = format!("http://{addr}/{endpoint}/get");

    let request_body = AuthRequest {
        credentials,
        payload: (),
    };

    match client.get(url).json(&request_body).send() {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<Vec<T>>() {
                    Ok(list) => {
                        *target = list;
                    }
                    Err(e) => {
                        *error_message = Some(format!("Failed to parse response: {}", e));
                    }
                }
            }
        }
        Err(e) => {
            *error_message = Some(format!("Error during GET request: {}", e));
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
        ADDRESS,
        "section",
    );
}

pub fn get_chains(
    target: &mut Vec<Chain>,
    error_message: &mut Option<String>,
    credentials: Credentials,
    client: &mut Client,
) {
    process_get_list(target, error_message, credentials, client, ADDRESS, "chain");
}

pub fn get_users(
    target: &mut Vec<User>,
    error_message: &mut Option<String>,
    credentials: Credentials,
    client: &mut Client,
) {
    process_get_list(target, error_message, credentials, client, ADDRESS, "user");
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
        ADDRESS,
        "accessories",
    );
}
