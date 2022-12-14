#![allow(unused)]


/// maths libs
use libm::{self, powf};
use num::complex::Complex;
use num::traits::Pow;
use rulinalg::matrix::{Matrix, BaseMatrix};
extern crate minifb;

use minifb::{Key, Window, WindowOptions};

/// standard libs
use std::f64::consts::PI;
use std::i128;
use std::io::{Cursor, Read};
use core::default::Default;
use std::ops::Div;

/// libs for read images properly
/// jpg, png has its own encoding
/// need to parse the encoding
/// inorder to get correct rbg vectors
use image::io::Reader as ImageReader;
use image::imageops::FilterType::Nearest;
use image::{GenericImage, Pixel, Pixels, DynamicImage};


/*
 *    +==========================+
 *    |   macros!                |
 *    +==========================+
 */

macro_rules! cmplx_new {
    ($re: expr, $im: expr) => {
        {
            Complex::new($re, $im)
        }
    };
}

macro_rules! cmplx_push {
    ($vec: expr; $re: expr, $im: expr) => {
        $vec.push(cmplx_new!($re, $im));
    };

    ($vec: expr, $cmplx: expr) => {
        $vec.push($cmplx);
    }
}

/// open a image file
/// return a dynamic image enum
/// input: filename
pub fn read_img(filename: &str) -> DynamicImage {
    let mut img = ImageReader::open(filename)
    .expect("Unable to open img file")
    .decode()
    .expect("unable to decode image");

    img
}

/// restore complex number vector into f32 vector
fn complex2f32(v: Vec<Complex<f32>>) -> Vec<f32> {
    let mut vf32: Vec<f32> = Vec::new();
    for i in v.iter() {
        let changed: f32 = i.re / v.len() as f32;
        vf32.push(changed);
    }
    drop(v);
    vf32
}

/// calculate the mod of complex number
fn mod_complex(v: Vec<Complex<f32>>) -> Vec<f32> {
    let mut rl32: Vec<f32> = Vec::new();
    for i in v.iter() {
        let moded_val = (i.re.powf(2.0) + i.im.powf(2.0)).sqrt();
        rl32.push(moded_val);
    }
    rl32
}

/// check if given number is base of 2
/// input n: int
fn is_base2(n: u32) -> bool {
    if (n & (n - 1)) == 0 {
        true
    } else {
        false
    }
}

/// resize image to the closest dimension
/// fft requires the dimensions of vectors to be base of 2
/// input
/// image: image object
/// height: int
/// width: int
fn img_resize(img: DynamicImage, height: &mut u32, width: &mut u32) -> DynamicImage {
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
    img.resize(new_width, new_height, Nearest)

}

/// https://cs.uwaterloo.ca/~kogeddes/cs487/LectureMaterials/Chapter_4_Materials/FFTalgorithm.pdf
/// http://paulbourke.net/miscellaneous/dft/
/// this is for fft a 1 dimensional vector
/// input
/// a: vector of complex numbers with im part 0
/// ifft is a flag to indicate if the value is inverse fft
/// changes vector reference `a' passed in to fourier coefficients generated by fft
fn fft_1d(a: &mut Vec<Complex<f32>>, ifft: bool) {
    let n = a.len();

    if n == 1 {
        return;
    }

    let mut a1: Vec<Complex<f32>> = vec![Default::default(); n >> 1];
    let mut a2: Vec<Complex<f32>> = vec![Default::default(); n >> 1];

    for i in (0..n).step_by(2) {
        a1[i >> 1] = a[i];
        a2[i >> 1] = a[i + 1];

    }

    fft_1d(&mut a1, ifft);
    fft_1d(&mut a2, ifft);

    let im_conjugate = if ifft {
        1.0
    } else {
        -1.0
    };

    let omega_n: Complex<f32> = Complex::new((2.0 * (PI as f32) / n as f32).cos(),
        im_conjugate * (2.0 * (PI as f32) / n as f32).sin());
    
    let mut omega = Complex::new(1.0, 0.0);

    for i in (0..n >> 1) {
        
        
        a[i] = a1[i] + omega * a2[i];
        a[i + (n >> 1)] = a1[i] - omega * a2[i];
        omega *= omega_n;
    }
    
    
}

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

pub fn display(width: u32, height: u32, display_vec: &Vec<u32>) {
    // set up window object for image display
    let mut window = Window::new(
        "Display Image",
        width as usize,
        height as usize,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
      }
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });
    
    // open a window for display image, will be dropped after function out of scope
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    window.update_with_buffer(&display_vec, width as usize, height as usize)
    .expect("unable to open window");
    
    // press any key to continue
    println!("Press Any Key to Continue");
    let _ = std::io::stdin().read(&mut [0]).unwrap();

}

