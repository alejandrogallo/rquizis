extern crate yaml_rust;
use std::fs;
use std::io::Read;
use std::vec;

pub fn from_file(file_name: &String) -> vec::Vec<yaml_rust::Yaml> {
    let mut fd = fs::File::open(&file_name).expect("Unable to open file");
    let mut data = String::new();
    fd.read_to_string(&mut data).expect("Unable to read file");
    println!("{}", data);
    let docs = yaml_rust::YamlLoader::load_from_str(&data).unwrap();
    docs
}
