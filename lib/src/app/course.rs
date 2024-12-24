use std::cmp::max;
use std::path::{Path, PathBuf};
use std::{fs, io};

use deku::prelude::*;
use serde::Serialize;
use snafu::prelude::*;

use crate::app::{power2022, ziplineadded2019};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to deserialize course"))]
    DeserializationFailedError { source: DekuError },

    #[snafu(display("Failed to read file [{path:?}]"))]
    IoError { path: PathBuf, source: io::Error },
}

#[derive(Clone, Debug, DekuRead, Serialize)]
#[deku(id_type = "u32")]
pub enum CourseKind {
    None = 0,
    // All tracks downloaded using a code from the app seem to be "Custom" courses
    Custom = 1,
    RegularEditorial = 2,
    Tutorial = 4,
    DownloadUser = 5,
    Recovery = 6,
    DownloadEditorial = 7,
    InAppPurchase = 8,
    ProEditorial = 9,
    PowerEditorial = 10,
}

#[derive(Clone, Debug, DekuRead, Serialize)]
#[deku(id_type = "u32")]
pub enum ObjectiveKind {
    None = 0,
}

#[derive(Debug, DekuRead, Serialize)]
#[deku(id_type = "u32")]
pub enum CourseElementGeneration {
    InitialLaunch = 0,
    Christmas2018 = 1,
    Easter2019 = 2,
    Autumn2019 = 3,
    Easter2020 = 4,
    Pro = 5,
    Fall2021 = 6,
    Spring2022 = 7,
    Power = 8,
    Autumn2023 = 9,
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    NorthEast,
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    pub fn hex_rotation_to_direction(hex_rotation: i32) -> Direction {
        match hex_rotation {
            0 => Direction::East,
            1 => Direction::SouthEast,
            2 => Direction::SouthWest,
            3 => Direction::West,
            4 => Direction::NorthWest,
            5 => Direction::NorthEast,
            _ => panic!("Invalid hex rotation"),
        }
    }
}

#[derive(Clone, Debug, DekuRead, Serialize)]
pub struct HexVector {
    pub y: i32,
    pub x: i32,
}

impl HexVector {
    pub fn new(x: i32, y: i32) -> HexVector {
        HexVector { x, y }
    }

    pub fn add(&self, other: &HexVector) -> HexVector {
        HexVector {
            y: self.y + other.y,
            x: self.x + other.x,
        }
    }

    pub fn hex_vector_in_distance(&self, direction: &Direction, amount: i32) -> HexVector {
        match direction {
            Direction::NorthEast => HexVector::new(self.x + amount, self.y - amount),
            Direction::East => HexVector::new(self.x, self.y - amount),
            Direction::SouthEast => HexVector::new(self.x - amount, self.y),
            Direction::SouthWest => HexVector::new(self.x - amount, self.y + amount),
            Direction::West => HexVector::new(self.x, self.y + amount),
            Direction::NorthWest => HexVector::new(self.x + amount, self.y),
        }
    }

    pub fn neighbor(&self, direction: &Direction) -> HexVector {
        self.hex_vector_in_distance(direction, 1)
    }

    /// This expects world coordinates
    pub fn distance(&self, to: &HexVector) -> i32 {
        // Distance between the two on each axis
        let dx = (self.x - to.x).abs();
        let dy = (self.y - to.y).abs();

        dx + max(0, dy - dx)
    }
}

#[deku_derive(DekuRead)]
#[derive(Clone, Debug, Serialize)]
pub struct CourseMetaData {
    pub creation_timestamp: u64,

    #[deku(temp)]
    size: u8,

    #[deku(count = "size")]
    #[deku(map = "CourseMetaData::decode_title")]
    pub title: String,

    pub order_number: i32,
    pub course_kind: CourseKind,
    pub objective_kind: ObjectiveKind,
    pub difficulty: i32,
    pub completed: bool,
}

impl CourseMetaData {
    /// This tries to decode the original title that was used by the creator in the App.
    /// As this should always come from the App directly it _should_ not fail.
    fn decode_title(bytes: Vec<u8>) -> Result<String, DekuError> {
        std::str::from_utf8(&bytes)
            .map(|title| title.to_string())
            .map_err(|source| {
                DekuError::Parse(format!(
                    "Could not interpret title bytes as valid UTF-8: {source}"
                ).into())
            })
    }
}

#[derive(Debug, DekuRead, Serialize)]
pub struct SavedCourse {
    pub header: SaveDataHeader,
    #[deku(ctx = "header.version")]
    pub course: Course,
}

impl SavedCourse {
    /// Reads a serialized course from a `Path`
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<SavedCourse, Error> {
        let path = path.as_ref();
        let contents = fs::read(path).context(IoSnafu { path })?;
        SavedCourse::from_bytes(&contents)
    }

    /// Reads a serialized course from the provided bytes.
    ///
    /// Note: This does not take an offset or return the rest as it wasn't required for my use-case.
    /// Such a method could be included easily if needed.
    pub fn from_bytes(bytes: &[u8]) -> Result<SavedCourse, Error> {
        Ok(<SavedCourse as DekuContainerRead>::from_bytes((bytes, 0))
            .context(DeserializationFailedSnafu)?
            .1)
    }
}

/// A `Course` is the main entry point.
///
/// There are multiple versions of courses which have been added over the years.
/// Anything older than 2019 (`ZiplineAdded2019`) is not currently supported.
/// Only courses since 2020 (`Pro2020` and `LightStones2023`) have any meaningful support besides showing their contents.
/// This is because most courses that have been created are 2020 or newer.
#[derive(Debug, DekuRead, Serialize)]
#[deku(ctx = "version: CourseSaveDataVersion", id = "version")]
#[serde(untagged)]
pub enum Course {
    #[deku(id_pat = "CourseSaveDataVersion::ZiplineAdded2019")]
    ZiplineAdded2019(
        #[deku(ctx = "CourseSaveDataVersion::ZiplineAdded2019")] ziplineadded2019::Course,
    ),

    #[deku(id_pat = "CourseSaveDataVersion::Power2022")]
    Power2022(#[deku(ctx = "CourseSaveDataVersion::Power2022")] power2022::Course),

    #[deku(id_pat = "CourseSaveDataVersion::Pro2020")]
    Pro2020(#[deku(ctx = "CourseSaveDataVersion::Pro2020")] power2022::Course),

    #[deku(id_pat = "CourseSaveDataVersion::LightStones2023")]
    LightStones2023(#[deku(ctx = "CourseSaveDataVersion::LightStones2023")] power2022::Course),
}

impl Course {
    pub fn meta_data(&self) -> CourseMetaData {
        match self {
            Course::ZiplineAdded2019(course) => course.meta_data.clone(),
            Course::Power2022(course) | Course::Pro2020(course) => course.meta_data.clone(),
            Course::LightStones2023(course) => course.meta_data.clone(),
        }
    }
}

#[derive(Debug, DekuRead, Serialize)]
pub struct SaveDataHeader {
    pub guid: u128,
    pub version: CourseSaveDataVersion,
}

// Copy needed for deku magic
#[derive(Clone, Copy, Debug, DekuRead, PartialEq, Serialize)]
#[deku(id_type = "u32")]
pub enum CourseSaveDataVersion {
    InitialLaunch = 100101,
    RailRework2018 = 100201,
    PersistenceRefactor2019 = 1,
    ZiplineAdded2019 = 2,
    Pro2020 = 3,
    Power2022 = 4,
    LightStones2023 = 5,
}
