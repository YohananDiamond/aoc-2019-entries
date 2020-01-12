// https://adventofcode.com/2019/day/5
// Intcode Computer v2.0!

fn main() {
    let puzzle_input = vec![3,225,1,225,6,6,1100,1,238,225,104,0,1101,91,67,225,1102,67,36,225,1102,21,90,225,2,13,48,224,101,-819,224,224,4,224,1002,223,8,223,101,7,224,224,1,223,224,223,1101,62,9,225,1,139,22,224,101,-166,224,224,4,224,1002,223,8,223,101,3,224,224,1,223,224,223,102,41,195,224,101,-2870,224,224,4,224,1002,223,8,223,101,1,224,224,1,224,223,223,1101,46,60,224,101,-106,224,224,4,224,1002,223,8,223,1001,224,2,224,1,224,223,223,1001,191,32,224,101,-87,224,224,4,224,102,8,223,223,1001,224,1,224,1,223,224,223,1101,76,90,225,1101,15,58,225,1102,45,42,224,101,-1890,224,224,4,224,1002,223,8,223,1001,224,5,224,1,224,223,223,101,62,143,224,101,-77,224,224,4,224,1002,223,8,223,1001,224,4,224,1,224,223,223,1101,55,54,225,1102,70,58,225,1002,17,80,224,101,-5360,224,224,4,224,102,8,223,223,1001,224,3,224,1,223,224,223,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,1008,677,677,224,102,2,223,223,1005,224,329,1001,223,1,223,1108,677,226,224,1002,223,2,223,1006,224,344,101,1,223,223,107,677,226,224,1002,223,2,223,1006,224,359,101,1,223,223,108,677,677,224,1002,223,2,223,1006,224,374,1001,223,1,223,108,226,677,224,1002,223,2,223,1006,224,389,101,1,223,223,7,226,677,224,102,2,223,223,1006,224,404,1001,223,1,223,1108,677,677,224,1002,223,2,223,1005,224,419,101,1,223,223,1008,226,677,224,102,2,223,223,1006,224,434,101,1,223,223,107,226,226,224,102,2,223,223,1005,224,449,1001,223,1,223,1007,677,677,224,1002,223,2,223,1006,224,464,1001,223,1,223,1007,226,226,224,1002,223,2,223,1005,224,479,101,1,223,223,1008,226,226,224,102,2,223,223,1006,224,494,1001,223,1,223,8,226,226,224,102,2,223,223,1006,224,509,101,1,223,223,1107,677,677,224,102,2,223,223,1005,224,524,1001,223,1,223,1108,226,677,224,1002,223,2,223,1006,224,539,101,1,223,223,1107,677,226,224,1002,223,2,223,1006,224,554,101,1,223,223,1007,677,226,224,1002,223,2,223,1005,224,569,101,1,223,223,7,677,226,224,1002,223,2,223,1006,224,584,101,1,223,223,107,677,677,224,1002,223,2,223,1005,224,599,1001,223,1,223,8,226,677,224,1002,223,2,223,1005,224,614,101,1,223,223,7,677,677,224,1002,223,2,223,1006,224,629,1001,223,1,223,1107,226,677,224,1002,223,2,223,1006,224,644,101,1,223,223,108,226,226,224,102,2,223,223,1005,224,659,1001,223,1,223,8,677,226,224,1002,223,2,223,1005,224,674,101,1,223,223,4,223,99,226];

    part_1(&puzzle_input);
    // part_2();
}

fn part_1(puzzle_input: &Vec<i32>) {
    let mut current_memory = puzzle_input.clone();
    compute(&mut current_memory);
    println!("\n* Resulting memory:\n  {:?}", current_memory);
}

