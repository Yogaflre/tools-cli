use clap::{arg, ArgGroup, Command};

trait InputArgs {
    fn input_args(self, required: bool) -> Self;
}

impl InputArgs for Command {
    fn input_args(self, required: bool) -> Self {
        self.arg(arg!([VALUE]).required(false))
            .arg(arg!(-i --input <PATH> "Input file path").required(false))
            .group(
                ArgGroup::new("input_args")
                    .args(&["VALUE", "input"])
                    .required(required),
            )
    }
}

pub fn cli() -> Command {
    return Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg_required_else_help(true)
        .subcommand(json_command())
        .subcommand(time_command())
        .subcommand(coding_command())
        .arg(arg!(-o --output <PATH> "Output file path").required(false));
}

fn time_command() -> Command {
    Command::new("time")
        .about("Time tools")
        .arg(arg!(-n --now "Current time"))
        .arg(arg!(-f --format "Format time").requires("input_args"))
        .group(ArgGroup::new("opt").args(&["now", "format"]).required(true))
        .input_args(false)
}

fn coding_command() -> Command {
    Command::new("coding")
        .about("Data coding tools")
        .arg(arg!(-e --encoding "Data encoding"))
        .arg(arg!(-d --decoding "Data decoding"))
        .group(
            ArgGroup::new("opt")
                .args(&["encoding", "decoding"])
                .required(true),
        )
        .arg(arg!(-s --base64 "Base64"))
        .arg(arg!(-t --base32 "Base32"))
        .arg(arg!(-H --base16 "Base16 / Hex"))
        .group(
            ArgGroup::new("type")
                .args(&["base64", "base32", "base16"])
                .required(true),
        )
        .input_args(true)
}

fn json_command() -> Command {
    Command::new("json")
        .about("Json tools")
        .arg(arg!(-f --format "Format json string"))
        .arg(arg!(-c --compress "Compress json string"))
        .group(
            ArgGroup::new("opt")
                .args(&["format", "compress"])
                .required(true),
        )
        .input_args(true)
}
