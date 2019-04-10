#[macro_use]
extern crate serde_derive;
extern crate ron;
extern crate serde;
extern crate image;
extern crate clap;

use clap::{
    Arg, App,
};

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
use pack_pack::{
    AnimationType, RawAnimation,
    utils::{
        Description, load_image, find_all_subdirs, find_all_format
    },
};


fn main() -> std::io::Result<()> {
    let matches = App::new("PackPack")
        .version("0.1.0")
        .author("Vlad Zhukov")
        .about("2d texture packing(with animations)")
        .arg(
            Arg::with_name("textures")
                .required(true)
                .takes_value(true)
                .help("directory with textures to pack")
        )
        .arg(
            Arg::with_name("name")
                .required(true)
                .takes_value(true)
                .help("name which final files will have")
        )
        .arg(
            Arg::with_name("description_destination")
                .required(true)
                .takes_value(true)
                .help("description .ron file destination")
        )
        .arg(
            Arg::with_name("image_destination")
                .required(true)
                .takes_value(true)
                .help("image file destination")
        )
        .get_matches();
    // * PARSING ARGS
    let textures = matches.value_of("textures").unwrap();
    let packed_name = matches
        .value_of("name")
        .unwrap();
    let description_destination = matches
        .value_of("description_destination")
        .unwrap();
    let image_destination = matches
        .value_of("image_destination")
        .unwrap();
    
    // * READING DESCTIPTION FILE FOR TEXTURES
    let mut description_file = File::open(textures.to_string() + "/description.ron").expect("map not found");
    let mut description_string = String::new();
    description_file.read_to_string(&mut description_string);
    let description = match ron::de::from_str::<Description>(&description_string) {
        Ok(ok_description) => ok_description,
        Err(e) => {
            eprintln!("failed to load description  {} \n using default description", e);
            Description::default()
        }
    };
    // * READING AND PROCESSING IMAGES
    let images_path = Path::new(textures);
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
        let frame_ticks = match description.frame_times.get(&dir_name) {
            Some(frame_ticks) => {
                *frame_ticks
            },
            None => 1usize
        };
        animation_types.insert(
            dir_name, 
            AnimationType{
                start_id: start_animation_id, 
                end_id: end_animation_id,
                frame_ticks: frame_ticks,
            }
        );
        filenames.extend(move_images_filenames);
    }
    let mut images = vec!();
    for image_name in filenames.iter() {
        images.push(load_image(&textures, &image_name));
    };
    // !@ Assuming images dimentions are the same for now
    let (w_dim, h_dim) = images[0].dimensions();
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
        texture_filename: packed_name.to_string(),
        texture_dimentions: (w_num as usize, h_num as usize),
        animation_types: animation_types,

    };
    let description_file_destination = format!(
        "{}/{}.ron", 
        description_destination, 
        packed_name
    );
    let image_file_destination = format!(
        "{}/{}.png",
        image_destination,
        packed_name,
    );
    fs::write(
        &description_file_destination,
        ron::ser::to_string(&raw_animation).unwrap()
    ).expect("can't write map");
    imgbuf.save(&image_file_destination).unwrap();

    Ok(())
}
