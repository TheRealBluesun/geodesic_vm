extern crate bytes;
extern crate byteorder;

use self::bytes::{Buf, BufMut, Bytes, BytesMut};
use self::byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use instruction::Opcode;
use std::mem::size_of;

const REGSIZE: usize = 0xFF / 4;

pub struct VMScript {
    pc: usize,  // Program Counter -- will be used as an index, could be u8 otherwise
    rem: u128,  // Remainder for DIV
    f_eq: bool, // is_equal flag
    f_lt: bool, // lessthan flag
    f_gt: bool, // greaterthan flag
    regs32: [i32; REGSIZE],
    regs64: [i64; REGSIZE],
    regs128: [i128; REGSIZE],
    script: Bytes,
}
#[derive(Debug)]
enum RegLocal {
    REG32,
    REG64,
    REG128,
}

impl From<u8> for RegLocal {
    fn from(v: u8) -> Self {
        let masked = v >> 6; // Top two bits decide which reg
        println!("Val, Masked: {:?}, {:?}", v, masked);
        match masked {
            0 => RegLocal::REG32,
            1 => RegLocal::REG64,
            2 => RegLocal::REG128,
            _ => panic!("Invalid register specified! {:?}", v),
        }
    }
}

impl VMScript {
    pub fn new(bytes: Bytes) -> VMScript {
        VMScript {
            pc: 0,
            rem: 0,
            f_eq: false,
            f_lt: false,
            f_gt: false,
            regs32: [0; REGSIZE],
            regs64: [0; REGSIZE],
            regs128: [0; REGSIZE],
            script: bytes,
        }
    }

    pub fn reset(&mut self) {
        self.pc = 0;
        self.rem = 0;
        self.f_eq = false;
        self.f_lt = false;
        self.f_gt = false;
        self.regs32 = [0; REGSIZE];
        self.regs64 = [0; REGSIZE];
        self.regs128 = [0; REGSIZE];
    }

    pub fn run(&mut self) -> bool {
        let mut finished = false;
        while !finished {
            finished = !self.step();
        }
        return true;
    }

    // Expected return value is "shuld we keep running"
    fn step(&mut self) -> bool {
        // Get opcode from script
        let o = Opcode::from(self.next_bytes(1)[0]);
        println!("Opcode found: {:?}", o);
        match o {
            Opcode::HLT => {
                false;
            }
            Opcode::NOP => {
                true;
            }
            Opcode::LOD => {
                let reg = self.next_bytes(1)[0];
                let r = RegLocal::from(reg);
                println!("LOD Reg: {:?}", r);
                match r {
                    RegLocal::REG32 => {
                        let idx = (reg & 0x3F) as usize;
                        let sz = size_of::<u32>();
                        let b = self.next_bytes(sz);
                        let mut val = b[sz-1] as u32;
                        for i in 0..sz-1{
                            val += (b[i] as u32) << ((sz-1-i) * 8);
                        }
                        self.regs32[idx] = val as i32;
                    }
                    RegLocal::REG64 => {
                        let idx = (reg & 0x3F) as usize;
                        let sz = size_of::<u64>();
                        let b = self.next_bytes(sz);
                        let mut val = b[sz-1] as u64;
                        for i in 0..sz-1{
                            val += (b[i] as u64) << ((sz-1-i) * 8);
                        }
                        self.regs64[idx] = val as i64;
                    }
                    RegLocal::REG128 => {
                        let idx = (reg & 0x3F) as usize;
                        let sz = size_of::<u128>();
                        let b = self.next_bytes(sz);
                        let mut val = b[sz-1] as u128;
                        for i in 0..sz-1{
                            val += (b[i] as u128) << ((sz-1-i) * 8);
                        }
                        self.regs128[idx] = val as i128;
                    }
                }
            }
            Opcode::ADD => {}
            Opcode::SUB => {}
            Opcode::MUL => {}
            Opcode::DIV => {}
            Opcode::MOD => {}
            Opcode::SHR => {}
            Opcode::SHL => {}
            Opcode::CAL => {}
            _ => {
                println!("Unknown opcode! {:?}", o);
                false;
            }
        }
        false
    }

    fn next_bytes(&mut self, numbytes: usize) -> Bytes {
        self.pc += numbytes;
        if self.pc > self.script.len() {
            panic!("Program counter overrun!");
        }
        return self.script.slice(self.pc - numbytes, self.pc);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lod32() {
        let script = Bytes::from_static(&[2, 0 + 1, 0xFF, 0xFF, 0xFF, 0xFF]);
        let mut test_vm = VMScript::new(script);
        test_vm.run();
        assert_eq!(test_vm.regs32[1], -1);
    }

    #[test]
    fn test_lod64() {
        let script = Bytes::from_static(&[2, (1 << 6) + 1, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
        let mut test_vm = VMScript::new(script);
        test_vm.run();
        assert_eq!(test_vm.regs64[1], -1);
    }

        #[test]
    fn test_lod128() {
        let script = Bytes::from_static(&[2, (1 << 7) + 1, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
        let mut test_vm = VMScript::new(script);
        test_vm.run();
        assert_eq!(test_vm.regs128[1], -1);
    }

}
