pub mod compress;
use compress::huffman;
extern crate clap;
use clap::{App, Arg, ArgGroup};
use std::fs;
use std::io::Read;
use crate::fs::File;


fn get_file_as_byte_vec(filename: &String)->Vec<u8> {
    let error_msg = "Error reading file: ".to_string() + filename;
    let mut f = File::open(&filename).expect(&error_msg);
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");
    
    buffer
}

fn main(){
    let matches = App::new("Simple Compression Using Rust")
        .arg(
            Arg::with_name("compress")
                .short("c")
                .long("compress")
                .value_name("FILE")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("decompress")
                .short("d")
                .long("decompress")
                .value_name("FILE")
                .takes_value(true),
        )
        .group(
            ArgGroup::with_name("action")
                .args(&["compress","decompress"])
                .required(true),
        )
        .get_matches();
 
        if let Some(file) = matches.value_of("compress"){
            let byte_stream: Vec<u8> = get_file_as_byte_vec(&file.to_string());
            // println!("{:?}",byte_stream);
            let compressed_data = huffman::compress(byte_stream);
        }
}