pub fn fft_2d(img: DynamicImage, watermark: DynamicImage) {
    
    let mut width = img.width();
    let mut height = img.height();

    let mut new_img = img.clone();
    if !is_base2(width) || !is_base2(height) {
        println!("image dimensions must be in base of 2, currently {} x {}, resizing...", width, height);
        new_img = img_resize(img, &mut width, &mut height);
    }

    println!("now {} x {}", new_img.width(), new_img.height());
    
    let n = (width * height) as usize;

    let mut rgb_vec = 
    new_img.as_rgb8()
    .unwrap()
    .to_vec()
    .iter()
    .map(|&x| {
    
        cmplx_new!(x as f32, 0f32)
    
    })
    .collect::<Vec<Complex<f32>>>();
    
    let mut r_vec = Vec::new();
    let mut g_vec = Vec::new();
    let mut b_vec = Vec::new();

    for r in rgb_vec.iter().step_by(3) {
        r_vec.push(r.to_owned());
    }

    for g in rgb_vec.iter().skip(1).step_by(3) {
        g_vec.push(g.to_owned());
    }

    for b in rgb_vec.iter().skip(2).step_by(3) {
        b_vec.push(b.to_owned());
    }

    fft_1d(&mut r_vec, false);
    fft_1d(&mut g_vec, false);
    fft_1d(&mut b_vec, false);
    

    // process watermark
    let mut watermark_rgb_vec = 
    watermark.as_rgb8()
    .unwrap()
    .to_vec()
    .iter()
    .map(|&x| {
    
        cmplx_new!(x as f32, 0f32)
    
    })
    .collect::<Vec<Complex<f32>>>();
    
    let mut watermark_r_vec = Vec::new();
    let mut watermark_g_vec = Vec::new();
    let mut watermark_b_vec = Vec::new();

    for r in rgb_vec.iter().step_by(3) {
        watermark_r_vec.push(r.to_owned());
    }

    for g in rgb_vec.iter().skip(1).step_by(3) {
        watermark_g_vec.push(g.to_owned());
    }

    for b in rgb_vec.iter().skip(2).step_by(3) {
        watermark_b_vec.push(b.to_owned());
    }

    // FFT
    fft_1d(&mut r_vec, false);
    fft_1d(&mut g_vec, false);
    fft_1d(&mut b_vec, false);

    fft_1d(&mut watermark_r_vec, false);
    fft_1d(&mut watermark_g_vec, false);
    fft_1d(&mut watermark_b_vec, false);

    // process img here
    for idx_r in 0..watermark_r_vec.len() {
        r_vec[idx_r] *= 0.2;
        r_vec[idx_r] += watermark_r_vec[idx_r] * 0.8;
    }

    for idx_g in 0..watermark_g_vec.len() {
        g_vec[idx_g] *= 0.2;
        g_vec[idx_g] += watermark_g_vec[idx_g] * 0.8;
        // g_vec[idx_g].re /= 2.0;
        // g_vec[idx_g].im /= 2.0;
    }

    for idx_b in 0..watermark_b_vec.len() {
        b_vec[idx_b] *= 0.2;
        b_vec[idx_b] += watermark_b_vec[idx_b] * 0.8;
        // b_vec[idx_b].re /= 2.0;
        // b_vec[idx_b].im /= 2.0;
    }
    
    // IFFT
    fft_1d(&mut r_vec, true);
    fft_1d(&mut g_vec, true);
    fft_1d(&mut b_vec, true);
    

    let r_f32 = complex2f32(r_vec);
    let g_f32 = complex2f32(g_vec);
    let b_f32 = complex2f32(b_vec);
    
    // let r_f32 = mod_complex(r_vec);
    // let g_f32 = mod_complex(g_vec);
    // let b_f32 = mod_complex(b_vec);

    let mut display_vec = Vec::new();
    for i in 0..n {
        let r = from_u8_rgb(r_f32[i] as u8, g_f32[i] as u8, b_f32[i] as u8);
        display_vec.push(r)
    }

    display(width, height, &display_vec);
    

}


/*
 *   +==========================+
 *   |   test sections          |
 *   +==========================+
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

    #[test]
    fn test_fft1d() {
        let mut test_data = Vec::new();

        cmplx_push!(test_data; 1.0, 0.0);
        cmplx_push!(test_data; 2.0, 0.0);
        cmplx_push!(test_data; 4.0, 0.0);
        cmplx_push!(test_data; 7.0, 0.0);

        fft_1d(&mut test_data, false);

        dbg!(&test_data);
    }
}
