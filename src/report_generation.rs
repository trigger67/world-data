use crate::*;

use image::{GenericImageView, ImageBuffer, RgbaImage};
const TITLE_HEIGHT: u32 = 100;

pub fn generate_all_reports() {
    generate_death_full();
    generate_death_part();
}

pub fn generate_death_full() {
    let files =
        tools::get_files_from_dir(String::from(format!("output/{}", REPORT1_NAME))).unwrap();

    let height: f64 = TITLE_HEIGHT as f64 + 400.0 * (files.len() as f64 / 2.0).ceil();
    let mut img: RgbaImage = ImageBuffer::new(600 * 2, height as u32);

    print_image_to_png(format!("data/other/report_1_title.png"), &mut img);

    for (i, file) in files.iter().enumerate() {
        let i = i as u32;
        let current_image = image::open(file).unwrap();

        for pixel in current_image.pixels() {
            img.put_pixel(
                pixel.0 + ((i % 2) * 600),
                pixel.1 + TITLE_HEIGHT + (i / 2) * 400,
                pixel.2,
            );
        }
    }

    img.save(format!("output/report/{}.png", REPORT1_NAME))
        .unwrap();
}

pub fn generate_death_part() {
    let files =
        tools::get_files_from_dir(String::from(format!("output/{}", REPORT2_NAME))).unwrap();

    let height: f64 = TITLE_HEIGHT as f64 + 400.0 * (files.len() as f64 / 2.0).ceil();
    let mut img: RgbaImage = ImageBuffer::new(600 * 2, height as u32);

    print_image_to_png(String::from("data/other/report_2_title.png"), &mut img);

    for (i, file) in files.iter().enumerate() {
        let i = i as u32;
        let current_image = image::open(file).unwrap();

        for pixel in current_image.pixels() {
            img.put_pixel(
                pixel.0 + ((i % 2) * 600),
                pixel.1 + TITLE_HEIGHT + (i / 2) * 400,
                pixel.2,
            );
        }
    }
    img.save(format!("output/report/{}.png", REPORT2_NAME))
        .unwrap();
}

pub fn print_image_to_png(image_path: String, img: &mut RgbaImage) {
    let image_to_print = image::open(image_path).unwrap();

    for pixel in image_to_print.pixels() {
        img.put_pixel(pixel.0, pixel.1, pixel.2);
    }
}
