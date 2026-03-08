use core_app::credentials::Credentials;
use core_app::types::AuthRequest;
use reqwest::blocking::Client;

pub fn send_auth_request<T: serde::Serialize + Send + Sync + 'static>(
    credentials: Credentials,
    client: &mut Client,
    error_message: &mut Option<String>,
    endpoint: &str,
    payload: T,
    addr: &str
) -> Result<(), String> {
    let auth_request = AuthRequest {
        credentials,
        payload,
    };

    match client
        .post(format!("http://{addr}{endpoint}"))
        .json(&auth_request)
        .send()
    {
        Ok(response) => {
            if response.status().is_success() {
                error_message.take();
                Ok(())
            } else {
                Err(format!(
                    "Server responded with an error: {:?}",
                    response.status()
                ))
            }
        }
        Err(e) => Err(format!("Failed to send request: {}", e)),
    }
}