mod decoder;
mod displacement_mode;
mod effective_address_calculation;
mod op_code;
pub mod program;
mod register;
mod simulator;

use std::env;

fn main() -> Result<(), ()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        print_help();
        return Ok(());
    }

    let operation = &args[1];
    let operand = &args[2];

    match operation.as_str() {
        "decode" => {
            decoder::decode(operand)?;
        }
        "simulate" => {
            simulator::simulate::simulate(operand);
        }
        &_ => {
            print_help();
        }
    }

    Ok(())
}

fn print_help() {
    println!("Usage: perfaware_8086 OPERATION INPUT_FILE");
    println!("\nOperations:");
    println!("  decode");
    println!("  simulate");
    println!();
}
