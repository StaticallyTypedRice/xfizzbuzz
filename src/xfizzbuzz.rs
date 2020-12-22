//use std::env;
use std::process;

pub struct Word {
    word: String,
    multiple_of: u32,
}

impl Word {
    pub fn from_str(string: &str) -> Word {
        let split_string: Vec<&str> = string.split(":").collect();
        if split_string.len() != 2 {
            eprintln!("Error: Incorrectly formatted word argument '{}'", string);
            process::exit(2)
        }
        Word {
            word: split_string[1].to_string(),
            multiple_of: match split_string[0].parse::<u32>() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!(
                        "Error: Invalid factor on '{}': Please enter an integer.",
                        string
                    );
                    process::exit(4)
                }
            },
        }
    }
}

pub fn xfizzbuzz(max_number: &u32, words: &Vec<Word>) -> String {
    let mut output: String = String::new();
    for i in 1..=*max_number {
        let mut line: String = String::new();

        for word in words {
            if i % word.multiple_of == 0 {
                line += &word.word;
            }
        }

        if line.is_empty() {
            output += &i.to_string();
        } else {
            output += &line;
        }

        output += "\n";
    }
    output
}
