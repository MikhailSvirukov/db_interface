use crate::{request, UpdateStatus};
use core_app::credentials::Credentials;
use reqwest::blocking::Client;

pub fn process_update<T, R: serde::Serialize + Send + Sync + 'static, FParse>(
    updater: T,
    change_flag: &mut bool,
    updater_mode: &mut UpdateStatus,
    error_message: &mut Option<String>,
    parse_fn: FParse,
    addr: &str,
    endpoint: &str,
    credentials: Credentials,
    client: &mut Client,
) where
    FParse: Fn(T) -> Result<R, String>,
{
    if !*change_flag {
        return;
    }

    match parse_fn(updater) {
        Ok(parsed_payload) => match updater_mode {
            UpdateStatus::None => {
                unreachable!()
            }
            UpdateStatus::Update => {
                match request::send_auth_request(
                    credentials,
                    client,
                    error_message,
                    "PUT",
                    format!("/{endpoint}").as_str(),
                    parsed_payload,
                    addr,
                ) {
                    Ok(_) => {
                        error_message.take();
                    }
                    Err(err) => {
                        *error_message = Some(format!("Error sending update message: {}", err));
                    }
                }
            }
            UpdateStatus::Add => {
                match request::send_auth_request(
                    credentials,
                    client,
                    error_message,
                    "POST",
                    format!("/{endpoint}").as_str(),
                    parsed_payload,
                    addr,
                ) {
                    Ok(_) => {
                        error_message.take();
                    }
                    Err(err) => {
                        *error_message = Some(format!("Error sending add message: {}", err));
                    }
                }
            }
        },
        Err(err) => {
            *error_message = Some(format!("Error parsing message: {}", err));
        }
    }
    *change_flag = false;
    *updater_mode = UpdateStatus::None;
}
