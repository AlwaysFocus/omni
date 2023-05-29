use anyhow::{anyhow, Result};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;

#[derive(Serialize)]
pub struct CompleteTaskInput {
    #[serde(rename = "CaseNum")]
    case_num: u32,
    #[serde(rename = "AssignNextToName")]
    assign_next_to_name: String,
}

#[derive(Deserialize)]
pub struct CompleteTaskResponse {
    #[serde(rename = "Error")]
    error: bool,
    #[serde(rename = "Message")]
    message: String,
    #[serde(rename = "HasActiveTask")]
    has_active_task: bool,
    #[serde(rename = "AuthorizedToCompleteTask")]
    authorized_to_complete_task: bool,
    #[serde(rename = "MultipleSalesRepMatches")]
    multiple_sales_rep_matches: bool,
    #[serde(rename = "NoSalesRepMatch")]
    no_sales_rep_match: bool,
}

pub async fn send_complete_task(case_num: u32, assign_next_to_name: &str) -> Result<()> {

    // Retrieve environment variables
    let api_key = env::var("EPICOR_API_KEY").map_err(|_| anyhow!("EPICOR_API_KEY must be set"))?;
    let basic_auth =
        env::var("EPICOR_BASIC_AUTH").map_err(|_| anyhow!("EPICOR_BASIC_AUTH must be set"))?;
    let base_url =
        env::var("EPICOR_BASE_URL").map_err(|_| anyhow!("EPICOR_BASE_URL must be set"))?;

    // Prepare the HTTP client.
    let client = Client::new();

    // Prepare the JSON payload.
    let complete_task_input = CompleteTaskInput {
        case_num,
        assign_next_to_name: assign_next_to_name.to_string(),
    };

    // Prepare the headers.
    let mut headers = HeaderMap::new();
    headers.insert("X-API-Key", HeaderValue::from_str(&api_key)?);
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&basic_auth)?);
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/json; charset=utf-8"),
    );

    // Construct the URL
    // TODO: Make company dynamic
    let url = format!("{}/api/v2/efx/100/Omni/CompleteTask", base_url);

    // Send the request and get the response.
    let resp: Response = client
        .post(&url)
        .headers(headers)
        .json(&complete_task_input)
        .send()
        .await?;

    // Check to see if the response was successful.
    if !resp.status().is_success() {
        // if the error is 404, this means that the function library is likely not published
        if resp.status().as_u16() == 404 {
            return Err(anyhow!(
                "Error: {}",
                "The Omni function library is not published in Epicor. Please publish the function library and try again."
            ));
        }
        return Err(anyhow!("Error: {}", resp.status()));
    }

    // Deserialize the response.
    let complete_task_response: CompleteTaskResponse = resp.json().await?;

    // Check for errors.
    if complete_task_response.error {
        return Err(anyhow!("Error: {}", complete_task_response.message));
    }

    Ok(())
}
