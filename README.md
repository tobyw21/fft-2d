# fft-2d
apply fft on image processing

Build and Run on UNSW CSE
===============
The project is tested on UNSW CSE environment and it can be successfully compiled on CSE. CSE has default `Rust` envrionment. However, it is outdated, fortunately, cs6991 provided an updated `Rust` toolchain, consider borrowing it. 

To run an example: `6991 cargo run sample/coffee.jpg sample/watermark.jpeg`

Setup Information for Devices Does not Have Rust Toolchain
==========================================================
The essential toolchain to build this project is [rustup](https://rustup.rs/), follow the guide to install if you want to run it on Debian and Arch based Linux Distribution.

Once `rustup` is installed, run following command to check if tool chain is successfully installed.

`cargo --version`


Build and Run
==============
To run, on current directory level,
`cargo run <img> <watermark>`


Important Information and Notes
===============================
- This project is never tested on Windows based systems and WSL2.
- The image format supported only includes `jpg` and `jpeg`.
- The watermark size CANNOT be greater than the image file.
- `cargo run` command is in development/debug mode, it doesn't represent the final performance of the program, to produce the build/compile artifact, [references](doc.rust-lang.org/cargo/commands/cargo-build.html).

Contact me
==========
tobywang21@gmail.com