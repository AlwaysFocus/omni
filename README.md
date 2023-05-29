# Omni

Omni is a command-line interface application, written in Rust, that interacts with different services like Bitwarden and Epicor.

## Getting Started

To get a local copy up and running, follow these simple steps.

### Prerequisites

- Rust: You need to have Rust installed on your system to build from source. If you don't have Rust installed, you can follow the instructions [here](https://www.rust-lang.org/tools/install) to install it.

- Environment Variables: The program uses environment variables for configuration. Please run the `omni setup` command to create the `.env` file accordingly.

### Installation

There are two ways to install Omni:

1. **Downloading a pre-compiled binary:**

   Navigate to the [releases page](https://github.com/alwaysfocus/omni/releases) of this repository and download the latest binary suitable for your operating system. Once downloaded, you need to make it executable and move it to a location in your `PATH`.
    
    For Unix-like systems:

   ```sh
   chmod +x ./omni
   sudo mv ./omni /usr/local/bin/
   ```
   For Windows systems, you can just double-click on the downloaded .exe file.

2. **Building from source:**
    If you have Rust installed, you can build the binary from source by running the following command:

    ```sh
   git clone https://github.com/alwaysfocus/omni.git
    cd omni
    cargo build --release
    sudo mv ./target/release/omni /usr/local/bin/
    ```
   The binary omni will now be available for use.

## Usage
Omni CLI supports several commands under each entity type. Here is a brief overview of each:

### Setup
Set up Omni by running the `setup` command. It accepts BitWarden Client ID, Client Secret, Master Password, and Epicor Base URL, API Key, Username, and Password as arguments and then creates a `.env` file in the current directory. The `.env` file is used to store the environment variables for the application.

Example:
```sh
    omni-cli Setup -i [bw_client_id] -s [bw_client_secret] -p [bw_master_password] -u [epicor_base_url] -k [epicor_api_key] -n [epicor_username] -w [epicor_password]
```

### BitWarden
BitWarden commands are used to interact with the BitWarden service. The following commands are available:

`List`: Lists all Bitwarden Vault items.

```sh
omni bitwarden list
```
`Get`: Gets Bitwarden Vault item. Requires item_type and name.
```sh
omni Bitwarden get -t [item_type] -n [name]
```


### Epicor
Epicor commands are used to interact with the Epicor ERP. The following commands are available:

`Case`: Interacts with Epicor Cases. The available subcommand is `CompleteTask`.

`CompleteTask`: Completes the current task for a given Epicor case. Requires `case_number` and `assign_to`, `comment` is optional.
```sh
omni epicor case complete-task -n [case_number] -a [assign_to] -c [comment]
```

