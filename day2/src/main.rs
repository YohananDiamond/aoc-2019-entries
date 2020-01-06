// https://adventofcode.com/2019/day/2
// Part 1 is okay.
// Part 2 is missing.

fn main() {
    let puzzle_input = vec![1,12,2,3,1,1,2,3,1,3,4,3,1,5,0,3,2,10,1,19,1,19,5,23,1,23,9,27,2,27,6,31,1,31,6,35,2,35,9,39,1,6,39,43,2,10,43,47,1,47,9,51,1,51,6,55,1,55,6,59,2,59,10,63,1,6,63,67,2,6,67,71,1,71,5,75,2,13,75,79,1,10,79,83,1,5,83,87,2,87,10,91,1,5,91,95,2,95,6,99,1,99,6,103,2,103,6,107,2,107,9,111,1,111,5,115,1,115,6,119,2,6,119,123,1,5,123,127,1,127,13,131,1,2,131,135,1,135,10,0,99,2,14,0,0];

    part_1(&puzzle_input);
    // part_2();
}

fn part_1(puzzle_input: &Vec<i32>) {
    let result_memory = compute(&puzzle_input);
    println!("Resulting memory:\n => {:?}", result_memory);
}

fn compute(input_memory: &Vec<i32>) -> Vec<i32> {
    let mut memory = input_memory.to_vec();
    let mut running = true;

    for slice_base_index in 0 .. (memory.len() as f64 / 4f64).ceil() as i32 {
        // Set up command info
        let command_position = slice_base_index * 4;
        let arg1 = (command_position + 1) as usize;
        let arg2 = (command_position + 2) as usize;
        let dest = (command_position + 3) as usize;
        let command_position = command_position as usize;

        // Parse and execute commands
        match memory[command_position] {
            1 => {
                // Case 1: sum
                // Only access the contents if needed (reduces chances of out-of-bound index error)
                let arg1_contents = memory[arg1] as usize;
                let arg2_contents = memory[arg2] as usize;
                let dest_contents = memory[dest] as usize;

                println!("Case 1 => (p{}: ${} = {}) + (p{}: ${} = {}) => (p{}: ${} was {})",
                    arg1, arg1_contents, memory[arg1_contents],
                    arg2, arg2_contents, memory[arg2_contents],
                    dest, dest_contents, memory[dest_contents],
                );
                memory[dest_contents] = memory[arg1_contents] + memory[arg2_contents];
            },

            2 => {
                // Case 2: multiply
                // Only access the contents if needed (reduces chances of out-of-bound index error)
                let arg1_contents = memory[arg1] as usize;
                let arg2_contents = memory[arg2] as usize;
                let dest_contents = memory[dest] as usize;

                println!("Case 2 => (p{}: ${} = {}) * (p{}: ${} = {}) => (p{}: ${} was {})",
                    arg1, arg1_contents, memory[arg1_contents],
                    arg2, arg2_contents, memory[arg2_contents],
                    dest, dest_contents, memory[dest_contents],
                );
                memory[dest_contents] = memory[arg1_contents] * memory[arg2_contents];
            },

            99 => {
                println!("Halt!");
                running = false;
            },
            _ => println!("Invalid command")
        }

        // Break if the program is halted
        if !running {
            break;
        }
    }

    memory
}

fn part_2() {
    // let mem = vec![2, 0, 0, 0];
    // let result_memory = compute(&mem);
    // println!("Resulting memory:\n => {:?}", result_memory);
    println!("Not implemented.");
}
