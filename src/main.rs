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
    string::ToString,
    ffi::OsString
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

fn main() -> std::io::Result<()> {
    let path_str = format!("{}/textures", env!("CARGO_MANIFEST_DIR"));
    let images_path = Path::new(&path_str);
    let subdirs = find_all_subdirs(images_path)?;
    let mut filenames = vec!();
    let mut counter = 0;
    let mut name_to_id = HashMap::new();
    let mut animation_types = HashMap::new();
    for subdir in subdirs.iter() {
        // !@ Assuming png for now
        let (mut move_images_filenames, move_name_to_id) = find_all_format(&subdir.path(), ".png");
        move_images_filenames.sort();
        let start_animation_id = counter;
        let filename = subdir.file_name();
        let dir_name = filename.into_string().unwrap();
        for i in move_images_filenames.iter_mut() {
            *i = dir_name.clone() + "/" + i;
            name_to_id.insert(i.clone(), counter);
            counter += 1;
        }
        let end_animation_id = counter - 1;
        animation_types.insert(
            dir_name, 
            AnimationType(start_animation_id, end_animation_id)
        );
        filenames.extend(move_images_filenames);
    }
    // return Ok(());
    // let (filenames, name_to_id) = find_all_format(
    //     images_path,
    //     ".png",
    // );
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
    };
    let raw_animation = RawAnimation{
        texture_filename: "packed".to_string(),
        texture_dimentions: (w_num as usize, h_num as usize),
        animation_types: animation_types,

    };
    fs::write(
        "./packed.ron", 
        ron::ser::to_string(&raw_animation).unwrap()
    ).expect("can't write map");
    imgbuf.save("packed.png").unwrap();

    Ok(())
}
