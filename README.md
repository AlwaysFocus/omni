Omni CLI

Omni CLI is a command-line interface application, written in Rust, that interacts with different services like Bitwarden and Epicor.

Getting Started

To get a local copy up and running, follow these simple steps.

Prerequisites
Rust: You need to have Rust installed on your system to build from source. If you don't have Rust installed, you can follow the instructions here to install it.
Environment Variables: The program uses environment variables for configuration. Please setup .env file accordingly.
Installation
There are two ways to install Omni CLI:

Downloading a pre-compiled binary:
Navigate to the releases page of this repository and download the latest binary suitable for your operating system. Once downloaded, you need to make it executable and move it to a location in your PATH.
For Unix-like systems:
sh
Copy code
chmod +x ./omni
sudo mv ./omni /usr/local/bin/
For Windows systems, you can just double-click on the downloaded .exe file.
Building from source:
If you have Rust installed, you can clone the repository and build from source:
sh
Copy code
git clone https://github.com/alwaysfocus/omni.git
cd omni
cargo build --release
sudo mv ./target/release/omni-cli /usr/local/bin/
The binary omni-cli will now be available for use.