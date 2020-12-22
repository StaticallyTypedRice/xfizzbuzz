mod cli;
mod xfizzbuzz;

use structopt::StructOpt;

//use crate::{cli, xfizzbuzz};

fn main() {
    let args = cli::Cli::from_args();
    let words: Vec<xfizzbuzz::Word> = cli::parse_words_from_command_line(&args);
    println!("{}", xfizzbuzz::xfizzbuzz(&args.max_number, &words));
}
