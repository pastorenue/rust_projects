use clap::{Arg, ArgMatches, Command, Parser};
use clap::error::ErrorKind;


#[derive(Parser, Debug)]
#[command(
    author = "Emmanuel Pastor <@empastor>",
    version = "1.0",
    about = "A simple todo list CLI",
    long_about = None
)]
pub struct Args {
    #[arg(short, long)]
    pub name: String,
    #[arg(short='c', long)]
    pub number: i32,
    #[arg(short='f', long, default_value_t = false)]
    pub flag: bool,
    #[arg(short='t', long, default_value_t = 2)]
    pub count: u8,
}


/// .
pub fn build_cmd() -> Command {
    let m = Command::new("new program")
        .author("Emmanuel Pastor <@empastor>")
        .version("1.0")
        .about("A simple todo list CLI")
        .arg(
            Arg::new("in_file")
                .short('i')
                .help("Input file")
        )
        .mut_arg("in_file", |a| a.default_value("foo.txt").short('f'))
        .after_help("More help text. Used when displaying the help information \
            from --help or -h");
    
    m
}

pub fn playing_with_commands(matches: ArgMatches) {
    if let Some(file) = matches.get_one::<String>("in_file") {
        println!("in_file: {}", file);
    }
}

#[allow(dead_code)]
pub fn raise_err() -> clap::Error {
    let mut cmd = build_cmd();
    let err = cmd.error(ErrorKind::InvalidValue, "invalid value");
    err
}
