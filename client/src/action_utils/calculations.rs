use crate::{AuthRequest, SelectBlockHolder, ADDRESS};
use core_app::credentials::Credentials;
use core_app::replies::Calculation;
use core_app::requests::SelectedBlock;
use reqwest::blocking::Client;
use serde::de::DeserializeOwned;

fn process_get_single<T: DeserializeOwned + ToString>(
    target: &mut Option<String>,
    payload: impl serde::Serialize,
    error_message: &mut Option<String>,
    credentials: Credentials,
    client: &mut Client,
    addr: &str,
    endpoint: &str,
) {
    let url = format!("http://{addr}/{endpoint}");

    let request_body = AuthRequest {
        credentials,
        payload,
    };

    match client.post(url).json(&request_body).send() {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<T>() {
                    Ok(value) => {
                        *target = Some(value.to_string());
                    }
                    Err(e) => {
                        *error_message = Some(format!("Failed to parse response: {}", e));
                    }
                }
            }
        }
        Err(e) => {
            *error_message = Some(format!("Error during POST request: {}", e));
        }
    }
}

pub fn get_calculations(
    calculation_sum: &mut Option<String>,
    selected_block: &Vec<SelectBlockHolder>,
    error_message: &mut Option<String>,
    credentials: Credentials,
    client: &mut Client,
) {
    let raw_blocks = selected_block
        .iter()
        .map(|s| s.selected_block.clone())
        .collect::<Vec<SelectedBlock>>();
    process_get_single::<Calculation>(
        calculation_sum,
        raw_blocks,
        error_message,
        credentials,
        client,
        ADDRESS,
        "calculation",
    );
}
