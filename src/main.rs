#[macro_use]
extern crate serde_derive;
extern crate ron;
extern crate serde;
extern crate image;

use std::{
    collections::HashMap,
    fs::{File},
    io::BufReader,
    path::Path,
    fs::{self, DirEntry},
};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct AnimationType(usize, usize); // * start_id end_id

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawAnimation {
    texture_filename: String,
    texture_dimentions: (usize, usize),
    animation_types: HashMap<String, AnimationType>,
}

type DefaultImageBuffer = image::ImageBuffer<image::Rgba<u8>, std::vec::Vec<u8>>;

pub fn load_image(name: &str) -> DefaultImageBuffer {
    let path_str = &format!("{}/textures/{}.png", env!("CARGO_MANIFEST_DIR"), name);
    let texture_file = File::open(path_str).unwrap();
    let reader = BufReader::new(texture_file);
    let image = image::load(reader, image::PNG).unwrap().to_rgba();
    return image;
}

pub fn find_all_format(dir: &Path, format: &str) -> (Vec<String>, HashMap<String, usize>) {
    let file_type = format.len();
    let mut result = vec!();
    let mut name_to_id = HashMap::new();
    if dir.is_dir() {
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
    }
    return (result, name_to_id);
}

fn main() {
    let path_str = format!("{}/textures", env!("CARGO_MANIFEST_DIR"));
    let images_path = Path::new(&path_str);
    // !@ Assuming png for now
    let (filenames, name_to_id) = find_all_format(
        images_path,
        ".png",
    );
    let mut images = vec!();
    for image_name in filenames.iter() {
        images.push(load_image(&image_name));
    };
    // !@ Assuming images dimentions are the same for now
    let (w_dim, h_dim) = images[0].dimensions();
    // * isn't it weird that image crate uses u32 instead of usize?
    let images_num = images.len() as u32;
    let w_num = 10u32;
    let h_num = (images_num + w_num - 1u32) / w_num;
    let (width, height) = (
        w_dim * w_num, 
        h_dim * h_num
    );
    let mut imgbuf = image::ImageBuffer::new(width, height);
    for x in 0..width {
        for y in 0..height {
            let h_id = y / h_dim;
            let w_id = x / w_dim;
            let local_x = x % w_dim;
            let local_y = y % h_dim;
            let current_tile = h_id * w_num + w_id;
            let pixel = imgbuf.get_pixel_mut(x, y);
            if current_tile < images_num {
                eprintln!("num:{:?} x:{:?}, y:{:?}", current_tile, local_x, local_y);
                let current_image = &images[current_tile as usize];
                let local_pixel = current_image.get_pixel(local_x, local_y);

                *pixel = *local_pixel;
            }
            else {
                *pixel = image::Rgba([0u8, 0u8, 0u8, 0u8]);
            }
        }
    }
    imgbuf.save("packed.png").unwrap();
}
