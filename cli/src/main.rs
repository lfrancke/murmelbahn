pub(crate) mod command;

use std::fs;

use anyhow::Result;
use clap::Parser;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use tracing::info;
use crate::command::download::DownloadArgs;
use crate::command::download_multiple::DownloadMultipleArgs;
use crate::command::dump::{dump_course, DumpArgs};

#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    #[command(subcommand)]
    action: Action,
}

#[derive(clap::Subcommand)]
enum Action {
    Download(DownloadArgs),
    DownloadMultiple(DownloadMultipleArgs),
    Dump(DumpArgs),
}

// By not using options and implementing Default instead we can write a config file to disk on first run
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    // TODO: Can this be an enum?
    pub config_version: u8,
    pub default_language: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            config_version: 1,
            default_language: "en".to_string(),
        }
    }
}

fn main() -> Result<()> {
    // Parse args first as something like displaying version or help doesn't need config to be initialized
    let args = Args::parse();

    // TODO: Introduce https://docs.rs/clap-verbosity-flag
    tracing_subscriber::fmt::init();

    let config = read_config();

    match args.action {
        Action::Download(args) => command::download::download_course(args, config),
        Action::Dump(args) => dump_course(args, config),
        Action::DownloadMultiple(args) => command::download_multiple::download_multiple_courses(args, config),
    }
}

fn read_config() -> Config {
    if let Some(proj_dirs) = ProjectDirs::from("", "", "murmelbahn") {
        let conf_dir = proj_dirs.config_dir();
        let conf_file = conf_dir.join("murmelbahn.toml");

        // Try to write a default config file if one doesn't exist yet
        if !conf_file.exists() {
            if let Err(e) = fs::create_dir_all(conf_dir) {
                info!("Failed creating '{:?}' for configuration, skipping config init, will use defaults: {}", conf_dir, e);
                return Default::default();
            }

            let toml_string = toml::to_string(&Config::default()).expect("Could not encode TOML value");
            if let Err(e) = fs::write(conf_file.clone(), toml_string) {
                info!("Failed writing default config, will use defaults: {}", e);
                return Default::default();
            } else {
                info!("I have written a default config file to '{:?}', please adjust as needed", conf_file);
            }
        }

        let toml_string = match fs::read_to_string(&conf_file) {
            Err(e) => {
                info!("Failed reading config file from disk, will use defaults: {}", e);
                return Default::default();
            }
            Ok(toml) => toml
        };

        return match toml::from_str(&toml_string) {
            Ok(config) => config,
            Err(e) => {
                info!("Failed deserializing config, will use defaults: {}", e);
                Default::default()
            }
        };
    } else {
        info!("No valid home directory could be found, will use defaults");
    }

    Default::default()
}
