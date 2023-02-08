use crate::Config;
use clap::ValueEnum;
use std::path::PathBuf;

#[derive(clap::Args)]
pub struct DumpArgs {
    course: PathBuf,

    #[arg(short, long)]
    format: Option<Format>,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum Format {
    Json,
    Toml,
    Rust,
}

pub fn dump_course(args: DumpArgs, _config: Config) -> anyhow::Result<()> {
    let course = murmelbahn_lib::course::common::SavedCourse::from_path(args.course);

    let format = args.format.unwrap_or(Format::Json);
    let output = match format {
        Format::Json => serde_json::to_string_pretty(&course)?,
        Format::Toml => toml::to_string_pretty(&course)?,
        Format::Rust => format!("{:#?}", course),
    };

    println!("{}", output);

    Ok(())
}
