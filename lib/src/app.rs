//! This module contains all code needed to read files as used by the official Ravensburger Gravitrax app
//! The data they contain might be suitable for recreating courses and creating building instructions
//! but they are unsuitable for a lot of processing tasks.
//!
//! As such we use this module to process the data and then convert it into a more useful form.
pub mod bom;
pub mod course;
pub mod download;
pub mod layer;
pub mod pillar;
pub mod power2022;
pub mod rail;
pub mod wall;
pub mod ziplineadded2019;

pub use bom::BillOfMaterials;
