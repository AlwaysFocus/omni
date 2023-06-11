use anyhow::{anyhow, Result, Ok};
use colored::Colorize;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::fmt::Debug;

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
    AddCaseCommentBody(AddCaseCommentInput),
    GetLastCommentBody(GetLastCommentInput),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiResponse {
    UpdateQuoteBody(UpdateQuoteResponse),
    CompleteTaskBody(CompleteTaskResponse),
    CaseStatusBody(CaseStatusResponse),
    AddCaseCommentBody(AddCaseCommentResponse),
    GetLastCommentBody(GetLastCommentResponse),
}

#[derive(Serialize, Debug)]
pub struct GetLastCommentInput {
    #[serde(rename = "CaseNum")]
    case_num: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetLastCommentResponse {
    #[serde(rename = "Error")]
    error: bool,
    #[serde(rename = "Message")]
    message: Option<String>,
    #[serde(rename = "Comment")]
    comment: Option<String>,
}


#[derive(Serialize, Debug)]
pub struct AddCaseCommentInput {
    #[serde(rename = "CaseNum")]
    case_num: u32,
    #[serde(rename = "Comment")]
    comment: String,
}

impl AddCaseCommentInput {
    pub fn new(case_num: u32, comment: &str) -> Self {
        Self {
            case_num,
            comment: comment.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddCaseCommentResponse {
    #[serde(rename = "Error")]
    error: bool,
    #[serde(rename = "Message")]
    message: Option<String>,
}

#[derive(Serialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateQuoteResponse {
    #[serde(rename = "Error")]
    error: bool,
    #[serde(rename = "Message")]
    message: String,
}

#[derive(Serialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Debug)]
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

pub async fn add_case_comment(case_num: u32, comment: &str) -> Result<()> {

    let add_comment_input = AddCaseCommentInput {
        case_num,
        comment: comment.to_string()
    };

    let _result = send_request::<AddCaseCommentInput, AddCaseCommentResponse>(Some(RequestBodyType::AddCaseCommentBody(add_comment_input)), "efx/100/Omni/AddCaseComment").await?;

    println!(
        "{}",
        "Comment Added to Case".bright_green().bold(),
    );

    Ok(())
}

pub async fn get_last_case_comment(case_num: u32) -> Result<()> {
    // Retrieve environment variables
    let api_key = env::var("EPICOR_API_KEY").map_err(|_| anyhow!("EPICOR_API_KEY must be set"))?;
    let basic_auth =
        env::var("EPICOR_BASIC_AUTH").map_err(|_| anyhow!("EPICOR_BASIC_AUTH must be set"))?;
    let base_url =
        env::var("EPICOR_BASE_URL").map_err(|_| anyhow!("EPICOR_BASE_URL must be set"))?;

    // Prepare the HTTP client.
    let client = Client::new();

    // Prepare the JSON payload.
    let last_case_comment_input = GetLastCommentInput {
        case_num
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
    let url = format!("{}/api/v2/efx/100/Omni/GetLastComment", base_url);

    // Send the request and get the response.
    let resp: Response = client
        .post(&url)
        .headers(headers)
        .json(&last_case_comment_input)
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
    let last_comment_response: GetLastCommentResponse = resp.json().await?;

    // Check for errors.
    if last_comment_response.error {
        return Err(anyhow!("Error: {}", last_comment_response.message.unwrap_or("Unknown Error".to_string())));
    }

    println!("{}", "Last Comment".bright_green().bold().underline());

    println!(
        "{}",
        last_comment_response.comment.unwrap_or("No comments".to_string()).bright_red(),
    );

    Ok(())
}

async fn send_request<R: Serialize, S: for<'de> Deserialize<'de>>(
    req_body: Option<RequestBodyType>,
    api_endpoint: &str,
) -> Result<()> {
    // Retrieve environment variables
    let api_key = env::var("EPICOR_API_KEY")?;
    let basic_auth = env::var("EPICOR_BASIC_AUTH")?;
    let base_url = env::var("EPICOR_BASE_URL")?;

    // Prepare the HTTP client.
    let client = Client::new();

    // Prepare the JSON payload.
    let body = match req_body {
        Some(RequestBodyType::UpdateQuoteBody(update_quote_input)) => {
            serde_json::to_value(update_quote_input)?
        }
        Some(RequestBodyType::AddCaseCommentBody(add_comment_input)) => {
            serde_json::to_value(add_comment_input)?

        }
        Some(RequestBodyType::CaseStatusBody(case_status_input)) => {
            serde_json::to_value(case_status_input)?
        }
        Some(RequestBodyType::CompleteTaskBody(update_case_quote_input)) => {
            serde_json::to_value(update_case_quote_input)?
        }
        Some(RequestBodyType::GetLastCommentBody(get_last_comment_input)) => {
            serde_json::to_value(get_last_comment_input)?
        }
        // Handle other types...
        _ => return Err(anyhow!("Unsupported request body type")),
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
            return Err(anyhow!("The Omni function library is not published in Epicor. Please publish the function library and try again."));
        }
        return Err(anyhow!(format!("Error: {}", resp.status())));
    }

    // Deserialize the response. Make sure it is deserialized as the type passed in by the user.
    let api_response = serde_json::from_str::<ApiResponse>(&resp.text().await?)?;

    // print response
    println!("api_response: {:?}", api_response);

    match api_response {
        ApiResponse::UpdateQuoteBody(update_quote_response) => {
            // Check for errors.
            if update_quote_response.error {
                return Err(anyhow!(format!("Error: {}", update_quote_response.message)));
            }
        }
        ApiResponse::AddCaseCommentBody(add_comment_response) => {
            // Check for errors. Note that response.message can be null and is optional
            if add_comment_response.error {
                return Err(anyhow!(format!("Error: {}", add_comment_response.message.unwrap_or("".to_string()))));
            }
        }
        ApiResponse::CaseStatusBody(case_status_response) => {
            // Check for errors.
            if case_status_response.error {
                return Err(anyhow!(format!("Error: {}", case_status_response.message)));
            }
        }
        ApiResponse::CompleteTaskBody(complete_task_response) => {
            // Check for errors.
            if complete_task_response.error {
                return Err(anyhow!(format!("Error: {}", complete_task_response.message)));
            }
        }
        ApiResponse::GetLastCommentBody(get_last_comment_response) => {

            println!("get_last_comment_response: {:?}", get_last_comment_response);

            // Check for errors.
            if get_last_comment_response.error {
                return Err(anyhow!(format!("Error: {}", get_last_comment_response.message.unwrap_or("".to_string()))));
            }

            // print the comment
            println!("Last Comment: {}", get_last_comment_response.comment.unwrap_or("".to_string()));
        }
    }



    Ok(())
}
