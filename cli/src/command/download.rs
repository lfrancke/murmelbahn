use std::path::PathBuf;
use tracing::{debug, info};

use crate::Config;

#[derive(clap::Args)]
pub struct DownloadArgs {
    /// The code of the course to download
    code: String,

    /// Where to save the course.
    /// If it's left out, a file named `<code>.course` will be written in the current directory.
    /// If it references a directory a file named `<code>.course` will be written in that directory.
    /// Otherwise it'll try to write to a file with the given name.
    #[arg(short, long)]
    destination: Option<PathBuf>,
}

pub fn download_course(args: DownloadArgs, _config: Config) -> anyhow::Result<()> {
    let upper_code = args.code.to_uppercase();
    info!("Attempting download of course '{}'", upper_code);
    let course =
        murmelbahn_lib::course::download::download_course(&upper_code)?.decode_base64_file();
    debug!("Successfully downloaded");

    let dest = match args.destination {
        None => PathBuf::from(&format!("{}.course", upper_code)),
        Some(destination) => match destination.is_dir() {
            true => destination.join(format!("{}.course", upper_code)),
            false => destination,
        },
    };

    match std::fs::write(dest.clone(), course) {
        Ok(_) => info!("Successfully saved course to '{:?}'", dest),
        Err(e) => info!(
            "Downloading course succeeded but saving to '{:?}' failed: {}",
            dest, e
        ),
    }
    Ok(())
}
