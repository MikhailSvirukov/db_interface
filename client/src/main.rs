use reqwest::Error;
use core_app::types::{AuthRequest, Chain, Section, User, Type, SideMaterial, ChainMaterial};
use core_app::credentials::Credentials;
use serde::{Deserialize, Serialize};

const BASE_URL: &str = "http://127.0.0.1:3000";

#[tokio::main]
async fn main() -> Result<(), Error> {
    let admin_credentials = Credentials {
        login: "admin".to_string(),
        password: "adminpass".to_string(),
    };

    let user_credentials = Credentials {
        login: "user".to_string(),
        password: "userpass".to_string(),
    };

    // Example usage for Sections
    let new_section = Section {
        section_type: Type::Driving,
        width: 10,
        length: 100,
        price: 500,
        is_magnet: true,
        material_sides: SideMaterial::Steel,
        radius: 0,
        angle: 90,
        chains: vec![],
    };
    add_section(admin_credentials.clone(), new_section).await?;
    println!("Added section");

    let all_sections = get_all_sections(user_credentials.clone()).await?;
    println!("All sections: {:?}", all_sections);

    // Example usage for Chains
    let new_chain = Chain {
        chain_type: Type::Finite,
        material: ChainMaterial::Steel,
        width: 5,
        price: 100,
        is_magnet: false,
        name: "Test Chain".to_string(),
    };
    add_chain(admin_credentials.clone(), new_chain).await?;
    println!("Added chain");

    let all_chains = get_all_chains(user_credentials.clone()).await?;
    println!("All chains: {:?}", all_chains);

    // Example usage for Users
    let new_user = User {
        hash: "somehash".to_string(),
        name: "John Doe".to_string(),
        email: "john.doe@example.com".to_string(),
        phone: "1234567890".to_string(),
    };
    add_user(admin_credentials.clone(), new_user).await?;
    println!("Added user");

    let all_users = get_all_users(user_credentials.clone()).await?;
    println!("All users: {:?}", all_users);

    Ok(())
}

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
