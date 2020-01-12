// https://adventofcode.com/2019/day/5
// Intcode Computer v3.0!
// Got help mostly from this file: https://github.com/MrRobb/advent-of-code-2019/blob/master/src/day5.rs

/// Contains all available commands.
/// I stole this idea from the file I mentioned above, and the 3-character names are inspired
/// by assembly command syntax, idea which I also stole from someone, but I don't remember who.
enum Instruction {
    Add = 1, // Add
    Mul = 2, // Multiplication
    Inp = 3, // Input
    Out = 4, // Output
    Hlt = 99, // Halt
}

/// Parses an integer and returns a command and its parameter modes.
/// Return order: (command, p1mode, p2mode, p3mode)
/// For the `p*mode` arguments, false means position mode, true means immediate mode.
fn extract_command(mut command: i32) -> (i32, bool, bool, bool) {
    let p3mode = match command / 100_00 {
        0 => false,
        1 => { command %= 100_00; true },
        x => panic!("Invalid int in place for p3mode: {} at {}", x, command),
    };
    let p2mode = match command / 010_00 {
        0 => false,
        1 => { command %= 010_00; true },
        x => panic!("Invalid int in place for p3mode: {} at {}", x, command),
    };
    let p1mode = match command / 001_00 {
        0 => false,
        1 => { command %= 001_00; true },
        x => panic!("Invalid int in place for p3mode: {} at {}", x, command),
    };
    (command, p1mode, p2mode, p3mode)
}

/// Pipes an integer, `command`, through extract_command() and then parses the result, matching
/// the first number with an instruction.
fn parse_command(command: i32) -> (Instruction, bool, bool, bool) {
    let mut extraction = extract_command(command);
    (match extraction.0 {
        1 => Instruction::Add,
        2 => Instruction::Mul,
        3 => Instruction::Inp,
        4 => Instruction::Out,
        99 => Instruction::Hlt,
        _ => unimplemented!(),
    }, extraction.1, extraction.2, extraction.3)
}

/// Evaluates the current command and returns code for being interpreted by the function that
/// called it.
/// Return type is a tuple: (pos_add: usize, stop: bool)
/// pos_add: is the number to be added to the position.
/// stop: is a boolean indicating whether the machine is going to stop or not.
fn eval(memory: &mut Vec<i32>, position: usize, command: (Instruction, bool, bool, bool), input: i32) -> (usize, bool) {
    // Tip: on the match arms below, i32 variables usually mean values and usizes reference-only (considering the Intcode computer as the actual machine).
    match command.0 {
        // ADD OP1, OP2, &DEST
        Instruction::Add => {
            eprint!("{:?}: ADD ", &memory[position..position+4]);

            let param1: i32 = if command.1 { memory[position + 1] } else { memory[memory[position + 1] as usize] };
            let param2: i32 = if command.2 { memory[position + 2] } else { memory[memory[position + 2] as usize] };
            let param3: usize = if command.3 { panic!("param3 must be on address mode"); } else { memory[position + 3] as usize };

            eprint!("{}, {}, &{}\n", param1, param2, param3);
            memory[param3] = param1 + param2;
            (4, false)
        },

        // MUL OP1, OP2, &DEST
        Instruction::Mul => {
            eprint!("{:?}: MUL ", &memory[position..position+4]);

            let param1: i32 = if command.1 { memory[position + 1] } else { memory[memory[position + 1] as usize] };
            let param2: i32 = if command.2 { memory[position + 2] } else { memory[memory[position + 2] as usize] };
            let param3: usize = if command.3 { panic!("param3 must be on address mode"); } else { memory[position + 3] as usize };

            eprint!("{}, {}, &{}\n", param1, param2, param3);
            memory[param3] = param1 * param2;
            (4, false)
        },

        // INP &DEST
        Instruction::Inp => {
            eprint!("{:?}: INP ", &memory[position..position+2]);

            let param1: usize = if command.1 { panic!("param1 must be on address mode"); } else { memory[position + 1] as usize };

            eprint!("&{}\n", param1);
            memory[param1] = input;
            (2, false)
        },

        // OUT &ADDR
        Instruction::Out => {
            eprint!("{:?}: OUT ", &memory[position..position+2]);

            // let param1: usize = if command.1 { panic!("param1 must be on address mode"); } else { memory[position + 1] as usize };
            let param1: usize = memory[position + 1] as usize;

            eprint!("&{}\n", param1);
            println!("{}", memory[param1]);
            (2, false)
        },

        // HLT
        Instruction::Hlt => {
            eprint!("{:?}: HLT\n", &memory[position]);
            (0, true)
        },

        // ...
        _ => unimplemented!(),
    }
}

