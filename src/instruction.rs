#[derive(Debug, PartialEq)]
pub enum Opcode{
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
	AND,
	OR,
	NOT,
	XOR,
	CAL,
	CMP,
	RET,
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
			0xA => Opcode::AND,
			0xB => Opcode::OR,
			0xC => Opcode::NOT,
			0xD => Opcode::XOR,
			0xE => Opcode::CAL,
			0xF => Opcode::CMP,
			0x10 => Opcode::RET,
			_=> Opcode::ERR
		}
	}
}
