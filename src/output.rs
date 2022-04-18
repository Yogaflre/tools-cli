use std::{fs::OpenOptions, io::Write};

pub enum Type<'a> {
    Console,
    File(&'a str),
}

// TODO
// #[macro_export]
// macro_rules! output {
// }

pub fn output(content: String, t: Type) {
    match t {
        Type::Console => output_console(content),
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
