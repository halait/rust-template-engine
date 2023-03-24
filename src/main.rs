use std::{env, fs, path::Path};
use std::io::prelude::*;

use yartl_engine::render;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Usage: yartl_engine path_to_source path_to_json_context")
    }
    let source = fs::read_to_string(&args[1])
        .expect("Should have been able to read the file");
    let json = fs::read_to_string(&args[2])
        .expect("Should have been able to read the file");
    let output = render(&source, &json);
    println!("{}", &output);
    let path = Path::new(&args[1]);
    let file_stem = path.file_stem().expect("Unable to parse source filename");
    let extension = path.extension().expect("Unable to parse source file extension");
    let mut file = fs::File::create([file_stem.to_str().unwrap(), "_yartle_out.", extension.to_str().unwrap()].join(""))
        .expect("Error writing file");
    file.write_all(output.as_bytes()).unwrap();
}