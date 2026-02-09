use reqwest::Error;
use core_app::types::{AuthRequest, Chain, Section, User, Type, SideMaterial, ChainMaterial};
use core_app::credentials::Credentials;
use serde::{Deserialize, Serialize};

const BASE_URL: &str = "http://127.0.0.1:3000";

use eframe::egui;

async fn add_section(credentials: Credentials, section: Section) -> Result<(), Error> {
    let client = reqwest::Client::new();
    let auth_request = AuthRequest { credentials, payload: section };
    let res = client
        .post(&format!("{}/section", BASE_URL))
        .json(&auth_request)
        .send()
        .await?;
    res.error_for_status()?.text().await?;
    Ok(())
}

async fn get_all_sections(credentials: Credentials) -> Result<Vec<Section>, Error> {
    let client = reqwest::Client::new();
    let auth_request = AuthRequest { credentials, payload: () };
    let res = client
        .get(&format!("{}/sections", BASE_URL))
        .json(&auth_request)
        .send()
        .await?;
    res.json::<Vec<Section>>().await
}

async fn add_chain(credentials: Credentials, chain: Chain) -> Result<(), Error> {
    let client = reqwest::Client::new();
    let auth_request = AuthRequest { credentials, payload: chain };
    let res = client
        .post(&format!("{}/chain", BASE_URL))
        .json(&auth_request)
        .send()
        .await?;
    res.error_for_status()?.text().await?;
    Ok(())
}

async fn get_all_chains(credentials: Credentials) -> Result<Vec<Chain>, Error> {
    let client = reqwest::Client::new();
    let auth_request = AuthRequest { credentials, payload: () };
    let res = client
        .get(&format!("{}/chains", BASE_URL))
        .json(&auth_request)
        .send()
        .await?;
    res.json::<Vec<Chain>>().await
}

async fn add_user(credentials: Credentials, user: User) -> Result<(), Error> {
    let client = reqwest::Client::new();
    let auth_request = AuthRequest { credentials, payload: user };
    let res = client
        .post(&format!("{}/user", BASE_URL))
        .json(&auth_request)
        .send()
        .await?;
    res.error_for_status()?.text().await?;
    Ok(())
}

async fn get_all_users(credentials: Credentials) -> Result<Vec<User>, Error> {
    let client = reqwest::Client::new();
    let auth_request = AuthRequest { credentials, payload: () };
    let res = client
        .get(&format!("{}/users", BASE_URL))
        .json(&auth_request)
        .send()
        .await?;
    res.json::<Vec<User>>().await
}
