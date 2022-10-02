#![allow(unused)]

use image::imageops::FilterType::Nearest;
/// maths libs
use libm;
use num::complex::Complex;

/// standard libs
use std::f64::consts::PI;
use std::io::Cursor;

/// libs for read images properly
/// jpg, png has its own encoding
/// need to parse the encoding
/// inorder to get correct rbg vectors
use image::io::Reader as ImageReader;
use image::{GenericImage, Pixel, Pixels, DynamicImage};

/// open a image file
/// return a dynamic image enum
pub fn read_img(filename: &str) -> DynamicImage {
    let mut img = ImageReader::open(filename)
    .expect("Unable to open img file")
    .decode()
    .expect("unable to decode image")
    ;

    img
}

fn is_base2(n: u32) -> bool {
    if (n & (n - 1)) == 0 {
        true
    } else {
        false
    }
}

fn img_resize(img: &mut DynamicImage, height: &mut u32, width: &mut u32) {
    let mut original_height = *height as f32;
    let mut original_width = *width as f32;

    let mut new_height = 2;
    let mut new_width = 2;

    let mut counter_r = 0;
    let mut counter_c = 0;

    while original_height / 2.0 > 1.0 {
        original_height /= 2.0;
        counter_c += 1;
    }

    while original_width / 2.0 > 1.0 {
        original_width /= 2.0;
        counter_r += 1;
    }

    while counter_c > 0 {
        new_height *= 2;
        counter_c -= 1;
    }

    while counter_r > 0 {
        new_width *= 2;
        counter_r -= 1;
    }
    
    *height = new_height;
    *width = new_width;
    img.resize(new_width, new_height, Nearest);

}

pub fn fft_2d(img: &mut DynamicImage) {

    let mut width = img.width();
    let mut height = img.height();
    

    if !is_base2(width) || !is_base2(height) {
        println!("image dimensions must be in base of 2, currently {} x {}, resizing...", width, height);
        img_resize(img, &mut width, &mut height);
        println!("resized to {} x {}", width, height);
    }


}

/*
fn num_base2(n: u32) -> i32 {
    let mut result = 0;
    let mut num = n;
    
    while num != 1 {
        if num % 2 == 0 {
            result += 1;
            num /= 2;
        } else {
            result = -1;
            break;
        }
    }

    result
}
*/

/*
    +==========================+
    |   test sections          |
    +==========================+
*/

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_base2() {
        assert_eq!(is_base2(8), true);
    }

    #[test]
    fn test_not_base2() {
        assert_eq!(is_base2(9), false);
    }
}
