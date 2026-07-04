extern crate turing_machine_rs;

use clap::Parser;
use regex::Regex;
use turing_machine_rs::instruction::{Move, State};
use turing_machine_rs::machines::Classic;
use turing_machine_rs::program::{Extend, Program};
use turing_machine_rs::state::Tape;
use turing_machine_rs::TuringMachine;

/// Add two binary numbers using a Turing machine
#[derive(Parser)]
struct Cli {
    /// The text to reverse.
    #[arg(value_parser = is_binary_string)]
    arg0: String,
    #[arg(value_parser = is_binary_string)]
    arg1: String,
}

fn is_binary_string(s: &str) -> Result<String, String> {
    let bin_re = Regex::new(r"^[0-1]*$").unwrap();
    if bin_re.is_match(s) {
        Ok(s.to_string())
    } else {
        Err(format!("Input is not a string of 0s and 1s"))
    }
}

fn main() -> Result<(), String> {
    let args = Cli::parse();

    // 'F' is the "marked" version of '0'
    // 'T' is the "marked" version of '1'
    // '?' is an indeterminate bit of the answer
    let alphabet = vec!['0', '1', '+', 'F', 'T', '=', '?', '_'];
    // The *external* alphabet is just vec!['0', '1', '+', '=']
    let mut program = Program::new(alphabet, State(2));
    program.extend([
        // Initial State: State 1

        (1, '0', 0, '0', Move::None),
        (1, '1', 0, '1', Move::None),
        (1, '+', 0, '+', Move::None),
        (1, 'F', 0, 'F', Move::None),
        (1, 'T', 0, 'T', Move::None),
        (1, '=', 0, '=', Move::None),
        (1, '_', 0, '_', Move::None),
        
    ])?;

    let machine = Classic::new(program, '_')?;
    
    let input = Tape::from(args.arg0+"+"+&args.arg1+"="+"_");
    println!("Input text: {:?}", input.as_vec()
                                        .into_iter()
                                        .filter(|c| **c != '_')
                                        .collect::<String>());
    let output = machine.translate_nrm(input.clone())?;
    println!("Output text: {:?}", output.as_vec()
                                        .into_iter()
                                        .filter(|c| **c != '_')
                                        .collect::<String>());

    Ok(())
}
