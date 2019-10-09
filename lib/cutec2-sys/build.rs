extern crate cc;
extern crate bindgen;
use std::env;
use std::path::PathBuf;
fn main() {
    cc::Build::new().file("src/cutec2.c").compile("cutec2");

}
