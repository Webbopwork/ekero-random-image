#![feature(random)]
use std::random::random;

use ekero::prelude::*;
use std::{env, fs};

pub mod file;
use file::File;

// Run with "cargo run +nightly ..."
// Install "nightly" with "rustup install nightly"

struct State {
    images: Vec<Vec<u8>>,
    images_length: usize
}

fn get_random(max_num: usize) -> usize {
    let random_num: usize = random();
    let percentage: f32 = random_num as f32 / usize::MAX as f32;
    (max_num as f32 * percentage).round() as usize
}

fn main() {
    clang_log::init(log::Level::Trace, "random_image");

    let images: Vec<Vec<u8>> = env::args().skip(1).map(|path| fs::read(path).unwrap()).collect();

    log::info!("Images: {}", images.len());
    let mut app = App::new("0.0.0.0:8000", 20, State { images: images.clone(), images_length: &images.len()-1 });

    app.get("/random", |ctx| {
        let (images, images_length) = {
            let lock = ctx.state_lock()?;
            (lock.images.clone(), lock.images_length.clone())
        };
        Ok(Response::new()
            .body(File { data: images[get_random(images_length)].clone() })
            .status_code(202)
            .header("Content-Type", "image/webp"))
    });

    app.poll_forever()
    
}
