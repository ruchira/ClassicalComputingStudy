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
        if s.len() == 0 {
            Ok("0".to_string())
        } else {
            Ok(s.to_string())
        }
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
    let mut program = Program::new(alphabet, State(40));
    // If arg0 is n0 bits and arg1 is n1 bits, then the answer has at most 
    // max(n0, n1) + 1 bits
    program.extend([
        // Initial State: State 1

        // State-1: seeking-equals

        // If we find something other than '=', skip it & keep seeking right
        (1, '0', 1, '0', Move::Right),
        (1, '1', 1, '1', Move::Right),
        (1, '+', 1, '+', Move::Right),
        (1, 'F', 1, 'F', Move::Right),
        (1, 'T', 1, 'T', Move::Right),

        // When we find the '=', skip it and step right
        // Transition to State 2: marking-most-significant-bit
        (1, '=', 2, '=', Move::Right),

        // '_' is not expected to occur here

        // State-2: marking-most-significant-bit
        // Mark this as an indeterminate bit, possibly the most significant bit
        // of the answer
        // Transition to State 3: seeking-arg1-least-unmarked-bit
        (2, '_', 3, '?', Move::Left),

        // No other symbol is expected to occur here

        // State-3: seeking-arg1-least-unmarked-bit

        // When we find an unmarked bit, mark it
        // Transition to State 4: seeking-plus-arg0-least-unmarked-bit
        (3, '0', 4, 'F', Move::Left),
        (3, '1', 4, 'T', Move::Left),

        // If we find the '+', there are no unmarked bits left in arg1
        // Transition to State 5: seeking-arg0-alone-least-unmarked-bit
        (3, '+', 5, '+', Move::Left),

        // Skip marked bits and continue
        (3, 'F', 3, 'F', Move::Left),
        (3, 'T', 3, 'T', Move::Left),
        // Skip the equals sign and continue
        (3, '=', 3, '=', Move::Left),
        // Skip the '?' (previous indeterminate bits) and continue
        (3, '?', 3, '?', Move::Left), 

        // The '_' is not expected to occur here
        
        // State 4: seeking-plus-arg0-least-unmarked-bit

        // Skip unmarked bits (these are still in arg1) and continue
        (4, '0', 4, '0', Move::Left),
        (4, '1', 4, '1', Move::Left),

        // Found plus
        // Transition to State 6: seeking-arg0-least-unmarked-bit
        (4, '+', 6, '+', Move::Left),

        // State 5: seeking-arg0-alone-least-unmarked-bit
        
        // When we find an unmarked bit, mark it
        // Transition to State 7: add-one-indeterminate-bit-from-arg0
        (5, '0', 7, 'F', Move::Right),
        (5, '1', 7, 'T', Move::Right),

        // Skip marked bits and continue
        (5, 'F', 5, 'F', Move::Left),
        (5, 'T', 5, 'T', Move::Left),

        // Skip '+', '=', and '?' and continue
        (5, '+', 5, '+', Move::Left),
        (5, '=', 5, '=', Move::Left),
        (5, '?', 5, '?', Move::Left),

        // If we find a '_', there are no unmarked bits left in arg0 either
        // Transition to State 11: initialize
        (5, '_', 11, '_', Move::Right),
        
        // State 6: seeking-arg0-least-unmarked-bit
        
        // When we find an unmarked bit, mark it
        // Transition to State 8: add-one-indeterminate-bit-from-both
        (6, '0', 8, 'F', Move::Right),
        (6, '1', 8, 'T', Move::Right),

        // Skip marked bits and continue
        (6, 'F', 6, 'F', Move::Left),
        (6, 'T', 6, 'T', Move::Left),

        // If we find a '_', there are no unmarked bits left in arg0
        // Transition to State 9: add-one-indeterminate-bit-from-arg1
        (6, '_', 9, '_', Move::Right),

        // State 7: add-one-indeterminate-bit-from-arg0

        // Skip non-blank symbols and continue
        (7, '0', 7, '0', Move::Right),
        (7, '1', 7, '1', Move::Right),
        (7, '+', 7, '+', Move::Right),
        (7, 'F', 7, 'F', Move::Right),
        (7, 'T', 7, 'T', Move::Right),
        (7, '=', 7, '=', Move::Right),
        (7, '?', 7, '?', Move::Right),

        // On finding a blank, write an indeterminate bit
        // There are no unmarked bits left in arg1
        // Transition to State 5: seeking-arg0-alone-least-unmarked-bit
        (7, '_', 5, '?', Move::Left),

        // State 8: add-one-indeterminate-bit-from-both
        
        // Skip non-blank symbols and continue
        (8, '0', 8, '0', Move::Right),
        (8, '1', 8, '1', Move::Right),
        (8, '+', 8, '+', Move::Right),
        (8, 'F', 8, 'F', Move::Right),
        (8, 'T', 8, 'T', Move::Right),
        (8, '=', 8, '=', Move::Right),
        (8, '?', 8, '?', Move::Right),

        // On finding a blank, write an indeterminate bit
        // Transition to State 3: seeking-arg1-least-unmarked-bit
        (8, '_', 3, '?', Move::Left),

        // State 9: add-one-indeterminate-bit-from-arg1

        // Skip non-blank symbols and continue
        (9, '0', 9, '0', Move::Right),
        (9, '1', 9, '1', Move::Right),
        (9, '+', 9, '+', Move::Right),
        (9, 'F', 9, 'F', Move::Right),
        (9, 'T', 9, 'T', Move::Right),
        (9, '=', 9, '=', Move::Right),
        (9, '?', 9, '?', Move::Right),

        // On finding a blank, write an indeterminate bit
        // There are no unmarked bits left in arg0
        // Transition to State 10: seeking-arg1-alone-least-unmarked-bit
        (9, '_', 10, '?', Move::Left),
        
        // State 10: seeking-arg1-alone-least-unmarked-bit

        // When we find an unmarked bit, mark it
        // Transition to State 9: add-one-indeterminate-bit-from-arg1
        (10, '0', 9, 'F', Move::Right),
        (10, '1', 9, 'T', Move::Right),

        // Skip marked bits and continue
        (10, 'F', 10, 'F', Move::Left),
        (10, 'T', 10, 'T', Move::Left),

        // Skip '=' and '?' and continue
        (10, '=', 10, '=', Move::Left),
        (10, '?', 10, '?', Move::Left),

        // If we find a '+', there are no unmarked bits left in arg1 either
        // Transition to State 12: seeking-left-blank
        (10, '+', 12, '+', Move::Left),

        // State 11: initialize

        // Skip unmarked bits and continue
        (11, '0', 11, '0', Move::Right),
        (11, '1', 11, '1', Move::Right),
        
        // Skip '+' and continue
        (11, '+', 11, '+', Move::Right),

        // Unmark marked bits
        (11, 'F', 11, '0', Move::Right),
        (11, 'T', 11, '1', Move::Right),

        // Found '='
        // Transition to State 13: add-arg1-least-unmarked-bit
        (11, '=', 13, '=', Move::Left),

        // State 12: seeking-left-blank

        // Skip all non-blank characters and continue
        (12, '0', 12, '0', Move::Left),
        (12, '1', 12, '1', Move::Left),
        (12, '+', 12, '+', Move::Left),
        (12, 'F', 12, 'F', Move::Left),
        (12, 'T', 12, 'T', Move::Left),
        (12, '=', 12, '=', Move::Left),
        (12, '?', 12, '?', Move::Left),

        // Found '_'
        // Transition to State 11: initialize
        (12, '_', 11, '_', Move::Right),

        // State 13: add-arg1-least-unmarked-bit

        // Found unmarked bit
        // Mark the bit
        // Transition to State 14: find-arg0-add-least-unmarked-bit-to-0
        //            or State 15: find-arg0-add-least-unmarked-bit-to-1
        (13, '0', 14, 'F', Move::Left),
        (13, '1', 15, 'T', Move::Left),

        // Found '+'
        // There are no unmarked bits left in arg1
        // Transition to State 16: transfer-arg0-least-unmarked-bit
        (13, '+', 16, '+', Move::Left),

        // Skip marked bits and continue
        (13, 'F', 13, 'F', Move::Left),
        (13, 'T', 13, 'T', Move::Left),

        // State 14: find-arg0-add-least-unmarked-bit-to-0 and
        // State 15: find-arg0-add-least-unmarked-bit-to-1

        // Skip unmarked bits (of arg1) and continue
        (14, '0', 14, '0', Move::Left),
        (15, '0', 15, '0', Move::Left),
        (14, '1', 14, '1', Move::Left),
        (15, '1', 15, '1', Move::Left),

        // Found '+'
        // Transition to State 17: add-least-arg0-unmarked-bit-to-0 or
        //               State 18: add-least-arg0-unmarked-bit-to-1
        (14, '+', 17, '+', Move::Left),
        (15, '+', 18, '+', Move::Left),

        // State 16: transfer-arg0-least-unmarked-bit
        
        // Found unmarked bit
        // Mark it and transfer it
        // Transition to State 19: seek-end-of-sum-transfer-0
        //            or State 20: seek-end-of-sum-transfer-1
        (16, '0', 19, 'F', Move::Right),
        (16, '1', 20, 'T', Move::Right),

        // Found marked bit
        // Skip and continue
        (16, 'F', 16, 'F', Move::Left),
        (16, 'T', 16, 'T', Move::Left),

        // Found '_'
        // There are no unmarked bits left in arg0 either
        // Transition to State 21: unmark-right-and-halt
        (16, '_', 21, '_', Move::Right),

        // State 17: add-least-arg0-unmarked-bit-to-0 and
        // State 18: add-least-arg0-unmarked-bit-to-1

        // Found unmarked bit
        // Mark the bit
        // Transfer the sum to the least unmarked sum bit
        // Remember the carry if any
        // Transition to State 19: seek-end-of-sum-transfer-0
        //        or State 20: seek-end-of-sum-transfer-1
        //        or State 22: seek-end-of-sum-transfer-0-with-carry
        (17, '0', 19, 'F', Move::Right),
        (18, '0', 20, 'F', Move::Right),
        (17, '1', 20, 'T', Move::Right),
        (18, '1', 22, 'T', Move::Right),

        // Skip marked bits and continue
        (17, 'F', 17, 'F', Move::Left),
        (18, 'F', 18, 'F', Move::Left),
        (17, 'T', 17, 'T', Move::Left),
        (18, 'T', 18, 'T', Move::Left),

        // Found '_'
        // There are no unmarked bits left in arg0
        // Transfer the bit we have to the sum bit
        (17, '_', 19, '_', Move::Right),
        (18, '_', 20, '_', Move::Right),

        // State 19: seek-end-of-sum-transfer-0 or
        // State 20: seek-end-of-sum-transfer-1 or
        // State 22: seek-end-of-sum-transfer-0-with-carry
        
        // Skip non-blank characters and continue
        (19, '0', 19, '0', Move::Right),
        (20, '0', 20, '0', Move::Right),
        (22, '0', 22, '0', Move::Right),
        (19, '1', 19, '1', Move::Right),
        (20, '1', 20, '1', Move::Right),
        (22, '1', 22, '1', Move::Right),
        (19, '+', 19, '+', Move::Right),
        (20, '+', 20, '+', Move::Right),
        (22, '+', 22, '+', Move::Right),
        (19, 'F', 19, 'F', Move::Right),
        (20, 'F', 20, 'F', Move::Right),
        (22, 'F', 22, 'F', Move::Right),
        (19, 'T', 19, 'T', Move::Right),
        (20, 'T', 20, 'T', Move::Right),
        (22, 'T', 22, 'T', Move::Right),
        (19, '=', 19, '=', Move::Right),
        (20, '=', 20, '=', Move::Right),
        (22, '=', 22, '=', Move::Right),
        (19, '?', 19, '?', Move::Right),
        (20, '?', 20, '?', Move::Right),
        (22, '?', 22, '?', Move::Right),

        // Found blank character
        // Start seeking indeterminate character in sum to transfer to
        // Transition to State 23: seek-least-indeterminate-transfer-0
        //        or State 24: seek-least-indeterminate-transfer-1
        //        or State 25: seek-least-indeterminate-transfer-0-with-carry
        (19, '_', 23, '_', Move::Left),
        (20, '_', 24, '_', Move::Left),
        (22, '_', 25, '_', Move::Left),

        // State 21: unmark-right-and-halt

        // Skip unmarked bits and continue
        (21, '0', 21, '0', Move::Right),
        (21, '1', 21, '1', Move::Right),

        // Skip '+' and continue
        (21, '+', 21, '+', Move::Right),

        // Found marked bit
        // Unmark it and continue
        (21, 'F', 21, '0', Move::Right),
        (21, 'T', 21, '1', Move::Right),

        // Skip '=' and continue
        (21, '=', 21, '=', Move::Right),

        // Found '?'
        // Turn it into a '0' and continue
        (21, '?', 21, '0', Move::Right),

        // Found '_'
        // We are done, so halt
        (21, '_', 0, '_', Move::None),

        // State 23: seek-least-indeterminate-transfer-0
        // State 24: seek-least-indeterminate-transfer-1
        // State 25: seek-least-indeterminate-transfer-0-with-carry

        // Skip determinate bits and continue
        (23, '0', 23, '0', Move::Left),
        (24, '0', 24, '0', Move::Left),
        (25, '0', 25, '0', Move::Left),
        (23, '1', 23, '1', Move::Left),
        (24, '1', 24, '1', Move::Left),
        (25, '1', 25, '1', Move::Left),

        // Found indeterminate bit
        // Fill it with the appropriate sum
        // Transition to State 26: seeking-equals-no-carry
        //            or State 27: seeking-equals-with-carry
        (23, '?', 26, '0', Move::Left),
        (24, '?', 26, '1', Move::Left),
        (25, '?', 27, '0', Move::Left),

        // State 26: seeking-equals-no-carry

        // Skip indeterminate bits
        (26, '?', 26, '?', Move::Left),
        
        // Found '=' sign
        // Transition to State 13: add-arg1-least-unmarked-bit
        (26, '=', 13, '=', Move::Left),

        // State 27: seeking-equals-with-carry

        // Skip indeterminate bits
        (27, '?', 27, '?', Move::Left),

        // Found '=' sign
        // Transition to State 28: add-arg1-least-unmarked-bit-with-carry
        (27, '=', 28, '=', Move::Left),

        // State 28: add-arg1-least-unmarked-bit-with-carry

        // Found unmarked bit
        // Mark the bit
        // Transition to State 29: find-arg0-add-least-unmarked-bit-to-0-carry
        //            or State 30: find-arg0-add-least-unmarked-bit-to-1-carry
        (28, '0', 29, 'F', Move::Left),
        (28, '1', 30, 'T', Move::Left),

        // Found '+'
        // There are no unmarked bits left in arg1
        // Transition to State 31: transfer-arg0-least-unmarked-bit-carry
        (28, '+', 31, '+', Move::Left),

        // Skip marked bits and continue
        (28, 'F', 28, 'F', Move::Left),
        (28, 'T', 28, 'T', Move::Left),

        // State 29: find-arg0-add-least-unmarked-bit-to-0-carry and
        // State 30: find-arg0-add-least-unmarked-bit-to-1-carry

        // Skip unmarked bits (of arg1) and continue
        (29, '0', 29, '0', Move::Left),
        (30, '0', 30, '0', Move::Left),
        (29, '1', 29, '1', Move::Left),
        (30, '1', 30, '1', Move::Left),

        // Found '+'
        // Transition to State 32: add-least-arg0-unmarked-bit-to-0-carry or
        //               State 33: add-least-arg0-unmarked-bit-to-1-carry
        (29, '+', 32, '+', Move::Left),
        (30, '+', 33, '+', Move::Left),

        // State 31: transfer-arg0-least-unmarked-bit-carry
        
        // Found unmarked bit
        // Mark it, add the carry bit to it, and transfer the result
        // Transition to State 20: seek-end-of-sum-transfer-1
        //            or State 22: seek-end-of-sum-transfer-0-with-carry
        (31, '0', 20, 'F', Move::Right),
        (31, '1', 22, 'T', Move::Right),

        // Found marked bit
        // Skip and continue
        (31, 'F', 31, 'F', Move::Left),
        (31, 'T', 31, 'T', Move::Left),

        // Found '_'
        // There are no unmarked bits left in arg0 either
        // Transition to State 34: unmark-right-with-carry
        (31, '_', 34, '_', Move::Right),

        // State 32: add-least-arg0-unmarked-bit-to-0-carry and
        // State 33: add-least-arg0-unmarked-bit-to-1-carry

        // Found unmarked bit
        // Mark the bit
        // Transfer the sum including carry to the least unmarked sum bit
        // Remember the further carry if any
        // Transition to State 20: seek-end-of-sum-transfer-1
        //            or State 22: seek-end-of-sum-transfer-0-with-carry
        //            or State 35: seek-end-of-sum-transfer-1-with-carry
        (32, '0', 20, 'F', Move::Right),
        (33, '0', 22, 'F', Move::Right),
        (32, '1', 22, 'T', Move::Right),
        (33, '1', 35, 'T', Move::Right),

        // Skip marked bits and continue
        (32, 'F', 32, 'F', Move::Left),
        (33, 'F', 33, 'F', Move::Left),
        (32, 'T', 32, 'T', Move::Left),
        (33, 'T', 33, 'T', Move::Left),

        // Found '_'
        // There are no unmarked bits left in arg0
        // Transfer the bit we have to the sum bit
        (32, '_', 20, '_', Move::Right),
        (33, '_', 22, '_', Move::Right),

        // State 34: unmark-right-with-carry

        // Skip unmarked bits and continue
        (34, '0', 34, '0', Move::Right),
        (34, '1', 34, '1', Move::Right),

        // Skip '+' and continue
        (34, '+', 34, '+', Move::Right),

        // Found marked bit
        // Unmark it and continue
        (34, 'F', 34, '0', Move::Right),
        (34, 'T', 34, '1', Move::Right),

        // Found '=' sign
        // Transition to State 36: seeking-most-determined-sum-bit-last-carry
        (34, '=', 36, '=', Move::Right),

        // State 35: seek-end-of-sum-transfer-1-with-carry
        
        // Skip non-blank characters and continue
        (35, '0', 35, '0', Move::Right),
        (35, '1', 35, '1', Move::Right),
        (35, '+', 35, '+', Move::Right),
        (35, 'F', 35, 'F', Move::Right),
        (35, 'T', 35, 'T', Move::Right),
        (35, '=', 35, '=', Move::Right),
        (35, '?', 35, '?', Move::Right),

        // Found blank character
        // Start seeking indeterminate character in sum to transfer to
        // Transition to 
        //      State 37: seek-least-indeterminate-transfer-1-with-carry
        (35, '_', 37, '_', Move::Left),

        // State 36: seeking-most-determined-sum-bit-last-carry

        // Found determinate bit
        // Transition to State 38: write-final-carry
        (36, '0', 38, '0', Move::Left),
        (36, '1', 38, '0', Move::Left),

        // Skip indeterminate bit and continue
        (36, '?', 36, '?', Move::Right),

        // State 37: seek-least-indeterminate-transfer-1-with-carry

        // Skip determinate bits and continue
        (37, '0', 37, '0', Move::Left),
        (37, '1', 37, '1', Move::Left),

        // Found indeterminate bit
        // Fill it with the 1
        // Transition to State 27: seeking-equals-with-carry
        (37, '?', 27, '1', Move::Left),

        // State 38: write-final-carry

        // Found indeterminate bit
        // Fill it with the 1 from the final carry
        // Transition to State 39: zero-remaining-indeterminates
        (38, '?', 39, '1', Move::Left),

        // State 39: zero-remaining-indeterminates
        
        // Found indeterminate bit
        // Turn it into a 0 and continue
        (39, '?', 39, '0', Move::Left),

        // Found '=' sign
        // Halt
        (39, '=', 0, '=', Move::None),

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
