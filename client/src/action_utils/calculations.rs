use crate::{request, SelectBlockHolder};
use core_app::credentials::Credentials;
use core_app::replies::Calculation;
use core_app::requests::SelectedBlock;
use reqwest::blocking::Client;

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
    match request::post_auth_request::<_, Calculation>(
        credentials,
        client,
        error_message,
        "/calculations",
        raw_blocks,
        crate::ADDRESS,
    ) {
        Ok(calc) => {
            *calculation_sum = Some(calc.to_string());
        }
        Err(e) => {
            *error_message = Some(e);
        }
    }
}