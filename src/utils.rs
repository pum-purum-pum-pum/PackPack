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


#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Description {
    pub frame_times: HashMap<String, usize>, // * number of ticks to sped on each frame of animation
}

type DefaultImageBuffer = image::ImageBuffer<image::Rgba<u8>, std::vec::Vec<u8>>;

pub fn load_image(path: &str, name: &str) -> DefaultImageBuffer {
    let path_str = &format!("{}/{}.png", path, name);
    let texture_file = File::open(path_str).unwrap();
    let reader = BufReader::new(texture_file);
    let image = image::load(reader, image::PNG).unwrap().to_rgba();
    return image;
}

pub fn find_all_subdirs(dir: &Path) -> Result<Vec<DirEntry>, std::io::Error> {
    let mut res = vec!();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            res.push(entry);
        }
    }
    return Ok(res);
}

pub fn find_all_format(dir: &Path, format: &str) -> (Vec<String>, HashMap<String, usize>) {
    let file_type = format.len();
    let mut result = vec!();
    let mut name_to_id = HashMap::new();
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let file_name = entry.file_name();
        let file_name= file_name.into_string().unwrap(); 
        let format = file_name
            .chars()
            .rev()
            .take(file_type)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect::<String>();
        if format == format {
            let object_name = file_name
                .chars()
                .take(file_name.len() - file_type)
                .collect::<String>();
            result.push(object_name.clone());
            name_to_id.insert(object_name, result.len() - 1);
        }
    }
    return (result, name_to_id);
}
