use crate::{
    decoder::decode,
    op_code::op::OpCode,
    program::{Instruction, OperandType},
    simulator::simulator_state::SimulatorState,
};

pub fn simulate(file_name: &str) {
    println!("Simulator started with {}", file_name);

    let mut program = match decode(file_name) {
        Ok(program) => program,
        Err(_) => {
            println!("Error: decoder failed, can't simulate program.");
            return;
        }
    };

    let mut state = SimulatorState::new();
    println!("Initial state");
    state.registers.print();

    while program.has_pending_instructions() {
        match program.next_instruction() {
            Some(instruction) => match instruction.op_code {
                OpCode::Invalid => {
                    println!("Error: Can't simulate intruction: invalid op code.")
                }
                OpCode::Mov => simulate_mov(&instruction, &mut state),
                OpCode::Add => todo!(),
                OpCode::Sub => todo!(),
                OpCode::Cmp => todo!(),
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
            },
            None => todo!(),
        }
    }

    println!("\nFinal state");
    state.registers.print();
}

fn simulate_mov(instruction: &Instruction, state: &mut SimulatorState) {
    let src_operand = &instruction.src_operand.as_ref().unwrap();

    let data = match src_operand.operand_type {
        OperandType::REGISTER => todo!(),
        OperandType::EAC => todo!(),
        OperandType::LITERAL => src_operand.literal,
    }
    .unwrap();

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
