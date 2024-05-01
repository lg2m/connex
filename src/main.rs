pub(crate) mod config;

use clap::{Args, Parser, Subcommand};
use config::{Config, Server};
use std::process::Command;

/// SSH Connection Management
#[derive(Debug, Parser)]
#[command(name = "connex")]
#[command(about = "connex simplifies SSH connection management.")]
#[command(long_about = "Connex is a CLI tool designed to simplify the management of SSH connections. Providing users with an efficient way of handling of multiple SSH sessions, port forwarding, and secure file transfers.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Opens a connection to remote server
    #[command(arg_required_else_help = true)]
    Open {
        /// Host to connect to
        #[arg(short, long, required = true)]
        name: String,
        /// Override user configured key
        #[arg(short, long, required = false)]
        key_path: Option<String>,
    },
    /// Port forwarding
    #[command(arg_required_else_help = true)]
    Forward {
        /// Host to connect to
        #[arg(short, long, required = true)]
        name: String,
        #[arg(short, long, required = true)]
        app_name: String,
        /// Override user configured key
        #[arg(short, long, required = false)]
        key_path: Option<String>,
    }
}

fn main() {
    let config = match Config::load() {
        Ok(config) => config,
        Err(_) => {
            println!("Error: couldn't find connex.yaml in the home directory.");
            return
        },
    };

    let args = Cli::parse();

    match args.command {
        Commands::Open { name, key_path } => {
            if let Some(server) = config.servers.iter().find(|s| s.name == name) {
                println!("Connecting to => {}", server.host);
                match connect_to_server(server) {
                    Ok(_) => println!("Connection successful!"),
                    Err(e) => println!("Failed to connect: {:#?}", e),
                }
            } else {
                println!("Server not found in configuration..");
            }
        }
        Commands::Forward { name, app_name, key_path } => {
            if let Some(server) = config.servers.iter().find(|s| s.name == name) {
                if let Some(app) = server.applications.iter().find(|a| a.name == app_name) {
                    println!("Forwarding port => {}:{} to localhost:{}", server.host, app.port.remote, app.port.local);
                    match port_forward(server, app.port.local, app.port.remote) {
                        Ok(_) => println!("Port forwarding successful."),
                        Err(e) => println!("Failed to setup port forwarding: {:#?}", e),
                    }
                }
            }
        }
    }
}

fn connect_to_server(server: &Server) -> Result<(), String> {
    let ssh_command = format!(
        "ssh -i {} {}@{}",
        server.key_path,
        server.user,
        server.host,
    );

    let status = Command::new("sh")
        .arg("-c")
        .arg(ssh_command)
        .status()
        .expect("Failed to execute process.");

    if status.success() {
        Ok(())
    } else {
        Err(format!("SSH command failed with status: {}", status))
    }
}

fn port_forward(server: &Server, local_port: u16, remote_port: u16) -> Result<(), String> {
    let ssh_command = format!(
        "ssh -i {} -N -L {}:localhost:{} {}@{}",
        server.key_path,
        local_port,
        remote_port,
        server.user,
        server.host,
    );

    let status = Command::new("sh")
        .arg("-c")
        .arg(ssh_command)
        .status()
        .expect("Failed to execute process.");

    if status.success() {
        Ok(())
    } else {
        Err(format!("SSH port forwarding failed with status: {}", status))
    }
}
