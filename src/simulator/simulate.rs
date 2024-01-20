use crate::{
    decoder::decode,
    op_code::op::OpCode,
    program::instruction::{Instruction, OperandType},
    simulator::simulator_state::SimulatorState,
};

pub fn simulate(file_name: &str, dump_memory: bool, estimate_cycles: bool) {
    println!("Simulator started with {}", file_name);

    let mut program = match decode(file_name, false, estimate_cycles) {
        Ok(program) => program,
        Err(_) => {
            println!("Error: decoder failed, can't simulate program");
            return;
        }
    };

    let mut state = SimulatorState::new();
    println!("Starting simulation...");
    println!();

    loop {
        let instruction = program.get_instruction_at_byte(state.read_ip() as usize);
        match instruction {
            Some(instruction) => {
                if estimate_cycles {
                    state.cycles += match &instruction.time_estimation {
                        Some(time_estimation) => time_estimation.total_time(),
                        None if instruction.op_code == OpCode::EndOfProgram => 0,
                        _ => panic!("Error: invalid instruction for time estimation"),
                    }
                }
                match instruction.op_code {
                    OpCode::Mov => {
                        simulate_mov(&instruction, &mut state, estimate_cycles);
                    }
                    OpCode::Add | OpCode::Sub | OpCode::Cmp => {
                        simulate_add_sub_cmp(&instruction, &mut state, estimate_cycles);
                    }
                    OpCode::Jnz => {
                        simulate_conditional_jmp(&instruction, &mut state, estimate_cycles);
                    }
                    OpCode::Je => {
                        simulate_conditional_jmp(&instruction, &mut state, estimate_cycles);
                    }
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
                        println!("Error: can't simulate instruction: invalid op code");
                        break;
                    }
                    OpCode::InvalidAddress => {
                        println!("Error: can't simulate instruction: invalid address");
                        break;
                    }
                    OpCode::EndOfProgram => {
                        println!("\nReached end of program");
                        break;
                    }
                }
            }
            None => {
                println!("Error: no instruction decoded to simulate");
                break;
            }
        };
    }

    println!("\nFinal state");
    state.registers.print(true);
    println!();
    state.print_ip();
    println!();
    state.flags_register.print();
    println!();

    if dump_memory {
        state.dump_memory().unwrap();
    }
}

fn print_instruction_info(instruction: &Instruction, state: &SimulatorState, print_cycles: bool) {
    let cycles_string = if print_cycles {
        let time_estimation = instruction.time_estimation.unwrap();
        format!(
            " ; Cycles: +{} = {}",
            time_estimation.get_string(),
            state.cycles
        )
    } else {
        String::from("")
    };

    println!(
        "{}{}",
        instruction.decoded_string.as_ref().unwrap(),
        cycles_string
    );
}

fn simulate_mov(instruction: &Instruction, state: &mut SimulatorState, print_cycles: bool) {
    print_instruction_info(&instruction, &state, print_cycles);
    state.write_ip(state.read_ip() + instruction.length as u16);

    let src_operand = &instruction.src_operand.as_ref().unwrap();

    let data = match src_operand.operand_type {
        OperandType::REGISTER => state.registers.read(
            src_operand.register.unwrap(),
            src_operand.register_word.unwrap(),
        ),
        OperandType::EAC => {
            let mut address: u16 = 0;
            address += match src_operand.eac_reg_0 {
                Some(reg) => state.registers.read(reg, true),
                None => 0,
            };
            address += match src_operand.eac_reg_1 {
                Some(reg) => state.registers.read(reg, true),
                None => 0,
            };
            address += match src_operand.eac_displacement {
                Some(displacement) => displacement,
                None => 0,
            };

            let mut d = state.read_mem_byte(address as usize) as u16;
            if src_operand.register_word.unwrap() {
                d += state.read_mem_byte(address as usize + 1) as u16 * 256;
            }

            d
        }
        OperandType::LITERAL => src_operand.literal.unwrap(),
    };

    let dest_operand = instruction.dest_operand.as_ref().unwrap();
    match dest_operand.operand_type {
        OperandType::REGISTER => state.registers.write(
            data,
            dest_operand.register.unwrap(),
            dest_operand.register_word.unwrap(),
        ),
        OperandType::EAC => {
            let mut address: u16 = 0;
            address += match dest_operand.eac_reg_0 {
                Some(reg) => state.registers.read(reg, true),
                None => 0,
            };
            address += match dest_operand.eac_reg_1 {
                Some(reg) => state.registers.read(reg, true),
                None => 0,
            };
            address += match dest_operand.eac_displacement {
                Some(displacement) => displacement,
                None => 0,
            };

            if src_operand.register_word.unwrap() {
                let bytes = data.to_le_bytes();
                state.write_mem_byte(address as usize, bytes[0]);
                state.write_mem_byte(address as usize + 1, bytes[1]);
            } else {
                state.write_mem_byte(address as usize, data as u8)
            }
        }
        OperandType::LITERAL => todo!(),
    }
}

