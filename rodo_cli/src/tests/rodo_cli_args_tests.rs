use crate::cmd_utils::rodo::{build_cmd, playing_with_commands};
use clap::{arg, Command};
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
    let matches = cmd.get_matches_from(vec!["new program", "-f", "text.txt"]);

    // Process the matches
    let output = catch_unwind(AssertUnwindSafe(|| {
        playing_with_commands(matches);
    }));

    // Check the output
    // let out_str = format!("{:?}", output);
    // assert!(out_str.contains("in_file: text.txt"));

    assert_eq!(output.is_ok(), true);
}

#[test]
fn test_mutated_arg_with_err() {
    let cmd = build_cmd();
    let matches = cmd.try_get_matches_from(["new program", "-i"]);
    assert!(matches.is_err());
}

#[test]
fn test_mutated_arg() {
    let cmd = build_cmd();
    let matches = cmd.try_get_matches_from(["new program", "-f", "text.txt"]);
    assert!(matches.is_ok());
}