mod decoder;
mod displacement_mode;
mod effective_address_calculation;
mod op_code;
mod program;
mod register;
mod simulator;

use std::env;

fn main() -> Result<(), ()> {
    let args: Vec<String> = env::args().collect();
    let args_len = args.len();

    if args_len < 3 {
        print_help();
        return Ok(());
    }

    // Parse operand
    let operand = &args[args_len - 1];

    // Parse options
    let mut option_dump: bool = false;
    let mut option_time: bool = false;
    if args_len > 3 {
        for i in 1..(args_len - 2) {
            match args[i].as_str() {
                "dump" => option_dump = true,
                "time" => option_time = true,
                invalid_option_str => println!("Skipping invalid option: {invalid_option_str}"),
            }
        }
    }

    // Parse operation
    let operation = &args[args_len - 2];
    match operation.as_str() {
        "decode" => {
            decoder::decode(operand, true, option_time)?;
        }
        "simulate" => {
            simulator::simulate::simulate(operand, option_dump, option_time);
        }
        &_ => {
            print_help();
        }
    }

    Ok(())
}

fn print_help() {
    println!("Usage: perfaware_8086 OPTIONS OPERATION INPUT_FILE");
    println!("\nOptions:");
    println!("  dump:       if simulating, dumps memory into file \"memory.data\". ");
    println!("  time:       if simulating, estimates the cycles the program execution will take.");
    println!("\nOperations:");
    println!("  decode:     decodes the program and outputs the instruction.");
    println!("  simulate:   decodes and then simulates the program execution.");
    println!();
}
