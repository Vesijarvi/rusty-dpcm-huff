pub mod compress;
extern crate clap;
use clap::{App, Arg, ArgGroup};
use compress::huffman;
use std::fs;
use std::io::Read;
use crate::fs::File;

fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}
fn main() {
	let matches = App::new("Simple compression using rust")
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
				.args(&["compress", "decompress"])
				.required(true),
		)
		.get_matches();

	if let Some(file) = matches.value_of("compress") {
		let error_msg = "Error reading file: ".to_string() + file;
		// let data = fs::read_to_string(file).expect(&error_msg);
		let byte_stream = get_file_as_byte_vec(&file.to_string());
		let compressed_data = huffman::compress(&byte_stream);
		let output_file = file.to_string() + ".cmp";
		let error_msg = "Error writing file: ".to_string() + &output_file;
		fs::write(output_file, compressed_data).expect(&error_msg);
	}
	if let Some(file) = matches.value_of("decompress") {
		let error_msg = "Error reading file: ".to_string() + file;
		let data = fs::read(file).expect(&error_msg);
		let compressed_data = huffman::decompress(&data);
		let output_file = &file[0..file.len() - 4];
		let error_msg = "Error writing file: ".to_string() + &output_file;
		fs::write(output_file, compressed_data).expect(&error_msg);
	}
}