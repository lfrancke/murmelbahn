use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use snafu::prelude::*;
use crate::bom::BillOfMaterial;
use crate::error::{IoSnafu, MurmelbahnError, MurmelbahnResult, ReadSnafu, SerdeJsonSnafu};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Name {
    pub language_code: String,
    pub name: String
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Set {
    pub id: String,

    #[serde(default)]
    pub names: Vec<Name>,

    #[serde(default)]
    pub bill_of_materials: BillOfMaterial
}

impl Set {

    pub fn from_path<P: AsRef<Path>>(path: P) -> MurmelbahnResult<Set> {
         let file = File::open(path).context(ReadSnafu {})?;
         let reader = BufReader::new(file);

         let set = serde_json::from_reader(reader).context(SerdeJsonSnafu)?;
         Ok(set)
    }


}

pub struct Sets {
    sets: Vec<Set>
}


impl Sets {
    fn new() -> Sets {
        Sets { sets: Vec::new() }
    }

    pub fn read_directory<P: AsRef<Path>>(&mut self, path: P) -> MurmelbahnResult<()> {
        let path = path.as_ref();

        if !path.exists() || !path.is_dir() {
            return Err(MurmelbahnError::MiscError { msg: "Path is not a directory".to_string() });
        }

        // Read all files in the directory
        for entry in fs::read_dir(path).context(IoSnafu)? {
            let entry = entry.context(IoSnafu)?;
            let file_path = entry.path();

            // Only process files
            if file_path.is_file() {
                let set = Set::from_path(file_path)?;
                self.sets.push(set);
            }
        }

        Ok(())
    }
}
