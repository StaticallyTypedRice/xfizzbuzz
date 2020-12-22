use crate::xfizzbuzz;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "xfizzbuzz", about = "An extensible fizzbuzz implementation.")]
pub struct Cli {
    #[structopt(default_value = "15", help = "The maximum number to count to.")]
    pub max_number: u32,

    #[structopt(
        default_value = "3:fizz 5:buzz",
        help = "The words to replace the number with when it is a multiple of a number, formatted as a space-separated list of '<number>:<word>'. The order of the words specified in the arguments will be the order the words are placed if a number is qualified for more than one word."
    )]
    pub words: Vec<String>,
}

pub fn parse_words_from_command_line(cli: &Cli) -> Vec<xfizzbuzz::Word> {
    let mut output: Vec<xfizzbuzz::Word> = Vec::new();
    for word in &cli.words {
        output.push(xfizzbuzz::Word::from_str(&word));
    }
    output
}
