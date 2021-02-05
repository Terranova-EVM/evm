#[macro_use]
mod macros;
mod arithmetic;
mod bitwise;
mod misc;

use core::ops::{BitAnd, BitOr, BitXor};
use primitive_types::U256;
use crate::{ExitReason, ExitSucceed, ExitError, Machine, Opcode};

pub enum Control {
	Continue(usize),
	Exit(ExitReason),
	Jump(usize),
	Trap(Opcode),
}

pub fn eval(state: &mut Machine, opcode: Opcode, position: usize) -> Control {
	match opcode {
		Opcode::STOP => Control::Exit(ExitSucceed::Stopped.into()),
		Opcode::ADD => op2_u256_tuple!(state, overflowing_add),
		Opcode::MUL => op2_u256_tuple!(state, overflowing_mul),
		Opcode::SUB => op2_u256_tuple!(state, overflowing_sub),
		Opcode::DIV => op2_u256_fn!(state, self::arithmetic::div),
		Opcode::SDIV => op2_u256_fn!(state, self::arithmetic::sdiv),
		Opcode::MOD => op2_u256_fn!(state, self::arithmetic::rem),
		Opcode::SMOD => op2_u256_fn!(state, self::arithmetic::srem),
		Opcode::ADDMOD => op3_u256_fn!(state, self::arithmetic::addmod),
		Opcode::MULMOD => op3_u256_fn!(state, self::arithmetic::mulmod),
		Opcode::EXP => op2_u256_fn!(state, self::arithmetic::exp),
		Opcode::SIGNEXTEND => op2_u256_fn!(state, self::arithmetic::signextend),
		Opcode::LT => op2_u256_bool_ref!(state, lt),
		Opcode::GT => op2_u256_bool_ref!(state, gt),
		Opcode::SLT => op2_u256_fn!(state, self::bitwise::slt),
		Opcode::SGT => op2_u256_fn!(state, self::bitwise::sgt),
		Opcode::EQ => op2_u256_bool_ref!(state, eq),
		Opcode::ISZERO => op1_u256_fn!(state, self::bitwise::iszero),
		Opcode::AND => op2_u256!(state, bitand),
		Opcode::OR => op2_u256!(state, bitor),
		Opcode::XOR => op2_u256!(state, bitxor),
		Opcode::NOT => op1_u256_fn!(state, self::bitwise::not),
		Opcode::BYTE => op2_u256_fn!(state, self::bitwise::byte),
		Opcode::SHL => op2_u256_fn!(state, self::bitwise::shl),
		Opcode::SHR => op2_u256_fn!(state, self::bitwise::shr),
		Opcode::SAR => op2_u256_fn!(state, self::bitwise::sar),
		Opcode::CODESIZE => self::misc::codesize(state),
		Opcode::CODECOPY => self::misc::codecopy(state),
		Opcode::CALLDATALOAD => self::misc::calldataload(state),
		Opcode::CALLDATASIZE => self::misc::calldatasize(state),
		Opcode::CALLDATACOPY => self::misc::calldatacopy(state),
		Opcode::POP => self::misc::pop(state),
		Opcode::MLOAD => self::misc::mload(state),
		Opcode::MSTORE => self::misc::mstore(state),
		Opcode::MSTORE8 => self::misc::mstore8(state),
		Opcode::JUMP => self::misc::jump(state),
		Opcode::JUMPI => self::misc::jumpi(state),
		Opcode::PC => self::misc::pc(state, position),
		Opcode::MSIZE => self::misc::msize(state),
		Opcode::JUMPDEST => Control::Continue(1),

		Opcode::PUSH1 => self::misc::push(state, 1, position),
		Opcode::PUSH2 => self::misc::push(state, 2, position),
		Opcode::PUSH3 => self::misc::push(state, 3, position),
		Opcode::PUSH4 => self::misc::push(state, 4, position),
		Opcode::PUSH5 => self::misc::push(state, 5, position),
		Opcode::PUSH6 => self::misc::push(state, 6, position),
		Opcode::PUSH7 => self::misc::push(state, 7, position),
		Opcode::PUSH8 => self::misc::push(state, 8, position),
		Opcode::PUSH9 => self::misc::push(state, 9, position),
		Opcode::PUSH10 => self::misc::push(state, 10, position),
		Opcode::PUSH11 => self::misc::push(state, 11, position),
		Opcode::PUSH12 => self::misc::push(state, 12, position),
		Opcode::PUSH13 => self::misc::push(state, 13, position),
		Opcode::PUSH14 => self::misc::push(state, 14, position),
		Opcode::PUSH15 => self::misc::push(state, 15, position),
		Opcode::PUSH16 => self::misc::push(state, 16, position),
		Opcode::PUSH17 => self::misc::push(state, 17, position),
		Opcode::PUSH18 => self::misc::push(state, 18, position),
		Opcode::PUSH19 => self::misc::push(state, 19, position),
		Opcode::PUSH20 => self::misc::push(state, 20, position),
		Opcode::PUSH21 => self::misc::push(state, 21, position),
		Opcode::PUSH22 => self::misc::push(state, 22, position),
		Opcode::PUSH23 => self::misc::push(state, 23, position),
		Opcode::PUSH24 => self::misc::push(state, 24, position),
		Opcode::PUSH25 => self::misc::push(state, 25, position),
		Opcode::PUSH26 => self::misc::push(state, 26, position),
		Opcode::PUSH27 => self::misc::push(state, 27, position),
		Opcode::PUSH28 => self::misc::push(state, 28, position),
		Opcode::PUSH29 => self::misc::push(state, 29, position),
		Opcode::PUSH30 => self::misc::push(state, 30, position),
		Opcode::PUSH31 => self::misc::push(state, 31, position),
		Opcode::PUSH32 => self::misc::push(state, 32, position),

		Opcode::DUP1 => self::misc::dup(state, 1),
		Opcode::DUP2 => self::misc::dup(state, 2),
		Opcode::DUP3 => self::misc::dup(state, 3),
		Opcode::DUP4 => self::misc::dup(state, 4),
		Opcode::DUP5 => self::misc::dup(state, 5),
		Opcode::DUP6 => self::misc::dup(state, 6),
		Opcode::DUP7 => self::misc::dup(state, 7),
		Opcode::DUP8 => self::misc::dup(state, 8),
		Opcode::DUP9 => self::misc::dup(state, 9),
		Opcode::DUP10 => self::misc::dup(state, 10),
		Opcode::DUP11 => self::misc::dup(state, 11),
		Opcode::DUP12 => self::misc::dup(state, 12),
		Opcode::DUP13 => self::misc::dup(state, 13),
		Opcode::DUP14 => self::misc::dup(state, 14),
		Opcode::DUP15 => self::misc::dup(state, 15),
		Opcode::DUP16 => self::misc::dup(state, 16),

		Opcode::SWAP1 => self::misc::swap(state, 1),
		Opcode::SWAP2 => self::misc::swap(state, 2),
		Opcode::SWAP3 => self::misc::swap(state, 3),
		Opcode::SWAP4 => self::misc::swap(state, 4),
		Opcode::SWAP5 => self::misc::swap(state, 5),
		Opcode::SWAP6 => self::misc::swap(state, 6),
		Opcode::SWAP7 => self::misc::swap(state, 7),
		Opcode::SWAP8 => self::misc::swap(state, 8),
		Opcode::SWAP9 => self::misc::swap(state, 9),
		Opcode::SWAP10 => self::misc::swap(state, 10),
		Opcode::SWAP11 => self::misc::swap(state, 11),
		Opcode::SWAP12 => self::misc::swap(state, 12),
		Opcode::SWAP13 => self::misc::swap(state, 13),
		Opcode::SWAP14 => self::misc::swap(state, 14),
		Opcode::SWAP15 => self::misc::swap(state, 15),
		Opcode::SWAP16 => self::misc::swap(state, 16),

		Opcode::RETURN => self::misc::ret(state),
		Opcode::REVERT => self::misc::revert(state),
		Opcode::INVALID => Control::Exit(ExitError::DesignatedInvalid.into()),

		_ => Control::Trap(opcode),
	}
}
