mod fft;

use clap::Parser;
use fft::{fft_2d, read_img, img_resize, is_base2};
use std::process::exit;

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
    let img = match read_img(&args.image) {
        Ok(img) => {
            let w = img.width();
            let h = img.height();
            if !is_base2(w) || !is_base2(h) {
                img_resize(&img, h, w)
            } else {
                img
            }
        },
        Err(e) => {
            eprintln!("unable to open image: {}", e);
            exit(1);
        }
    };
    let watermark = match read_img(&args.watermark) {
        Ok(watermark) => {
            let w = watermark.width();
            let h = watermark.height();
            if !is_base2(w) || !is_base2(h) {
                img_resize(&watermark, h, w)
            } else {
                watermark
            }
        },
        Err(e) => {
            eprintln!("unable to open watermark: {}", e);
            exit(1);
        }
    };

    fft_2d(img, watermark);

}
