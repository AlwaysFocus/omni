use base64::engine::general_purpose;
use base64::Engine;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use zip::ZipArchive;

fn download_and_extract(url: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Perform the HTTP request
    let response = reqwest::blocking::get(url)?;

    // Create a temporary file to store the downloaded ZIP
    let mut temp_file = tempfile::NamedTempFile::new()?;

    // Write the response to the temporary file
    io::copy(&mut response.bytes().unwrap().as_ref(), &mut temp_file)?;

    // Open the downloaded ZIP file
    let zip_file = File::open(temp_file.path())?;
    let mut archive = ZipArchive::new(zip_file)?;

    // Extract the file named "bw" from the ZIP archive
    let mut bw_file = archive.by_name("bw")?;
    let mut buffer = Vec::new();
    bw_file.read_to_end(&mut buffer)?;

    // Determine the appropriate path to add "bw" to the system path
    let os = env::consts::OS;
    let system_path = match os {
        "windows" => "C:\\Windows\\System32", // Modify this as needed
        "macos" => "/usr/local/bin",          // Modify this as needed
        "linux" => "/usr/local/bin",          // Modify this as needed
        _ => {
            return Err(format!("Unsupported operating system: {}", os).into());
        }
    };

    // Create the path and write the "bw" file to it
    let path = Path::new(system_path).join("bw");
    let mut file = File::create(&path)?;
    file.write_all(&buffer)?;

    // Make the file executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = file.metadata()?.permissions();
        permissions.set_mode(0o755);
        file.set_permissions(permissions)?;
    }

    // Make the file executable for Windows
    #[cfg(windows)]
    {
        use std::os::windows::fs::PermissionsExt;
        let mut permissions = file.metadata()?.permissions();
        permissions.set_readonly(false);
        permissions.set_mode(0o755);
        file.set_permissions(permissions)?;
    }

    // Make the file executable for MacOS
    #[cfg(macos)]
    {
        use std::os::macos::fs::PermissionsExt;
        let mut permissions = file.metadata()?.permissions();
        permissions.set_readonly(false);
        permissions.set_mode(0o755);
        file.set_permissions(permissions)?;
    }

    // Add the path to the system path environment variable
    let path_var = match os {
        "windows" => "Path",
        _ => "PATH",
    };

    let current_path = env::var_os(path_var).unwrap_or_default();

    // Split the current_path into components
    let mut paths: Vec<_> = env::split_paths(&current_path).collect();

    // Add the new system_path to the paths list
    paths.push(PathBuf::from(system_path));

    // Join all paths together
    let new_path = env::join_paths(paths)?;

    // Set the new environment variable
    env::set_var(path_var, &new_path);

    Ok(())
}


fn create_env_file(
    client_id: Option<&str>,
    client_secret: Option<&str>,
    master_password: Option<&str>,
    epicor_base_url: Option<&str>,
    epicor_api_key: Option<&str>,
    epicor_basic_auth: Option<&str>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let os = env::consts::OS;

    let env_file_path = match os {
        "windows" | "macos" | "linux" => {
            let mut path = env::current_dir()?;
            path.push("./.env");
            path
        }
        _ => {
            return Err(format!("Unsupported operating system: {}", os).into());
        }
    };

    let mut env_file = File::create(&env_file_path)?;

    if let Some(client_id) = client_id {
        env_file.write_all(format!("BW_CLIENTID={}\n", client_id).as_bytes())?;
    }

    if let Some(client_secret) = client_secret {
        env_file.write_all(format!("BW_CLIENTSECRET={}\n", client_secret).as_bytes())?;
    }

    if let Some(master_password) = master_password {
        env_file.write_all(format!("MASTER_PASSWORD={}\n", master_password).as_bytes())?;
    }

    if let Some(epicor_base_url) = epicor_base_url {
        env_file.write_all(format!("EPICOR_BASE_URL={}\n", epicor_base_url).as_bytes())?;
    }

    if let Some(epicor_api_key) = epicor_api_key {
        env_file.write_all(format!("EPICOR_API_KEY={}\n", epicor_api_key).as_bytes())?;
    }

    if let Some(epicor_basic_auth) = epicor_basic_auth {
        env_file.write_all(format!("EPICOR_BASIC_AUTH='{}'\n", epicor_basic_auth).as_bytes())?;
    }

    // Ensure that all users have read/write permissions to the file
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = env_file.metadata()?.permissions();
        permissions.set_mode(0o666);
        env_file.set_permissions(permissions)?;
    }

    // Windows
    #[cfg(windows)]
    {
        use std::os::windows::fs::PermissionsExt;
        let mut permissions = env_file.metadata()?.permissions();
        permissions.set_mode(0o666);
        env_file.set_permissions(permissions)?;
    }

    // MacOS
    #[cfg(macos)]
    {
        use std::os::macos::fs::PermissionsExt;
        let mut permissions = env_file.metadata()?.permissions();
        permissions.set_mode(0o666);
        env_file.set_permissions(permissions)?;
    }

    Ok(())
}

fn generate_basic_auth(username: &str, password: &str) -> String {
    let auth_str = format!("{}:{}", username, password);
    let encoded_auth_str = general_purpose::STANDARD.encode(auth_str.as_bytes());
    return format!("Basic {}", encoded_auth_str);
}

pub(crate) async fn setup(
    client_id: Option<&str>,
    client_secret: Option<&str>,
    master_password: Option<&str>,
    epicor_base_url: Option<&str>,
    epicor_api_key: Option<&str>,
    epicor_username: Option<&str>,
    epicor_password: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    let os = env::consts::OS;

    match os {
        "windows" => {
            tokio::task::spawn_blocking(|| -> Result<(), Box<dyn Error + Send + Sync>> {
                download_and_extract(
                    "https://vault.bitwarden.com/download/?app=cli&platform=windows",
                )
            })
            .await?
            .expect("TODO: panic message");
        }
        "macos" => {
            tokio::task::spawn_blocking(|| -> Result<(), Box<dyn Error + Send + Sync>> {
                download_and_extract("https://vault.bitwarden.com/download/?app=cli&platform=macos")
            })
            .await?
            .expect("TODO: panic message");
        }
        "linux" => {
            tokio::task::spawn_blocking(|| -> Result<(), Box<dyn Error + Send + Sync>> {
                download_and_extract("https://vault.bitwarden.com/download/?app=cli&platform=linux")
            })
            .await?
            .expect("TODO: panic message");
        }
        _ => {
            println!("Unsupported operating system: {}", os);
            return Ok(());
        }
    }

    let epicor_basic_auth = match (epicor_username, epicor_password) {
        (Some(username), Some(password)) => generate_basic_auth(username, password),
        _ => String::new(),
    };

    
    let client_id = client_id.unwrap().to_string();
    let client_secret = client_secret.unwrap().to_string();
    let master_password = master_password.unwrap().to_string();
    let epicor_base_url = epicor_base_url.unwrap().to_string();
    let epicor_api_key = epicor_api_key.unwrap().to_string();
    let epicor_basic_auth = epicor_basic_auth;

    
    tokio::task::spawn_blocking(move || -> Result<(), Box<dyn Error + Send + Sync>> {
        create_env_file(
            Some(&client_id),
            Some(&client_secret),
            Some(&master_password),
            Some(&epicor_base_url),
            Some(&epicor_api_key),
            Some(&epicor_basic_auth),
        )
    })
    .await?
    .expect("TODO: panic message");

    println!("Successfully downloaded and added 'bw' to the system path.");
    Ok(())
}
