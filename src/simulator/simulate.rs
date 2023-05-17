use crate::{
    decoder::decode,
    program::{self, Instruction, InstructionOperand, OperandType},
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
                crate::op_code::op::OpCode::Invalid => (),
                crate::op_code::op::OpCode::Mov => simulate_mov(&instruction, &mut state),
                crate::op_code::op::OpCode::Add => todo!(),
                crate::op_code::op::OpCode::Sub => todo!(),
                crate::op_code::op::OpCode::Cmp => todo!(),
                crate::op_code::op::OpCode::Jnz => todo!(),
                crate::op_code::op::OpCode::Je => todo!(),
                crate::op_code::op::OpCode::Jl => todo!(),
                crate::op_code::op::OpCode::Jle => todo!(),
                crate::op_code::op::OpCode::Jb => todo!(),
                crate::op_code::op::OpCode::Jbe => todo!(),
                crate::op_code::op::OpCode::Jp => todo!(),
                crate::op_code::op::OpCode::Jo => todo!(),
                crate::op_code::op::OpCode::Js => todo!(),
                crate::op_code::op::OpCode::Jnl => todo!(),
                crate::op_code::op::OpCode::Jg => todo!(),
                crate::op_code::op::OpCode::Jnb => todo!(),
                crate::op_code::op::OpCode::Ja => todo!(),
                crate::op_code::op::OpCode::Jnp => todo!(),
                crate::op_code::op::OpCode::Jno => todo!(),
                crate::op_code::op::OpCode::Jns => todo!(),
                crate::op_code::op::OpCode::Loop => todo!(),
                crate::op_code::op::OpCode::Loopz => todo!(),
                crate::op_code::op::OpCode::Loopnz => todo!(),
                crate::op_code::op::OpCode::Jcxz => todo!(),
            },
            None => todo!(),
        }
    }

    println!("Final state");
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
