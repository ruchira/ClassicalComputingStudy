extern crate turing_machine_rs;

use clap::Parser;
use regex::Regex;
use turing_machine_rs::instruction::{Move, State};
use turing_machine_rs::machines::Classic;
use turing_machine_rs::program::{Extend, Program};
use turing_machine_rs::state::Tape;
use turing_machine_rs::TuringMachine;

/// Reverse the input string using a Turing machine
#[derive(Parser)]
struct Cli {
    /// The text to reverse.
    #[arg(value_parser = is_binary_string)]
    text: String,
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
    let alphabet = vec!['0', '1', 'F', 'T', '_'];
    // The *external* alphabet is just vec!['0', '1']
    let mut program = Program::new(alphabet, State(10));
    program.extend([
        // Initial State: State 1

        // State 1: seeking-left (seeking left bit of a pair to transpose)

        // Current bit is 0; mark it with a prime (visited)
        // Transition to State 4: seeking-right(left-0)
        (1, '0', 4, 'F', Move::Right), 
        
        // Current bit is 1; mark it with a prime (visited)
        // Transition to State 5: seeking-right(left-1)
        (1, '1', 5, 'T', Move::Right),

        // Current bit is already marked: we are done swapping bits
        // Transition to State 2: seeking-blank
        (1, 'F', 2, 'F', Move::Right),
        (1, 'T', 2, 'F', Move::Right),

        // We have hit the end of the string
        // We are done, so unmark everything before this point
        // Transition to State 3: unmarking
        (1, '_', 3, '_', Move::Left),

        // State 2: seeking-blank

        // If we are on a non-blank, skip it and keep seeking right
        (2, '0', 2, '0', Move::Right),
        (2, '1', 2, '1', Move::Right),
        (2, 'F', 2, 'F', Move::Right),
        (2, 'T', 2, 'T', Move::Right),

        // We have found the blank
        // We are done, so unmark everything before this point
        // Transition to State 3: unmarking
        (2, '_', 3, '_', Move::Left),

        // State 3: unmarking

        // If we are not on a marked bit, skip it and keep seeking left
        (3, '0', 3, '0', Move::Left),
        (3, '1', 3, '1', Move::Left),
        (3, '_', 3, '_', Move::Left),   // Don't expect this to occur

        // If we are on a marked bit, unmark it and keep seeking left
        (3, 'F', 3, '0', Move::Left),
        (3, 'T', 3, '1', Move::Left),

        // The machine will halt when it runs into the beginning of the tape
        (3, '_', 0, '_', Move::None),
        
        // State 4: seeking-right(left-0) and State 5: seeking-right(left-1)

        // If we are on an unmarked bit, skip it and keep seeking right
        (4, '0', 4, '0', Move::Right),
        (5, '0', 5, '0', Move::Right),
        (4, '1', 4, '1', Move::Right),
        (5, '1', 5, '1', Move::Right),

        // When we come to a marked bit or a blank, we are done seeking right
        // Leave the current square as it is
        // Step once to the left to find the right-bit of the pair to swap
        // Transition from State 4 seeking-right(left-0)
        //              to State 6: step-left(left-0) 
        // Transition from State 5 seeking-right(left-1)
        //              to State 7: step-left(left-1)
        (4, 'F', 6, 'F', Move::Left),
        (5, 'F', 7, 'F', Move::Left),
        (4, 'T', 6, 'T', Move::Left),
        (5, 'T', 7, 'T', Move::Left),
        (4, '_', 6, '_', Move::Left),
        (5, '_', 7, '_', Move::Left),

        // State 6: step-left(left-0) and State 7: step-left(left-1)
        // The current square is the right bit of the pair to swap
        // Save the current value of the right bit into the new state
        // Write the value of the left bit into the current square
        // Mark the current square as visited
        // Start seeking back to the (marked) left bit to finish the swap
        // Transition to State 8: seeking-left(right-0)
        //            or State 9: seeking-left(right-1)
        //   depending on the old value of the current square
        (6, '0', 8, 'F', Move::Left),  // Square gets value 0 from left bit
        (7, '0', 8, 'T', Move::Left),  // Square gets value 1 from left bit
        (6, '1', 9, 'F', Move::Left),  // Square gets value 0 from left bit
        (7, '1', 9, 'T', Move::Left),  // Square gets value 1 from left bit

        // If we run into a marked bit here, we are done
        // Transition to State 2: seeking-blank
        (6, 'F', 2, 'F', Move::Right),
        (7, 'F', 2, 'F', Move::Right),
        (6, 'T', 2, 'T', Move::Right),
        (7, 'T', 2, 'T', Move::Right),

        // If we run into a blank we can start unmarking
        // Transition to State 3: unmarking
        (6, '_', 3, '_', Move::Left),   // Don't expect this to occur
        (7, '_', 3, '_', Move::Left),   // Don't expect this to occur
        
        // State 8: seeking-left(right-0) and State 9: seeking-left(right-1)

        // If we run into an unmarked bit, skip it and keep seeking left
        (8, '0', 8, '0', Move::Left),
        (9, '0', 9, '0', Move::Left),
        (8, '1', 8, '1', Move::Left),
        (9, '1', 9, '1', Move::Left),

        // When we come to a marked bit, write the value from the right bit
        // This completes the swap; seek the next inner pair of bits to swap
        // Move to the right 
        // Transition to State 1: seeking-left
        (8, 'F', 1, 'F', Move::Right),
        (9, 'F', 1, 'T', Move::Right),
        (8, 'T', 1, 'F', Move::Right),
        (9, 'T', 1, 'T', Move::Right),

        // If we encounter a blank, go into the unmarking state
        (8, '_', 3, '_', Move::Left),   // Don't expect this to occur
        (9, '_', 3, '_', Move::Left),   // Don't expect this to occur
        
    ])?;

    let machine = Classic::new(program, '_')?;
    
    let input = Tape::from(args.text+"_");
    println!("Input text: {:?}", input.as_vec().into_iter()
                                        .filter(|c| **c != '_')
                                        .collect::<String>());
    let output = machine.translate_nrm(input.clone())?;
    println!("Output text: {:?}", output.as_vec().into_iter()
                                        .filter(|c| **c != '_')
                                        .collect::<String>());

    Ok(())
}
