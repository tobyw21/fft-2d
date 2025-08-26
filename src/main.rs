#![allow(unused)]

mod fft;

use clap::Parser;
use fft::{fft_2d, read_img};
use num::complex::Complex64;
use std::{env::args, process::exit};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    image: String,

    #[arg(short, long)]
    watermark: String,
}

fn main() {
    let args = Args::parse();

    // load image and pipe into fft2d
    let mut img = match read_img(&args.image) {
        Ok(img) => img,
        Err(e) => {
            eprintln!("unable to open image: {}", e);
            exit(1);
        }
    };
    let mut watermark = match read_img(&args.watermark) {
        Ok(watermark) => watermark,
        Err(e) => {
            eprintln!("unable to open watermark: {}", e);
            exit(1);
        }
    };
    fft_2d(img, watermark);
}
