use std::{
    fs::OpenOptions,
    io::{Read, Write},
};

pub enum Type<'a> {
    Console(Option<&'a str>),
    File(&'a str), // path
}

pub fn input_file(path: &str) -> String {
    let mut file = OpenOptions::new().read(true).open(path).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    return buffer;
}

pub fn output(content: String, t: Type) {
    match t {
        Type::Console(_) => output_console(content),
        Type::File(path) => output_file(content, path),
    }
}

fn output_console(content: String) {
    println!("{}", content);
}

fn output_file(content: String, path: &str) {
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(path)
        .unwrap();
    file.write_all(content.as_bytes()).unwrap();
}
