pub mod utils;

use std::{
    collections::HashMap,
    fs::{File},
    io::BufReader,
    path::Path,
    fs::{self, DirEntry},
    string::ToString,
    ffi::OsString,
    io::Read as IORead,
};

use serde_derive::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawAnimation {
    pub texture_filename: String,
    pub texture_dimentions: (usize, usize),
    pub animation_types: HashMap<String, AnimationType>,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct AnimationType {
    pub start_id: usize, 
    pub end_id: usize,
    pub frame_ticks: usize, // * amount of ticks need to be done for frame update
}