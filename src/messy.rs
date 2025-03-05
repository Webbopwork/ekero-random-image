#![feature(random)]
use std::random::random;

//use core::random::Random;

use ekero::prelude::*;
use std::{env, fs};

pub mod file;
use file::File;

// Run with "cargo run +nightly ..."
// Install "nightly" with "rustup install nightly"

struct State {
    images: Vec<Vec<u8>>
}

fn get_random(max_num: usize) -> usize {
    let random_num: usize = random();
    let percentage: f32 = random_num as f32 / usize::MAX as f32;
    (max_num as f32 * percentage).round() as usize
}

fn main() {
    clang_log::init(log::Level::Trace, "random_image");

    let images: Vec<Vec<u8>> = env::args().skip(1).map(|path| fs::read(path).unwrap()).collect();

    //let random_num: u8 = u8::random(&mut DefaultRandomSource);

    //let random_num: u128 = random();

    //let percentage: f32 = random_num as f32 / u128::MAX as f32;

    //log::info!("Images: {}\nRandom: {} percentage: {:#?} max: {}", images.len(), random_num, percentage, u128::MAX);
    log::info!("Images: {}", images.len());
    let mut app = App::new("0.0.0.0:8000", 20, State { images });

    app.get("/random", |ctx| {
        let images = {
            let lock = ctx.state_lock()?;
            lock.images.clone()
        };
        Ok(Response::new()
            .body(File { data: images[get_random(images.len()-1)].clone() })
            .status_code(202)
            .header("Content-Type", "image/webp"))
    });

    app.poll_forever()
    
}
