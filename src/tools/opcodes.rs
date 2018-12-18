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
	CAL,
	CMP,
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
			0xB => Opcode::CMP,
			_=> Opcode::ERR
		}
	}
}

Opcode::HLT => {}
Opcode::NOP => {}
Opcode::LOD => {}
Opcode::ADD => {}
Opcode::SUB => {}
Opcode::MUL => {}
Opcode::DIV => {}
Opcode::MOD => {}
Opcode::SHR => {}
Opcode::SHL => {}
Opcode::CAL => {}
Opcode::CMP => {}
