use anyhow::{anyhow, Result};
use colored::Colorize;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;

pub struct TimeEntry {
    employee_id: u32,
    labor_type: LaborType,
    project_id: Option<String>,
    wbs_phase_id: Option<String>,
    operation: Option<u32>,
    expense_code: Option<ExpenseCode>,
}

enum ExpenseCode {
    DirectLabor = 1,
    IndirectLabor,
}

enum LaborType {
    Indirect,
    Project,
    Production,
    Service,
    Setup,
}

impl LaborType {
    fn as_str(&self) -> &str {
        match self {
            LaborType::Indirect => "Indirect",
            LaborType::Project => "Project",
            LaborType::Production => "Production",
            LaborType::Service => "Service",
            LaborType::Setup => "Setup",
        }
    }
}

pub enum RequestBodyType {
    UpdateQuoteBody(UpdateQuoteInput),
    CompleteTaskBody(CompleteTaskInput),
    CaseStatusBody(CaseStatusInput),
}

pub enum ResponseBodyType {
    UpdateQuoteBody(UpdateQuoteResponse),
    CompleteTaskBody(CompleteTaskResponse),
    CaseStatusBody(CaseStatusResponse),
}

#[derive(Serialize)]
pub struct UpdateQuoteInput {
    #[serde(rename = "CaseNum")]
    case_num: u32,
    #[serde(rename = "Qty")]
    new_quantity: f32,
}

impl UpdateQuoteInput {
    pub fn new(case_num: u32, new_quantity: f32) -> Self {
        Self {
            case_num,
            new_quantity,
        }
    }
}

#[derive(Deserialize)]
pub struct UpdateQuoteResponse {
    #[serde(rename = "Error")]
    error: bool,
    #[serde(rename = "Message")]
    message: String,
}

#[derive(Serialize)]
pub struct CompleteTaskInput {
    #[serde(rename = "CaseNum")]
    case_num: u32,
    #[serde(rename = "AssignNextToName")]
    assign_next_to_name: String,
}

impl CompleteTaskInput {
    pub fn new(case_num: u32, assign_next_to_name: &str) -> Self {
        Self {
            case_num,
            assign_next_to_name: assign_next_to_name.to_string(),
        }
    }
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

#[derive(Serialize)]
pub struct CaseStatusInput {
    #[serde(rename = "CaseNum")]
    case_num: u32,
}

impl CaseStatusInput {
    pub fn new(case_num: u32) -> Self {
        Self { case_num }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CaseStatusResponse {
    #[serde(rename = "Error")]
    pub error: bool,

    #[serde(rename = "Message")]
    pub message: String,

    #[serde(rename = "ProjectID")]
    pub project_id: String,

    #[serde(rename = "CaseDescription")]
    pub case_description: String,

    #[serde(rename = "PartNum")]
    pub part_num: String,

    #[serde(rename = "Qty")]
    pub qty: f64,

    #[serde(rename = "UnitPrice")]
    pub unit_price: f64,

    #[serde(rename = "CaseOwner")]
    pub case_owner: String,

    #[serde(rename = "InternalContact")]
    pub internal_contact: String,

    #[serde(rename = "CaseContact")]
    pub case_contact: String,

    #[serde(rename = "CurrentTask")]
    pub current_task: String,

    #[serde(rename = "CurrentTaskAssignedTo")]
    pub current_task_assigned_to: String,

    #[serde(rename = "RequestedDelivery")]
    pub requested_delivery: String,

    #[serde(rename = "StartDate")]
    pub start_date: String,

    #[serde(rename = "ExpectedDeliveryDate")]
    pub expected_delivery_date: String,

    #[serde(rename = "Developer")]
    pub developer: String,

    #[serde(rename = "WBSPhaseID")]
    pub wbs_phase_id: String,

    #[serde(rename = "WBSPhaseOp")]
    pub wbs_phase_op: i32,

    #[serde(rename = "EstimatedHours")]
    pub estimated_hours: f64,

    #[serde(rename = "HoursScheduled")]
    pub hours_scheduled: f64,

    #[serde(rename = "HoursApplied")]
    pub hours_applied: f64,

    #[serde(rename = "BilledPercent")]
    pub billed_percent: f64,
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

pub async fn get_case_status(case_num: u32) -> Result<()> {
    // Retrieve environment variables
    let api_key = env::var("EPICOR_API_KEY").map_err(|_| anyhow!("EPICOR_API_KEY must be set"))?;
    let basic_auth =
        env::var("EPICOR_BASIC_AUTH").map_err(|_| anyhow!("EPICOR_BASIC_AUTH must be set"))?;
    let base_url =
        env::var("EPICOR_BASE_URL").map_err(|_| anyhow!("EPICOR_BASE_URL must be set"))?;

    // Prepare the HTTP client.
    let client = Client::new();

    // Prepare the JSON payload.
    let complete_task_input = CaseStatusInput { case_num };

    // Prepare the headers.
    let mut headers = HeaderMap::new();
    headers.insert("X-API-Key", HeaderValue::from_str(&api_key)?);
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&basic_auth)?);
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/json; charset=utf-8"),
    );

