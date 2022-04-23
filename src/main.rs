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
            if command.is_present("now") {
                formatter::now()
            } else {
                formatter::convert(&get_value(&command))
            }
        }
        Some(("coding", commamd)) => {
            let base_type = match (
                commamd.is_present("base64"),
                commamd.is_present("base32"),
                commamd.is_present("base16"),
            ) {
                (true, _, _) => BaseType::Base64,
                (_, true, _) => BaseType::Base32,
                (_, _, true) => BaseType::Base16,
                _ => unreachable!(),
            };
            let encode: bool = commamd.is_present("encoding");
            base::convert(&get_value(&commamd), base_type, encode)
        }
        Some(("json", command)) => {
            if command.is_present("format") {
                json::format(&get_value(&command))
            } else {
                json::compress(&get_value(&command))
            }
        }
        _ => unreachable!(),
    };

    let output_type = match matches.value_of("output") {
        Some(path) => Type::File(path),
        None => Type::Console(None),
    };
    io::output(res, output_type);
}

fn get_value(cmd: &ArgMatches) -> String {
    return cmd
        .value_of("VALUE")
        .map(|v| v.to_string())
        .or(cmd.value_of("input").map(|path| io::input_file(&path)))
        .unwrap();
}
