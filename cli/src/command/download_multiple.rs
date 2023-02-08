use anyhow::Error;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use tracing::{debug, error, info};

use crate::Config;

#[derive(clap::Args)]
pub struct DownloadMultipleArgs {
    input: PathBuf,

    #[arg(short, long)]
    destination: Option<PathBuf>,
}

pub fn download_multiple_courses(
    args: DownloadMultipleArgs,
    _config: Config,
) -> anyhow::Result<()> {
    let destination = match args.destination {
        None => PathBuf::from(""),
        Some(dest) => {
            if !dest.is_dir() {
                // TODO: I'm sure clap can already catch this
                error!("`destination` needs to be an existing directory, exiting");
                return Err(Error::msg("Destination does not exist"));
            }
            dest
        }
    };

    let courses = std::fs::read_to_string(args.input)?;
    for line in courses.lines() {
        let upper_code = line.to_uppercase();
        info!("Attempting download of course '{}'", upper_code);
        let course =
            murmelbahn_lib::course::download::download_course(&upper_code)?.decode_base64_file();
        debug!("Successfully downloaded");

        let dest = destination.join(format!("{}.course", upper_code));

        match std::fs::write(dest.clone(), course) {
            Ok(_) => info!("Successfully saved course to '{:?}'", dest),
            Err(e) => info!(
                "Downloading course succeeded but saving to '{:?}' failed, skipping: {}",
                dest, e
            ),
        }

        debug!("Sleeping 500ms before attempting next download");
        thread::sleep(Duration::from_millis(500));
    }

    Ok(())
}
