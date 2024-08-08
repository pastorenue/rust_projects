mod tests;
mod cmd_utils;

use crate::cmd_utils::rodo::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();
    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }
}
