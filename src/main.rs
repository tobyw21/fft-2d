#![allow(unused)]

mod fft;

use fft::{read_img, fft_2d};
use std::env::args;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {

    #[arg(short, long)]
    image: String,

    #[arg(short, long)]
    watermark: String

}

fn main() {

    let args = Args::parse();


    // load image and pipe into fft2d
    let mut img = read_img(&args.image);
    let mut watermark = read_img(&args.watermark);
    fft_2d(img, watermark);

    
}

