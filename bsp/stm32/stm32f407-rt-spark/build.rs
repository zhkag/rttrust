// use std::env;

fn main() {
    // let out_dir = env::var("OUT_DIR").unwrap();
    println!("cargo:rustc-env=CARGO_TARGET_DIR=build");
}