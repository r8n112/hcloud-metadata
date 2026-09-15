//! `hcloud-metadata` command-line interface.

use clap::{Parser, Subcommand};
use hcloud_metadata::{metadata, Error, MetadataClient, Result};

#[derive(Debug, Parser)]
#[command(
    name = "hcloud-metadata",
    version,
    about = "Read the Hetzner Cloud instance metadata service"
)]
struct Cli {
    /// Print raw JSON instead of a human-readable summary.
    #[arg(long, global = true)]
    json: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Print the full instance metadata.
    Show,
    /// Print a single metadata field, for example `instance-id` or `hostname`.
    Get {
        /// Metadata key.
        key: String,
    },
    /// Print the cloud-init user data.
    UserData,
}

fn main() -> std::process::ExitCode {
    let cli = Cli::parse();
    match run(&cli) {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            std::process::ExitCode::FAILURE
        }
    }
}

fn run(cli: &Cli) -> Result<()> {
    let client = MetadataClient::new();
    match &cli.command {
        Command::Show => {
            let info = metadata::get(&client)?;
            if cli.json {
                print_json(&info)?;
            } else {
                print_metadata(&info);
            }
        }
        Command::Get { key } => println!("{}", metadata::field(&client, key)?),
        Command::UserData => print!("{}", metadata::user_data(&client)?),
    }
    Ok(())
}

fn print_metadata(info: &metadata::Metadata) {
    let fields = [
        ("instance-id", info.instance_id.as_deref()),
        ("hostname", info.hostname.as_deref()),
        ("availability-zone", info.availability_zone.as_deref()),
        ("region", info.region.as_deref()),
        ("public-ipv4", info.public_ipv4.as_deref()),
        ("public-ipv6", info.public_ipv6.as_deref()),
    ];
    for (label, value) in fields {
        if let Some(value) = value {
            println!("{label}: {value}");
        }
    }
    if !info.public_keys.is_empty() {
        println!("public-keys: {}", info.public_keys.len());
    }
}

fn print_json<V: serde::Serialize>(value: &V) -> Result<()> {
    let encoded =
        serde_json::to_string_pretty(value).map_err(|error| Error::Decode(error.to_string()))?;
    println!("{encoded}");
    Ok(())
}
