pub mod coding;
pub mod command;
pub mod io;
pub mod parser;
pub mod time;

use clap::ArgMatches;
use coding::base::{self, BaseType};
use io::Type;
use parser::json;
use time::formatter;

fn main() {
    let matches = command::cli().get_matches();

    let res: String = match matches.subcommand() {
        Some(("time", command)) => {
            if command.contains_id("now") {
                formatter::now()
            } else {
                formatter::convert(&get_value(&command))
            }
        }
        Some(("coding", commamd)) => {
            let base_type = match (
                commamd.contains_id("base64"),
                commamd.contains_id("base32"),
                commamd.contains_id("base16"),
            ) {
                (true, _, _) => BaseType::Base64,
                (_, true, _) => BaseType::Base32,
                (_, _, true) => BaseType::Base16,
                _ => unreachable!(),
            };
            let encode: bool = commamd.contains_id("encoding");
            base::convert(&get_value(&commamd), base_type, encode)
        }
        Some(("json", command)) => {
            if command.contains_id("format") {
                json::format(&get_value(&command))
            } else {
                json::compress(&get_value(&command))
            }
        }
        _ => unreachable!(),
    };

    let output_type = match matches.get_one::<&str>("output") {
        Some(path) => Type::File(path),
        None => Type::Console(None),
    };
    io::output(res, output_type);
}

fn get_value(cmd: &ArgMatches) -> String {
    return cmd
        .get_one::<&str>("VALUE")
        .map(|v| v.to_string())
        .or(cmd
            .get_one::<&str>("input")
            .map(|path| io::input_file(&path)))
        .unwrap();
}
