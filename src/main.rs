mod fft;

use fft::{read_img, fft_2d};

use std::env::args;

fn main() {
    let argv: Vec<String> = args().collect();

    let mut img = read_img(&argv[1]);
    fft_2d(&mut img);

}