/// Takes the memory vector and uses the computer to run the code.
/// After running, returns the memory once taken, but with the modifications that were done at runtime.
fn init_computer(mut memory: Vec<i32>, input: i32) -> Vec<i32> {
    let mut power = true;
    let mut command_index = 0;
    let input = 1;

    loop {
        if let Some(cmd) = memory.get(command_index) {
            let command_raw = *cmd;
            let parsed_command = parse_command(command_raw);
            let (pos_add, stop) = eval(&mut memory, command_index as usize, parsed_command, input);

            command_index += pos_add;
            if stop { power = false; break; };
        } else if power {
            panic!("Computer is still on after processing all commands.");
        }
    }

    memory
}

fn main() {
    let mut memory: Vec<i32> = vec![3,225,1,225,6,6,1100,1,238,225,104,0,1101,91,67,225,1102,67,36,225,1102,21,90,225,2,13,48,224,101,-819,224,224,4,224,1002,223,8,223,101,7,224,224,1,223,224,223,1101,62,9,225,1,139,22,224,101,-166,224,224,4,224,1002,223,8,223,101,3,224,224,1,223,224,223,102,41,195,224,101,-2870,224,224,4,224,1002,223,8,223,101,1,224,224,1,224,223,223,1101,46,60,224,101,-106,224,224,4,224,1002,223,8,223,1001,224,2,224,1,224,223,223,1001,191,32,224,101,-87,224,224,4,224,102,8,223,223,1001,224,1,224,1,223,224,223,1101,76,90,225,1101,15,58,225,1102,45,42,224,101,-1890,224,224,4,224,1002,223,8,223,1001,224,5,224,1,224,223,223,101,62,143,224,101,-77,224,224,4,224,1002,223,8,223,1001,224,4,224,1,224,223,223,1101,55,54,225,1102,70,58,225,1002,17,80,224,101,-5360,224,224,4,224,102,8,223,223,1001,224,3,224,1,223,224,223,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,1008,677,677,224,102,2,223,223,1005,224,329,1001,223,1,223,1108,677,226,224,1002,223,2,223,1006,224,344,101,1,223,223,107,677,226,224,1002,223,2,223,1006,224,359,101,1,223,223,108,677,677,224,1002,223,2,223,1006,224,374,1001,223,1,223,108,226,677,224,1002,223,2,223,1006,224,389,101,1,223,223,7,226,677,224,102,2,223,223,1006,224,404,1001,223,1,223,1108,677,677,224,1002,223,2,223,1005,224,419,101,1,223,223,1008,226,677,224,102,2,223,223,1006,224,434,101,1,223,223,107,226,226,224,102,2,223,223,1005,224,449,1001,223,1,223,1007,677,677,224,1002,223,2,223,1006,224,464,1001,223,1,223,1007,226,226,224,1002,223,2,223,1005,224,479,101,1,223,223,1008,226,226,224,102,2,223,223,1006,224,494,1001,223,1,223,8,226,226,224,102,2,223,223,1006,224,509,101,1,223,223,1107,677,677,224,102,2,223,223,1005,224,524,1001,223,1,223,1108,226,677,224,1002,223,2,223,1006,224,539,101,1,223,223,1107,677,226,224,1002,223,2,223,1006,224,554,101,1,223,223,1007,677,226,224,1002,223,2,223,1005,224,569,101,1,223,223,7,677,226,224,1002,223,2,223,1006,224,584,101,1,223,223,107,677,677,224,1002,223,2,223,1005,224,599,1001,223,1,223,8,226,677,224,1002,223,2,223,1005,224,614,101,1,223,223,7,677,677,224,1002,223,2,223,1006,224,629,1001,223,1,223,1107,226,677,224,1002,223,2,223,1006,224,644,101,1,223,223,108,226,226,224,102,2,223,223,1005,224,659,1001,223,1,223,8,677,226,224,1002,223,2,223,1005,224,674,101,1,223,223,4,223,99,226];
    eprintln!("Memory =>\n   {:?}", memory);
    memory = init_computer(memory, 1);
    eprintln!("Memory =>\n   {:?}", memory);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_parameter_modes() {
        assert_eq!(extract_command(1002), (02, false, true, false));
    }

    #[test]
    fn simple_memory_evals() {
        assert_eq!(init_computer(vec![1,0,0,0,99], 1), vec![2,0,0,0,99]);
        assert_eq!(init_computer(vec![2,3,0,3,99], 1), vec![2,3,0,6,99]);
        assert_eq!(init_computer(vec![2,4,4,5,99,0], 1), vec![2,4,4,5,99,9801]);
        assert_eq!(init_computer(vec![1,1,1,4,99,5,6,0,99], 1), vec![30,1,1,4,2,5,6,0,99]);
    }
}
