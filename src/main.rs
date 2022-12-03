#![allow(unused)]

mod fft;

use fft::{read_img, fft_2d};
use std::env::args;


fn main() {

    let argv: Vec<String> = args().collect();

    if argv.len() == 3 {
        // load image and pipe into fft2d
        let mut img = read_img(&argv[1]);
        let mut watermark = read_img(&argv[2]);
        fft_2d(img, watermark);
    } else {
        eprintln!("Usage: {} <img> <watermark>", &argv[0]);
    }
    
}

