use clap::{Args, Parser, Subcommand};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct OmniArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Setup Omni:
    /// This command will setup Omni by creating a .env file and setting up the BitWarden Vault Tool
    Setup(SetupCommand),
    /// Bitwarden CLI wrapper - Used to interact with Bitwarden Vault
    Bitwarden(BitwardenCommand),
    /// Interact with Epicor ERP
    Epicor(EpicorCommand),
}

#[derive(Debug, Args)]
pub struct SetupCommand {
    /// BitWarden Client ID
    #[clap(short = 'i', long)]
    pub bw_client_id: Option<String>,
    /// BitWarden Client Secret
    #[clap(short = 's', long)]
    pub bw_client_secret: Option<String>,
    /// BitWarden Master Password
    #[clap(short = 'p', long)]
    pub bw_master_password: Option<String>,
    /// Epicor Base URL
    #[clap(short = 'u', long)]
    pub epicor_base_url: Option<String>,
    /// Epicor API Key
    #[clap(short = 'k', long)]
    pub epicor_api_key: Option<String>,
    /// Epicor Username
    #[clap(short = 'n', long)]
    pub epicor_username: Option<String>,
    /// Epicor Password
    #[clap(short = 'w', long)]
    pub epicor_password: Option<String>,
}

#[derive(Debug, Args)]
pub struct BitwardenCommand {
    #[clap(subcommand)]
    pub subcommand: BitwardenSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum BitwardenSubcommand {
    /// Lists BitWarden Vault items
    List,
    /// Gets BitWarden Vault item
    Get(GetCommand),
    /// Creates BitWarden Vault item
    Create(CreateCommand),
}

#[derive(Debug, Args)]
pub struct GetCommand {
    /// Type of BitWarden Vault item (item|username|password|uri|totp|exposed|attachment|folder|collection|organization|org-collection|template|fingerprint)
    #[clap(short, long)]
    pub item_type: VaultItemType,
    /// Value of the vault item (e.g. CAEL10)
    #[clap(short, long)]
    pub name: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum VaultItemType {
    Item,
    Username,
    Password,
    Uri,
    Totp,
    Exposed,
    Attachment,
    Folder,
    Collection,
    Organization,
    OrgCollection,
    Template,
    Fingerprint,
}

impl Display for VaultItemType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            VaultItemType::Item => write!(f, "item"),
            VaultItemType::Username => write!(f, "username"),
            VaultItemType::Password => write!(f, "password"),
            VaultItemType::Uri => write!(f, "uri"),
            VaultItemType::Totp => write!(f, "totp"),
            VaultItemType::Exposed => write!(f, "exposed"),
            VaultItemType::Attachment => write!(f, "attachment"),
            VaultItemType::Folder => write!(f, "folder"),
            VaultItemType::Collection => write!(f, "collection"),
            VaultItemType::Organization => write!(f, "organization"),
            VaultItemType::OrgCollection => write!(f, "org-collection"),
            VaultItemType::Template => write!(f, "template"),
            VaultItemType::Fingerprint => write!(f, "fingerprint"),
        }
    }
}

impl FromStr for VaultItemType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "item" => Ok(VaultItemType::Item),
            "username" => Ok(VaultItemType::Username),
            "password" => Ok(VaultItemType::Password),
            "uri" => Ok(VaultItemType::Uri),
            "totp" => Ok(VaultItemType::Totp),
            "exposed" => Ok(VaultItemType::Exposed),
            "attachment" => Ok(VaultItemType::Attachment),
            "folder" => Ok(VaultItemType::Folder),
            "collection" => Ok(VaultItemType::Collection),
            "organization" => Ok(VaultItemType::Organization),
            "org-collection" => Ok(VaultItemType::OrgCollection),
            "template" => Ok(VaultItemType::Template),
            "fingerprint" => Ok(VaultItemType::Fingerprint),
            _ => Err(format!("{} is not a valid VaultItemType", s)),
        }
    }
}

#[derive(Debug, Args)]
pub struct CreateCommand {
    /// Name of BitWarden Vault item
    #[clap(short, long)]
    pub name: String,
    /// Username of BitWarden Vault item
    #[clap(short, long)]
    pub username: String,
    /// Password of BitWarden Vault item
    #[clap(short, long)]
    pub password: String,
    /// Notes of BitWarden Vault item
    #[clap(short, long)]
    pub notes: String,
}

#[derive(Debug, Args)]
pub struct EpicorCommand {
    #[clap(subcommand)]
    pub subcommand: EpicorSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum EpicorSubcommand {
    /// Interact with Epicor Cases
    Case(CaseCommand),
}

#[derive(Debug, Args)]
pub struct CaseCommand {
    #[clap(subcommand)]
    pub subcommand: CaseSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum CaseSubcommand {
    /// Completes the current task for a given Epicor case
    CompleteTask(CompleteTaskCommand),
    /// Adds a comment to a given Epicor case
    AddComment(AddCommentCommand),
    /// Retrieves the current status of a given case
    GetStatus(GetStatusCommand),
    /// Gets a summary of the case comments
    GetCommentSummary(GetCommentSummaryCommand),
    /// Updates the Quote for a given case
    UpdateQuote(UpdateQuoteCommand),
}

#[derive(Debug, Args)]
pub struct CompleteTaskCommand {
    /// Epicor case number
    #[clap(short = 'n', long)]
    pub case_number: u32,
    /// Who the next task should be assigned to
    #[clap(short, long)]
    pub assign_to: String,
    /// Optional comment to add to the case
    #[clap(short, long)]
    pub comment: Option<String>,
}

#[derive(Debug, Args)]
pub struct AddCommentCommand {
    /// Epicor case number
    #[clap(short = 'n', long)]
    pub case_number: u32,
    /// Comment to add to the case
    #[clap(short, long)]
    pub comment: String,
}

#[derive(Debug, Args)]
pub struct GetStatusCommand {
    /// Epicor case number
    #[clap(short = 'n', long)]
    pub case_number: u32,
}

#[derive(Debug, Args)]
pub struct GetCommentSummaryCommand {
    /// Epicor case number
    #[clap(short = 'n', long)]
    pub case_number: u32,
}

#[derive(Debug, Args)]
pub struct UpdateQuoteCommand {
    /// Epicor case number
    #[clap(short = 'c', long)]
    pub case_number: u32,
    /// New Quantity for the Case Part (used to update quote)
    #[clap(short = 'n', long)]
    pub new_quantity: f32,
}
