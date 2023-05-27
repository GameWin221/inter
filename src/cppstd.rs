use std::{fs::File, io::Read};

pub fn get_cpp_header(name: &str) -> String {
    let mut buffer = String::with_capacity(256);

    let mut file = File::open(format!("./cppstd/{name}.hpp")).expect("Failed to open an std file!");
    file.read_to_string(&mut buffer).expect("Failed to read an std file!");

    buffer
}