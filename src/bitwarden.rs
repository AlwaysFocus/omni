use crate::args::VaultItemType;
use anyhow::{anyhow, Result};
use clap::{arg, command, Command as ClapCommand, Parser, Subcommand};
use dotenv::dotenv;
use regex::Regex;
use std::env;
use std::process::Command;

fn login() -> Result<()> {
    let bw_clientid = env::var("BW_CLIENTID").map_err(|_| anyhow!("Failed to get BW_CLIENTID"))?;
    let bw_clientsecret =
        env::var("BW_CLIENTSECRET").map_err(|_| anyhow!("Failed to get BW_CLIENTSECRET"))?;

    env::set_var("BW_CLIENTID", bw_clientid);
    env::set_var("BW_CLIENTSECRET", bw_clientsecret);

    let login_output = Command::new("bw")
        .arg("login")
        .arg("--apikey")
        .output()
        .expect("Failed to execute command");

    if !login_output.status.success() {
        return Err(anyhow!("Failed to login with API key"));
    }

    println!("Login successful");

    Ok(())
}

fn unlock_vault() -> Result<()> {
    let master_password =
        env::var("MASTER_PASSWORD").map_err(|_| anyhow!("Failed to get MASTER_PASSWORD"))?;

    let unlock_output = Command::new("bw")
        .arg("unlock")
        .arg(&master_password)
        .output()
        .expect("Failed to execute command");

    if !unlock_output.status.success() {
        return Err(anyhow!("Failed to unlock vault"));
    }

    println!("Unlock successful");

    let output =
        String::from_utf8(unlock_output.stdout).map_err(|_| anyhow!("Failed to parse output"))?;

    let re = Regex::new(r#"export BW_SESSION="([^"]+)"#)
        .map_err(|_| anyhow!("Failed to compile regex"))?;

    let caps = re
        .captures(&output)
        .ok_or(anyhow!("Failed to find session key in output"))?;

    let session_key = caps
        .get(1)
        .ok_or(anyhow!("Failed to find session key in output"))?
        .as_str();

    env::set_var("BW_SESSION", session_key);

    Ok(())
}

fn lock_vault() -> Result<()> {
    let lock_output = Command::new("bw")
        .arg("lock")
        .output()
        .expect("Failed to execute command");

    if !lock_output.status.success() {
        return Err(anyhow!("Failed to lock vault"));
    }

    println!("Lock successful");

    Ok(())
}

fn logout() -> Result<()> {
    let logout_output = Command::new("bw")
        .arg("logout")
        .output()
        .expect("Failed to execute command");

    if !logout_output.status.success() {
        return Err(anyhow!("Failed to logout"));
    }

    println!("Logout successful");

    Ok(())
}

pub fn list_items() -> Result<()> {
    // Login to vault
    login()?;

    // Unlock vault
    unlock_vault()?;

    let list_output = Command::new("bw")
        .arg("list")
        .arg("items")
        .output()
        .expect("Failed to execute list command for bitwarden vault");

    if !list_output.status.success() {
        // Lock vault
        lock_vault()?;

        // Logout of vault
        logout()?;
        return Err(anyhow!("Failed to list vault items"));
    }

    println!("{}", String::from_utf8(list_output.stdout).unwrap());

    // Lock vault
    lock_vault()?;

    // Logout of vault
    logout()?;

    Ok(())
}

pub fn get_item(item_type: &VaultItemType, item_name: &str) -> Result<()> {
    // Login to vault
    login()?;

    // Unlock vault
    unlock_vault()?;

    let get_output = Command::new("bw")
        .arg("get")
        .arg(item_type.to_string())
        .arg(item_name)
        .output()
        .expect("Failed to execute get command for bitwarden vault");

    if !get_output.status.success() {
        // Lock vault
        lock_vault()?;

        // Logout of vault
        logout()?;
        return Err(anyhow!("Failed to get vault item"));
    }

    println!("{}", String::from_utf8(get_output.stdout).unwrap());

    // Lock vault
    lock_vault()?;

    // Logout of vault
    logout()?;

    Ok(())
}
