use crate::physical::Element;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use snafu::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::{fs, io};
use tracing::{debug, info};
use ts_rs::TS;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to read file [{path:?}]"))]
    FileRead { path: PathBuf, source: io::Error },

    #[snafu(display("Failed to deserialize JSON set file: [{path:?}]"))]
    JsonDeserialization {
        path: PathBuf,
        source: serde_json::Error,
    },

    #[snafu(display("Path [{path:?}] does not exist, can't load set file"))]
    InvalidPath { path: PathBuf },

    #[snafu(display("IO Error while traversing directory [{dir:?}]"))]
    Io { dir: PathBuf, source: io::Error },
}

#[derive(Clone, Serialize, Deserialize, JsonSchema, TS)]
#[ts(export)]
pub struct Name {
    pub language_code: String,
    pub name: String,
}

#[derive(Clone, Serialize, Default, Deserialize, JsonSchema, TS)]
#[ts(export)]
pub struct Set {
    pub id: String,

    #[serde(default)]
    pub names: Vec<Name>,

    #[serde(default)]
    pub content: HashMap<Element, i32>,
}

impl Set {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Set, Error> {
        let path = path.as_ref();
        let file = File::open(path).context(FileReadSnafu { path })?;
        let reader = BufReader::new(file);

        let set: Set =
            serde_json::from_reader(reader).context(JsonDeserializationSnafu { path })?;
        debug!("Successfully read set [{}] from file [{:?}]", set.id, path);
        Ok(set)
    }
}

#[derive(TS)]
#[ts(export)]
pub struct SetRepo {
    pub sets: HashMap<String, Set>,
}

impl Default for SetRepo {
    fn default() -> Self {
        Self::new()
    }
}

impl SetRepo {
    pub fn new() -> SetRepo {
        SetRepo {
            sets: HashMap::new(),
        }
    }

    pub fn read_directory<P: AsRef<Path>>(&mut self, path: P) -> Result<(), Error> {
        let path = path.as_ref();

        ensure!(
            path.exists() && path.is_dir(),
            InvalidPathSnafu {
                path: path.to_path_buf()
            }
        );

        debug!("Reading set definitions from directory [{path:?}] now");

        // Read all files in the directory
        for entry in fs::read_dir(path).context(IoSnafu { dir: path })? {
            let entry = entry.context(IoSnafu { dir: path })?;
            let file_path = entry.path();

            // Only process files
            if file_path.is_file() {
                let set = Set::from_path(file_path)?;
                let set_id = set.id.clone();
                if self.sets.insert(set_id.clone(), set).is_some() {
                    info!(
                        "Set with ID [{}] occurs twice, will use a random one",
                        set_id
                    );
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::SetRepo;
    use std::path::PathBuf;

    /// Every committed set definition parses, including its element names.
    #[test]
    fn all_committed_sets_load() {
        let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        dir.push("../data/sets");
        let mut repo = SetRepo::new();
        repo.read_directory(&dir)
            .expect("all sets in data/sets parse");
        assert!(
            repo.sets.len() >= 80,
            "expected the full set catalogue, got {}",
            repo.sets.len()
        );
    }
}
