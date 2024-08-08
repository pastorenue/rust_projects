use clap::{Arg, ArgMatches, Command, Parser};

fn main() {
    let args = Args::parse();
    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }
}

#[derive(Parser, Debug)]
#[command(author = "Emmanuel Pastor <@empastor>", version = "1.0", about = "A simple todo list CLI", long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String,
    #[arg(short='c', long)]
    number: i32,
    #[arg(short='f', long, default_value_t = false)]
    flag: bool,
    #[arg(short='t', long, default_value_t = 2)]
    count: u8,
}


/// .
fn build_cmd() -> Command {
    let m = Command::new("new program")
        .author("Emmanuel Pastor <@empastor>")
        .version("1.0")
        .about("A simple todo list CLI")
        .arg(
            Arg::new("in_file")
                .required(false)
        )
        .after_help("More help text. Used when displaying the help information \
            from --help or -h");
    
    m
}

fn playing_with_commands(matches: ArgMatches) {
    if let Some(file) = matches.get_one::<String>("in_file") {
        println!("in_file: {}", file);
    }
}

mod tests {
    use clap::{arg, Command};
    use super::*;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    #[test]
    fn test_command() {
        let cmd = Command::new("hello")
            .args(&[
                arg!([name] "name to print"),
                arg!(--config <config_file> "configuration file to use"),
                arg!(-d --debug ... "turn on debugging"),
            ]);
        
        let m = cmd.try_get_matches_from(["hello", "--config", "foo.yaml"]).unwrap();

        assert_eq!(m.get_one::<String>("config").unwrap(), "foo.yaml");
        assert_eq!(*m.get_one::<u8>("debug").unwrap(), 0);
    }

    #[test]
    fn test_playing_with_commands() {
        // Build the command
        let cmd = build_cmd();

        // Parse the command line arguments
        let matches = cmd.get_matches_from(vec!["new program", "in_file"]);

        // Process the matches
        let output = catch_unwind(AssertUnwindSafe(|| {
            playing_with_commands(matches);
        }));

        // Check the output
        // let out_str = format!("{:?}", output);
        // assert!(out_str.contains("in_file: text.txt"));

        assert_eq!(output.is_ok(), true);
    }
}
