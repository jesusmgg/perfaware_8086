use crate::{
    decoder::decode,
    op_code::op::OpCode,
    program::instruction::{Instruction, OperandType},
    simulator::simulator_state::SimulatorState,
};

pub fn simulate(file_name: &str) {
    println!("Simulator started with {}", file_name);

    let mut program = match decode(file_name, false) {
        Ok(program) => program,
        Err(_) => {
            println!("Error: decoder failed, can't simulate program.");
            return;
        }
    };

    let mut state = SimulatorState::new();
    println!("Starting simulation...");
    println!();

    loop {
        let instruction = program.get_instruction_at_byte(state.read_ip() as usize);
        match instruction {
            Some(instruction) => match instruction.op_code {
                OpCode::Mov => {
                    simulate_mov(&instruction, &mut state);
                }
                OpCode::Add | OpCode::Sub | OpCode::Cmp => {
                    simulate_add_mov_cmp(&instruction, &mut state);
                }
                OpCode::Jnz => todo!(),
                OpCode::Je => todo!(),
                OpCode::Jl => todo!(),
                OpCode::Jle => todo!(),
                OpCode::Jb => todo!(),
                OpCode::Jbe => todo!(),
                OpCode::Jp => todo!(),
                OpCode::Jo => todo!(),
                OpCode::Js => todo!(),
                OpCode::Jnl => todo!(),
                OpCode::Jg => todo!(),
                OpCode::Jnb => todo!(),
                OpCode::Ja => todo!(),
                OpCode::Jnp => todo!(),
                OpCode::Jno => todo!(),
                OpCode::Jns => todo!(),
                OpCode::Loop => todo!(),
                OpCode::Loopz => todo!(),
                OpCode::Loopnz => todo!(),
                OpCode::Jcxz => todo!(),

                OpCode::Invalid => {
                    println!("Error: Can't simulate instruction: invalid op code");
                }
                OpCode::InvalidAddress => {
                    println!("Error: Can't simulate instruction: invalid address");
                }
                OpCode::EndOfProgram => {
                    println!("\nReached end of program");
                    break;
                }
            },
            None => todo!(),
        };
    }

    println!("\nFinal state");
    state.registers.print(true);
    println!();
    state.print_ip();
    println!();
    state.flags_register.print();
    println!();
}

fn print_instruction_info(instruction: &Instruction) {
    println!("{}", instruction.decoded_string.as_ref().unwrap());
}

fn simulate_mov(instruction: &Instruction, state: &mut SimulatorState) {
    print_instruction_info(&instruction);
    state.write_ip(state.read_ip() + instruction.length as u16);

    let src_operand = &instruction.src_operand.as_ref().unwrap();

    let data = match src_operand.operand_type {
        OperandType::REGISTER => state.registers.read(
            src_operand.register.unwrap(),
            src_operand.register_word.unwrap(),
        ),
        OperandType::EAC => todo!(),
        OperandType::LITERAL => src_operand.literal.unwrap(),
    };

    let dest_operand = instruction.dest_operand.as_ref().unwrap();
    match dest_operand.operand_type {
        OperandType::REGISTER => state.registers.write(
            data,
            dest_operand.register.unwrap(),
            dest_operand.register_word.unwrap(),
        ),
        OperandType::EAC => todo!(),
        OperandType::LITERAL => todo!(),
    }
}

fn simulate_add_mov_cmp(instruction: &Instruction, state: &mut SimulatorState) {
    print_instruction_info(&instruction);
    state.write_ip(state.read_ip() + instruction.length as u16);

    let src_operand = &instruction.src_operand.as_ref().unwrap();
    let data_src = match src_operand.operand_type {
        OperandType::REGISTER => state.registers.read(
            src_operand.register.unwrap(),
            src_operand.register_word.unwrap(),
        ),
        OperandType::EAC => todo!(),
        OperandType::LITERAL => src_operand.literal.unwrap(),
    };

    let dest_operand = instruction.dest_operand.as_ref().unwrap();
    let data_dest = match dest_operand.operand_type {
        OperandType::REGISTER => state.registers.read(
            dest_operand.register.unwrap(),
            dest_operand.register_word.unwrap(),
        ),
        OperandType::EAC => todo!(),
        OperandType::LITERAL => dest_operand.literal.unwrap(),
    };

    match dest_operand.operand_type {
        OperandType::REGISTER => {
            let result: Option<u16> = match instruction.op_code {
                OpCode::Add => {
                    let r = (data_dest as i16 + data_src as i16) as u16;
                    state.flags_register.zero = r == 0;
                    state.flags_register.sign = (r & 0x8000) >> 15 == 1;
                    Some(r)
                }
                OpCode::Sub => {
                    let r = (data_dest as i16 - data_src as i16) as u16;
                    state.flags_register.zero = r == 0;
                    state.flags_register.sign = (r & 0x8000) >> 15 == 1;
                    Some(r)
                }
                OpCode::Cmp => {
                    let r = (data_dest as i16 - data_src as i16) as u16;
                    state.flags_register.zero = r == 0;
                    state.flags_register.sign = (r & 0x8000) >> 15 == 1;
                    None
                }
                _ => todo!(), // Invalid
            };
            if result.is_some() {
                let r = result.unwrap();
                state.registers.write(
                    r,
                    dest_operand.register.unwrap(),
                    dest_operand.register_word.unwrap(),
                );
            };
            state.flags_register.print();
        }
        _ => todo!(), // Invalid
    }
}
