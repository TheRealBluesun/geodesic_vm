pub enum Opcode{
	HLT,
	NOP,
	LOD,
	INC,
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
	PSH,
	POP,
	ERR,
}

impl From<u8> for Opcode {
	fn from(v: u8) -> Self {
		match v {
			0x0 => Opcode::HLT,
			0x1 => Opcode::NOP,
			0x2 => Opcode::LOD,
			0x3 => Opcode::INC,
			0x4 => Opcode::ADD,
			0x5 => Opcode::SUB,
			0x6 => Opcode::MUL,
			0x7 => Opcode::DIV,
			0x8 => Opcode::MOD,
			0x9 => Opcode::SHR,
			0xA => Opcode::SHL,
			0xB => Opcode::AND,
			0xC => Opcode::OR,
			0xD => Opcode::NOT,
			0xE => Opcode::XOR,
			0xF => Opcode::CAL,
			0x10 => Opcode::CMP,
			0x11 => Opcode::PSH,
			0x12 => Opcode::POP,
			_=> Opcode::ERR
		}
	}
}

Opcode::HLT => {}
Opcode::NOP => {}
Opcode::LOD => {}
Opcode::INC => {}
Opcode::ADD => {}
Opcode::SUB => {}
Opcode::MUL => {}
Opcode::DIV => {}
Opcode::MOD => {}
Opcode::SHR => {}
Opcode::SHL => {}
Opcode::AND => {}
Opcode::OR => {}
Opcode::NOT => {}
Opcode::XOR => {}
Opcode::CAL => {}
Opcode::CMP => {}
Opcode::PSH => {}
Opcode::POP => {}
