use core_app::credentials::Credentials;
use core_app::types::AuthRequest;
use reqwest::blocking::Client;
use serde::de::DeserializeOwned;

pub fn send_auth_request<T: serde::Serialize + Send + Sync + 'static>(
    credentials: Credentials,
    client: &mut Client,
    error_message: &mut Option<String>,
    method: &str,
    endpoint: &str,
    payload: T,
    addr: &str,
) -> Result<(), String> {
    let auth_request = AuthRequest {
        credentials,
        payload,
    };

    let request = match method {
        "GET" => client.get(format!("http://{addr}{endpoint}")),
        "POST" => client.post(format!("http://{addr}{endpoint}")),
        "PUT" => client.put(format!("http://{addr}{endpoint}")),
        "DELETE" => client.delete(format!("http://{addr}{endpoint}")),
        _ => return Err("Unsupported method".to_string()),
    }
    .json(&auth_request);

    match request.send() {
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

pub fn get_auth_request<T: DeserializeOwned>(
    credentials: Credentials,
    client: &mut Client,
    error_message: &mut Option<String>,
    endpoint: &str,
    addr: &str,
) -> Result<T, String> {
    let auth_request = AuthRequest {
        credentials,
        payload: (),
    };

    match client
        .get(format!("http://{addr}{endpoint}"))
        .json(&auth_request)
        .send()
    {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<T>() {
                    Ok(data) => {
                        error_message.take();
                        Ok(data)
                    }
                    Err(e) => Err(format!("Failed to parse response: {}", e)),
                }
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

pub fn post_auth_request<T: serde::Serialize + Send + Sync + 'static, R: DeserializeOwned>(
    credentials: Credentials,
    client: &mut Client,
    error_message: &mut Option<String>,
    endpoint: &str,
    payload: T,
    addr: &str,
) -> Result<R, String> {
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
                match response.json::<R>() {
                    Ok(data) => {
                        error_message.take();
                        Ok(data)
                    }
                    Err(e) => Err(format!("Failed to parse response: {}", e)),
                }
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
