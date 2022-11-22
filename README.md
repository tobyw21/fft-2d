# fft-2d
apply fft on image processing

Setup Information
=================
The essential toolchain to build this project is [rustup](https://rustup.rs/), follow the guide to install if you want to run it on Debian and Arch based Linux Distribution.

Once `rustup` is installed, run following command to check if tool chain is successfully installed.

`cargo --version`


Build and Run
==============
To run the code
`cargo run -- <file path>`


Important Information and Notes
===============================
- This project is never tested on Windows based systems and WSL2.
- If you want to run it on the UNSW CSE, maybe consider borrowing `cs6991`'s `cargo` e.g. `6991 cargo run -- <file path>`. However, this doesn't guarantee a successful build on the CSE server since it has not been tested on the CSE environment.
- The image format supported only includes `jpg` and `jpeg`.
