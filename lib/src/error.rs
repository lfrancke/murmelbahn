use snafu::prelude::*;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum MurmelbahnError {
    #[snafu(display("Unable to download course [{}]: {}", course, source))]
    DownloadFailed { source: reqwest::Error, course: String },

    #[snafu(display("Failed to deserialize course: {}", source))]
    DeserializeFailed { source: deku::DekuError },

    #[snafu(display("Failed reading file: {}", source))]
    ReadError { source: std::io::Error },

    #[snafu(display("Failed processing JSON: {}", source))]
    SerdeJsonError { source: serde_json::Error },

    #[snafu(display("Encountered unsupported piece"))]
    UnsupportedPiece,

    #[snafu(display("IO Error {}",  source))]
    IoError { source: std::io::Error },

    #[snafu(display("Encountered error: {}", msg))]
    ConversionFailed { msg: String},

    #[snafu(display("Encountered error: {}", msg))]
    MiscError { msg: String },
}

pub type MurmelbahnResult<T, E = MurmelbahnError> = Result<T, E>;
