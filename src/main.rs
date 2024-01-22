// use std::path::PathBuf;

use clap::Parser;
use std::fmt::Debug;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use svd_parser;
use svd_parser::svd::peripheral;

#[derive(Parser, Debug)]
#[command(name = "svd2rust-ng-version-:)")]
#[command(author = "Just me")]
#[command(version = "1.0")]
#[command(about = "Generate Rust register maps (`struct`s) from SVD files", long_about = None)]
// #[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Args {
    /// input .svd file to process
    #[arg(short, long)]
    input: Option<String>,
}

fn main() {
    let args: Args = Args::parse();

    let user_input_file_path = match &args.input {
        Some(input) => input,
        None => panic!("Hey, where is input file? You forgot?"),
    };

    let input_path = Path::new(&user_input_file_path);
    let mut xml_buff = String::new();
    let mut input_file = match File::open(input_path) {
        Err(why) => panic!("could not open file: {}", why),
        Ok(file) => file,
    };

    let num_of_bytes_readed = input_file.read_to_string(&mut xml_buff).unwrap();
    println!("num_of_bytes_readed: {}", num_of_bytes_readed);
    // println!("readed content: {}", xml_buff);

    let mut device = match svd_parser::parse(&xml_buff) {
        Err(why) => panic!("could not parse file: {}", why),
        Ok(file) => file,
    };

    for peripheral in device.peripherals {
        if peripheral.name.to_ascii_lowercase().contains("GPIOA") {
            
        }
    }

    ()
}