fn simulate_add_sub_cmp(instruction: &Instruction, state: &mut SimulatorState, print_cycles: bool) {
    print_instruction_info(&instruction, &state, print_cycles);
    state.write_ip(state.read_ip() + instruction.length as u16);

    let src_operand = &instruction.src_operand.as_ref().unwrap();
    let data_src = match src_operand.operand_type {
        OperandType::REGISTER => state.registers.read(
            src_operand.register.unwrap(),
            src_operand.register_word.unwrap(),
        ),
        OperandType::LITERAL => src_operand.literal.unwrap(),
        OperandType::EAC => {
            let mut address: u16 = 0;
            address += match src_operand.eac_reg_0 {
                Some(reg) => state.registers.read(reg, true),
                None => 0,
            };
            address += match src_operand.eac_reg_1 {
                Some(reg) => state.registers.read(reg, true),
                None => 0,
            };
            address += match src_operand.eac_displacement {
                Some(displacement) => displacement,
                None => 0,
            };

            let mut d = state.read_mem_byte(address as usize) as u16;
            if src_operand.register_word.unwrap() {
                d += state.read_mem_byte(address as usize + 1) as u16 * 256;
            }

            d
        }
    };

    let dest_operand = instruction.dest_operand.as_ref().unwrap();
    let data_dest = match dest_operand.operand_type {
        OperandType::REGISTER => state.registers.read(
            dest_operand.register.unwrap(),
            dest_operand.register_word.unwrap(),
        ),
        OperandType::EAC => {
            let mut addr = dest_operand.eac_displacement.unwrap_or_default();
            addr += match dest_operand.eac_reg_0 {
                Some(reg) => state.registers.read(reg, true),
                None => 0,
            };
            addr += match dest_operand.eac_reg_1 {
                Some(reg) => state.registers.read(reg, true),
                None => 0,
            };
            state.read_mem_word(addr as usize)
        }
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
                _ => panic!("Error: invalid opcode for ADD/SUB/CMP instruction"),
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
        OperandType::EAC => {
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
                _ => panic!("Error: invalid opcode for ADD/SUB/CMP instruction"),
            };
            if result.is_some() {
                // Get memory address
                let mut addr = dest_operand.eac_displacement.unwrap_or_default();
                addr += match dest_operand.eac_reg_0 {
                    Some(reg) => state.registers.read(reg, true),
                    None => 0,
                };
                addr += match dest_operand.eac_reg_1 {
                    Some(reg) => state.registers.read(reg, true),
                    None => 0,
                };

                // Write data to memory
                let r = result.unwrap();
                state.write_mem_word(addr as usize, r);
            };
            state.flags_register.print();
        }
        OperandType::LITERAL => panic!("Error: ADD/SUB/CMP to literal is not a valid operation"),
    }
}

fn simulate_conditional_jmp(
    instruction: &Instruction,
    state: &mut SimulatorState,
    print_cycles: bool,
) {
    print_instruction_info(&instruction, &state, print_cycles);
    state.write_ip(state.read_ip() + instruction.length as u16);

    match (instruction.op_code, state.flags_register.zero) {
        (OpCode::Jnz, false) | (OpCode::Je, true) => {
            let dest_operand = instruction.dest_operand.as_ref().unwrap();
            let data = match dest_operand.operand_type {
                OperandType::LITERAL => dest_operand.literal.unwrap(),
                _ => {
                    println!("Error: invalid increment data type in conditional jump");
                    0
                }
            };
            let increment = data as i16;
            let mut new_ip = state.read_ip() as i16 + increment;
            if increment < 0 {
                new_ip -= instruction.length as i16;
            }
            state.write_ip(new_ip as u16);
        }
        _ => {}
    };
}
