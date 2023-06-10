mod args;
mod bitwarden;
mod epicor;
mod setup;

use crate::args::{
    BitwardenSubcommand, CaseSubcommand, EntityType, EpicorCommand, EpicorSubcommand,
};
use crate::bitwarden::{get_item, list_items};
use crate::epicor::{add_case_comment, get_case_status, send_complete_task, update_case_quote};
use crate::setup::setup;
use anyhow::{anyhow, Result};
use args::OmniArgs;
use clap::{arg, command, Command as ClapCommand, Parser, Subcommand};
use dotenv::dotenv;
use figlet_rs::FIGfont;
use regex::Regex;
use std::env;
use std::process::Command;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let args = OmniArgs::parse();

    match args.entity_type {
        EntityType::Bitwarden(bitwarden) => match bitwarden.subcommand {
            BitwardenSubcommand::List => {
                return list_items();
            }
            BitwardenSubcommand::Get(get) => {
                return get_item(&get.item_type, &get.name);
            }
            BitwardenSubcommand::Create(create) => {
                println!("Create");
            }
        },
        EntityType::Epicor(epicor) => match epicor.subcommand {
            EpicorSubcommand::Case(case) => match case.subcommand {
                CaseSubcommand::CompleteTask(case) => {
                    match send_complete_task(case.case_number, case.assign_to.as_str()).await {
                        Ok(_) => println!("Task Completed"),
                        Err(e) => println!("Error Completing Task: {}", e),
                    };
                }
                CaseSubcommand::GetStatus(case) => {
                    get_case_status(case.case_number).await?;
                }
                CaseSubcommand::GetCommentSummary(case) => {
                    println!("Get Comment Summary");
                }
                CaseSubcommand::AddComment(case) => {
                    add_case_comment(case.case_number, case.comment.as_str()).await?;
                }
                CaseSubcommand::UpdateQuote(case) => {
                    update_case_quote(case.case_number, case.new_quantity).await?;
                }
            },
        },
        EntityType::Setup(setup_info) => {
            setup(
                setup_info.bw_client_id.as_deref(),
                setup_info.bw_client_secret.as_deref(),
                setup_info.bw_master_password.as_deref(),
                setup_info.epicor_base_url.as_deref(),
                setup_info.epicor_api_key.as_deref(),
                setup_info.epicor_username.as_deref(),
                setup_info.epicor_password.as_deref(),
            )
            .await
            .expect("Setup Failed.");
        }
    }
    Ok(())
}
