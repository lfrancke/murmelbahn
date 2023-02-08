use deku::prelude::*;
use serde::Serialize;
use snafu::ResultExt;
use std::cmp::max;
use std::fs;
use std::path::Path;

use crate::course::{power2022, ziplineadded2019};
use crate::error::{DeserializeFailedSnafu, MurmelbahnResult};

pub mod layer;
pub mod pillar;
pub mod rail;
pub mod wall;

#[derive(Clone, Debug, DekuRead, Serialize)]
#[deku(type = "u32")]
pub enum CourseKind {
    None = 0,
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
#[deku(type = "u32")]
pub enum ObjectiveKind {
    None = 0,
}

#[derive(Debug, DekuRead, Serialize)]
#[deku(type = "u32")]
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

        let distance = dx + max(0, dy - dx);
        distance
    }
}

#[deku_derive(DekuRead)]
#[derive(Clone, Debug, Serialize)]
pub struct CourseMetaData {
    pub creation_timestamp: u64,

    #[deku(temp)]
    size: u8,

    #[deku(count = "size")]
    #[deku(
        map = "|field: Vec<u8>| -> Result<_, DekuError> { Ok(std::str::from_utf8(&field).unwrap().to_owned()) }"
    )] // This is also horrible, maybe someone can tell me a better way to do it
    pub title: String,

    pub order_number: i32,
    pub course_kind: CourseKind,
    pub objective_kind: ObjectiveKind,
    pub difficulty: i32,
    pub completed: bool,
}

#[derive(Debug, DekuRead, Serialize)]
pub struct SavedCourse {
    pub header: SaveDataHeader,
    #[deku(ctx = "header.version")]
    pub course: Course,
}

impl SavedCourse {
    /// Reads a serialized course from a `Path`
    pub fn from_path<P: AsRef<Path>>(path: P) -> SavedCourse {
        let contents = fs::read(path.as_ref()).expect("Something went wrong reading the file");

        SavedCourse::from_bytes(&contents).unwrap()
    }

    /// Reads a serialized course from the provided bytes.
    ///
    /// Note: This does not take an offset or return the rest as it wasn't required for my use-case.
    /// Such a method could be included easily if needed.
    pub fn from_bytes(bytes: &[u8]) -> MurmelbahnResult<SavedCourse> {
        Ok(<SavedCourse as DekuContainerRead>::from_bytes((bytes, 0))
            .context(DeserializeFailedSnafu {})?
            .1)
    }
}

#[derive(Debug, DekuRead, Serialize)]
#[deku(ctx = "version: CourseSaveDataVersion", id = "version")]
pub enum Course {
    #[deku(id_pat = "CourseSaveDataVersion::ZiplineAdded2019")]
    ZiplineAdded2019(ziplineadded2019::Course),

    #[deku(id_pat = "CourseSaveDataVersion::Power2022")]
    Power2022(#[deku(ctx = "CourseSaveDataVersion::Power2022")] power2022::Course),

    #[deku(id_pat = "CourseSaveDataVersion::Pro2020")]
    Pro2020(#[deku(ctx = "CourseSaveDataVersion::Pro2020")] power2022::Course),
}

impl Course {
    pub fn meta_data(&self) -> CourseMetaData {
        match self {
            Course::ZiplineAdded2019(course) => course.meta_data.clone(),
            Course::Power2022(course) | Course::Pro2020(course) => course.meta_data.clone(),
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
#[deku(type = "u32")]
pub enum CourseSaveDataVersion {
    InitialLaunch = 100101,
    RailRework2018 = 100201,
    PersistenceRefactor2019 = 1,
    ZiplineAdded2019 = 2,
    Pro2020 = 3,
    Power2022 = 4,
}
