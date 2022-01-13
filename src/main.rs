extern crate image;
use image::{save_buffer, DynamicImage, GenericImageView};
use std::process::Command;
use std::{io, thread};

const DEST: &str = "/tmp/lock-blur.png";
fn main() -> io::Result<()> {
    let count = thread::available_parallelism()?get();
    println!("Using {} threads", count);
    // create a temp image
    let mut command = Command::new("grim")
        .arg("/tmp/lock.png")
        .output()
        .expect("failed to execute process");
    let image = image::open("/tmp/lock.png").unwrap();
    let blur_image = image.blur(5.0);
    generate_image(DEST, &blur_image);
    command = Command::new("swaylock")
        .arg("-i")
        .arg("/tmp/lock.png")
        .output()
        .expect("failed to execute process");

    Ok(())
}

fn generate_image(DEST: &str, blur_image: &DynamicImage) {
    save_buffer(
        DEST,
        &blur_image.to_rgb8(),
        blur_image.width(),
        blur_image.height(),
        image::ColorType::Rgb8,
    )
    .unwrap();
}
