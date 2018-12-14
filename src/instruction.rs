extern crate byteorder;
extern crate bytes;

use self::byteorder::LittleEndian;
use self::bytes::{Buf, BufMut, Bytes, BytesMut};

#[derive(Debug, PartialEq)]
pub enum Opcode {
	HLT,
	NOP,
	LOD,
	ADD,
	SUB,
	MUL,
	DIV,
	MOD,
	SHR,
	SHL,
	CAL,
	ERR,
}

impl From<u8> for Opcode {
	fn from(v: u8) -> Self {
		match v {
			0x0 => Opcode::HLT,
			0x1 => Opcode::NOP,
			0x2 => Opcode::LOD,
			0x3 => Opcode::ADD,
			0x4 => Opcode::SUB,
			0x5 => Opcode::MUL,
			0x6 => Opcode::DIV,
			0x7 => Opcode::MOD,
			0x8 => Opcode::SHR,
			0x9 => Opcode::SHL,
			0xA => Opcode::CAL,
			_ => Opcode::ERR,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	// #[test]
	// fn test_inst_create() {
	// 	let program = Bytes::from(&b"Hello world"[..]);
	// 	// let ins = Instruction::from(program);
	// 	// assert_eq!(Bytes::from(&b"el"[..]), ins.data);
	// }
}