fn compute(memory: &mut Vec<i32>) {
    let mut power = true;
    let mut i_cpos = 0usize; // Index of computer position
    let input = 1;

    while let Some(raw_command) = memory.get(i_cpos) {
        // Arg types:
        //  0 => Position Mode;
        //  1 => Immediate Mode;
        let mut command = raw_command.clone();
        let arg3_type = if command / 100_00 == 1 { command = command % 100_00; 1 } else { 0 };
        let arg2_type = if command / 010_00 == 1 { command = command % 010_00; 1 } else { 0 };
        let arg1_type = if command / 001_00 == 1 { command = command % 001_00; 1 } else { 0 };

        eprint!("[{:03}]: ", i_cpos);

        // Parse commands
        match command {
            // Command 1: add_memory_contents
            // Adds the contents of the memory on i_cposes `arg1` and `arg2` and replaces
            // the contents of memory i_cpos `arg3` with the result.
            // arg1 => operand_address_1;
            // arg2 => operand_address_2;
            // arg3 => dest_address;
            1 => {
                let arg1 = memory[i_cpos + 1] as usize;
                let arg2 = memory[i_cpos + 2] as usize;
                let arg3 = memory[i_cpos + 2] as usize;

                let value1 = if arg1_type == 0 { memory[arg1] } else { arg1 as i32 };
                let value2 = if arg2_type == 0 { memory[arg2] } else { arg2 as i32 };
                let dest = arg3;
                let sum = value1 + value2;

                eprintln!("{:?}: SUM {} + {} = {}, SENDTO ${:03} == {}",
                    &memory[i_cpos..i_cpos+4], value1, value2, sum, dest, memory[dest],
                );
                memory[dest] = sum;

                // Finishing
                i_cpos += 4;
            },

            // Command 2: multiply_memory_contents
            // Multiplies the contents of the memory on i_cposes `arg1` and `arg2` and replaces
            // the contents of memory i_cpos `arg3` with the result.
            // arg1 => operand_address_1;
            // arg2 => operand_address_2;
            // arg3 => dest_address;
            2 => {
                let arg1 = memory[i_cpos + 1] as usize;
                let arg2 = memory[i_cpos + 2] as usize;
                let arg3 = memory[i_cpos + 3] as usize;

                let value1 = if arg1_type == 0 { memory[arg1] } else { arg1 as i32 };
                let value2 = if arg2_type == 0 { memory[arg2] } else { arg2 as i32 };
                let dest = arg3;
                let mult = value1 * value2;

                eprintln!("{:?}: MULT {} * {} = {}, SENDTO ${:03} == {}",
                    &memory[i_cpos..i_cpos+4], value1, value2, mult, dest, memory[dest],
                );
                memory[dest] = mult;

                // Finishing
                i_cpos += 4;
            },

            // Command 3: input_to_position
            // Replaces the contents of `arg1` with the input.
            // arg1 => input_dest;
            3 => {
                let arg1 = memory[i_cpos + 1] as usize;
                let dest = if arg1_type == 0 { arg1 } else { (i_cpos + 1) as usize };

                eprintln!("{:?}: GET {}, SENDTO ${:03} == {}",
                    &memory[i_cpos..i_cpos+2], input, dest, memory[dest],
                );
                memory[dest] = input;

                // Finishing
                i_cpos += 2;
            },

            // Command 4: output_from_position
            // Outputs the contents of the address `arg1`.
            // arg1 => value_address;
            4 => {
                let arg1 = memory[i_cpos + 1] as usize;
                let orig = if arg1_type == 0 { arg1 } else { (i_cpos + 1) as usize };

                eprintln!("{:?}: OUT ${:03} == {}",
                    &memory[i_cpos..i_cpos+2], orig, memory[orig],
                );
                println!("{}", memory[orig]);

                // Finishing
                i_cpos += 2;
            },

            // Command 99: halt
            // Simply stops execution.
            99 => {
                eprintln!("{:?}: HALT", &memory[i_cpos..i_cpos+1]);
                power = false;

                // Finishing
                i_cpos += 1;
            }

            // Invalid commands handler
            _ => {
                eprintln!("Command {}: <INVALID>", command);
                power = false;
            },
        }

        // Break the loop if the program is halted.
        if !power { break; }
    }

    if power {
        panic!("Computer is still on after processing all commands.");
    }
}
