use crate::request;
use core_app::credentials::Credentials;
use core_app::requests::Id;
use reqwest::blocking::Client;

pub fn process_delete(
    delete_flag: &mut (bool, Option<Id>),
    error_message: &mut Option<String>,
    credentials: Credentials,
    client: &mut Client,
    addr: &str,
    endpoint: &str,
) {
    if let (true, Some(id)) = *delete_flag {
        match request::send_auth_request(
            credentials,
            client,
            error_message,
            "DELETE",
            format!("/{endpoint}").as_str(),
            vec![id],
            addr,
        ) {
            Ok(_) => {
                error_message.take();
            }
            Err(err) => {
                *error_message = Some(format!("Error sending delete message: {}", err));
            }
        }

        *delete_flag = (false, None);
    }
}
