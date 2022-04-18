use clap::{arg, Arg, ArgGroup, Command};
use output::Type;
use time::formatter;

pub mod coding;
mod output;
pub mod time;

fn main() {
    let matches = cli().get_matches();

    let output_type = match matches.value_of("output") {
        Some(path) => Type::File(path),
        None => Type::Console,
    };

    let res = match matches.subcommand() {
        Some(("time", command)) => {
            if let Some(time) = command.value_of("format") {
                formatter::convert(time)
            } else {
                formatter::now()
            }
        }
        _ => unreachable!(),
    };
    output::output(res, output_type);
}

fn cli() -> Command<'static> {
    return Command::new("tools-cli")
        .version("0.1.0")
        .author("Yogafire <yogafirew@gmail.com>")
        .about("Development tools")
        .arg_required_else_help(true)
        .arg(arg!(-o --output <PATH> "Result output path").required(false))
        .subcommand(
            Command::new("time")
                .about("Time tools")
                .arg(arg!(-n --now "Current time"))
                // FIXME <***> value is unclear in arg marco.
                // .arg(arg!(-f --format <TIME> "Format time"))
                .arg(
                    Arg::new("format")
                        .short('f')
                        .long("format")
                        .help("Format time")
                        .takes_value(true)
                        .forbid_empty_values(true),
                )
                .group(
                    ArgGroup::new("time_args")
                        .args(&["now", "format"])
                        .required(true),
                ),
        );
}