    // Construct the URL
    let url = format!("{}/api/v2/efx/100/Omni/GetCaseStatus", base_url);

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
    let case_status_response: CaseStatusResponse = resp.json().await?;

    // Check for errors.
    if case_status_response.error {
        return Err(anyhow!("Error: {}", case_status_response.message));
    }

    print_case_status(&case_num, case_status_response);

    Ok(())
}

fn print_case_status(case_num: &u32, case_status_response: CaseStatusResponse) {
    // Case Num
    println!("{} {}", "Case Number:".red().bold().underline(), case_num);
    // Case Owner
    println!(
        "{} {}",
        "Case Owner:".red().bold().underline(),
        case_status_response.case_owner
    );
    // Case Contact
    println!(
        "{} {}",
        "Case Contact:".red().bold().underline(),
        case_status_response.case_contact
    );
    // Internal Contact
    println!(
        "{} {}",
        "Internal Contact:".red().bold().underline(),
        case_status_response.internal_contact
    );
    // Case Description
    println!(
        "{} {}",
        "Case Description:".red().bold().underline(),
        case_status_response.case_description
    );
    // Project
    println!(
        "{} {}",
        "Project:".red().bold().underline(),
        case_status_response.project_id
    );
    // Part Num
    println!(
        "{} {}",
        "Part Num:".red().bold().underline(),
        case_status_response.part_num
    );
    // Unit Price
    println!(
        "{} {}",
        "Unit Price:".red().bold().underline(),
        case_status_response.unit_price
    );
    // Quantity
    println!(
        "{} {}",
        "Quantity:".red().bold().underline(),
        case_status_response.qty
    );
    // Phase
    println!(
        "{} {}",
        "Phase:".red().bold().underline(),
        case_status_response.wbs_phase_id
    );
    // Op
    println!(
        "{} {}",
        "Op:".red().bold().underline(),
        case_status_response.wbs_phase_op
    );
    // CurrentTask
    println!(
        "{} {}",
        "Current Task:".red().bold().underline(),
        case_status_response.current_task
    );
    // AssignedTo
    println!(
        "{} {}",
        "Assigned To:".red().bold().underline(),
        case_status_response.current_task_assigned_to
    );
    // Case Developer
    println!(
        "{} {}",
        "Case Developer:".red().bold().underline(),
        case_status_response.developer
    );
    // Request Date
    println!(
        "{} {}",
        "Request Date:".red().bold().underline(),
        case_status_response.requested_delivery
    );
    // Start Date
    println!(
        "{} {}",
        "Start Date:".red().bold().underline(),
        case_status_response.start_date
    );
    // Expected Delivery Date
    println!(
        "{} {}",
        "Expected Delivery Date:".red().bold().underline(),
        case_status_response.expected_delivery_date
    );
    // Estimated Hours
    println!(
        "{} {}",
        "Estimated Hours:".red().bold().underline(),
        case_status_response.estimated_hours
    );
    // Hours Scheduled
    println!(
        "{} {}",
        "Hours Scheduled:".red().bold().underline(),
        case_status_response.hours_scheduled
    );
    // Hours Applied
    println!(
        "{} {}",
        "Hours Applied:".red().bold().underline(),
        case_status_response.hours_applied
    );
    // Billed Percent
    println!(
        "{} {}",
        "Billed Percent:".red().bold().underline(),
        case_status_response.billed_percent
    );
}

pub async fn update_case_quote(case_num: u32, new_quantity: f32) -> Result<()> {
    // Retrieve environment variables
    let api_key = env::var("EPICOR_API_KEY").map_err(|_| anyhow!("EPICOR_API_KEY must be set"))?;
    let basic_auth =
        env::var("EPICOR_BASIC_AUTH").map_err(|_| anyhow!("EPICOR_BASIC_AUTH must be set"))?;
    let base_url =
        env::var("EPICOR_BASE_URL").map_err(|_| anyhow!("EPICOR_BASE_URL must be set"))?;

    // Prepare the HTTP client.
    let client = Client::new();

    // Prepare the JSON payload.
    let update_quote_input = UpdateQuoteInput {
        case_num,
        new_quantity,
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
    let url = format!("{}/api/v2/efx/100/Omni/UpdateCaseQuote", base_url);

    // Send the request and get the response.
    let resp: Response = client
        .post(&url)
        .headers(headers)
        .json(&update_quote_input)
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
    let update_quote_response: UpdateQuoteResponse = resp.json().await?;

    // Check for errors.
    if update_quote_response.error {
        return Err(anyhow!("Error: {}", update_quote_response.message));
    }

    println!(
        "{}",
        "Quote Updated and Attached to Case".bright_green().bold(),
    );

    Ok(())
}

pub async fn send_request<R: Serialize, S: for<'de> Deserialize<'de>>(
    req_body: Option<RequestBodyType>,
    api_endpoint: &str,
    _res_body: ResponseBodyType,
) -> Result<(), Box<dyn Error>> {
    // Retrieve environment variables
    let api_key = env::var("EPICOR_API_KEY")?;
    let basic_auth = env::var("EPICOR_BASIC_AUTH")?;
    let base_url = env::var("EPICOR_BASE_URL")?;

    // Prepare the HTTP client.
    let client = reqwest::Client::new();

    // Prepare the JSON payload.
    let body = match req_body {
        Some(RequestBodyType::CompleteTaskBody(body)) => body,
        // Handle other types...
        _ => return Err("Unsupported request body type".into()),
    };

    // Prepare the headers.
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "X-API-Key",
        reqwest::header::HeaderValue::from_str(&api_key)?,
    );
    headers.insert(
        reqwest::header::AUTHORIZATION,
        reqwest::header::HeaderValue::from_str(&basic_auth)?,
    );
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        reqwest::header::HeaderValue::from_static("application/json; charset=utf-8"),
    );

    // Construct the URL
    let url = format!("{}/api/v2/{}", base_url, api_endpoint);

    // Send the request and get the response.
    let resp = client
        .post(&url)
        .headers(headers)
        .json(&body)
        .send()
        .await?;

    // Check to see if the response was successful.
    if !resp.status().is_success() {
        // if the error is 404, this means that the function library is likely not published
        if resp.status().as_u16() == 404 {
            return Err("The Omni function library is not published in Epicor. Please publish the function library and try again.".into());
        }
        return Err(format!("Error: {}", resp.status()).into());
    }

    // Deserialize the response.
    match _res_body {
        ResponseBodyType::CompleteTaskBody(..) => {
            let complete_task_response: CompleteTaskResponse = resp.json().await?;

            // Check for errors.
            if complete_task_response.error {
                return Err(format!("Error: {}", complete_task_response.message).into());
            }
        }
        // Handle other response types...
        _ => return Err("Unsupported response body type".into()),
    }

    Ok(())
}
