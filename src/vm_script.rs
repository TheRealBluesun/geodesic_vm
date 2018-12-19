extern crate byteorder;
extern crate bytes;

use self::byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use self::bytes::{Buf, BufMut, Bytes, BytesMut};
use instruction::Opcode;
use std::mem::size_of;
use std::ptr;
use vm::VM;

const REGSIZE: usize = 0xFF / 4;

pub struct VMScript<'a> {
    //vm_handle: &'a VM<'a>,
    pc: usize,  // Program Counter -- will be used as an index, could be u8 otherwise
    f_eq: bool, // is_equal flag
    f_lt: bool, // lessthan flag
    f_gt: bool, // greaterthan flag
    pub regs32: [i32; REGSIZE],
    regs64: [i64; REGSIZE],
    regs128: [i128; REGSIZE],
    rem32: u32, // Remainder for DIV
    rem64: u64,
    rem128: u128,
    script: &'a Bytes,
    libs: Option<&'a [Bytes]>,
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
        match masked {
            0 => RegLocal::REG32,
            1 => RegLocal::REG64,
            2 => RegLocal::REG128,
            _ => panic!("Invalid register specified! {:?}", v),
        }
    }
}

impl<'a> VMScript<'a> {
    //    pub fn new(bytes: &'a Bytes, vm: &'a mut VM<'a>) -> VMScript<'a> {
    pub fn new(bytes: &'a Bytes, libs: Option<&'a [Bytes]>) -> VMScript<'a> {
        VMScript {
            pc: 0,
            rem32: 0,
            rem64: 0,
            rem128: 0,
            f_eq: false,
            f_lt: false,
            f_gt: false,
            regs32: [0; REGSIZE],
            regs64: [0; REGSIZE],
            regs128: [0; REGSIZE],
            script: bytes,
            libs: libs,
        }
    }

    pub fn reset(&mut self) {
        self.pc = 0;
        self.rem32 = 0;
        self.rem64 = 0;
        self.rem128 = 0;
        self.f_eq = false;
        self.f_lt = false;
        self.f_gt = false;
        self.regs32 = [0; REGSIZE];
        self.regs64 = [0; REGSIZE];
        self.regs128 = [0; REGSIZE];
    }

    pub fn run(&mut self) -> i32 {
        let mut finished = false;
        while !finished {
            finished = !self.step();
        }
        self.regs32[0]
    }

    // Expected return value is "shuld we keep running"
    fn step(&mut self) -> bool {
        // Get opcode from script
        let o = Opcode::from(self.next_bytes(1)[0]);
        println!("Opcode found: {:?}", o);
        match o {
            Opcode::HLT => {
                return false;
            }
            Opcode::NOP => {}
            Opcode::LOD => {
                let reg = self.next_bytes(1)[0];
                let r = RegLocal::from(reg);
                let idx = (reg & 0x3F) as usize;
                println!("LOD Reg: {:?}", r);
                match r {
                    RegLocal::REG32 => {
                        let val = self.read_u32();
                        self.regs32[idx] = val as i32;
                    }
                    RegLocal::REG64 => {
                        let val = self.read_u64();
                        self.regs64[idx] = val as i64;
                    }
                    RegLocal::REG128 => {
                        let val = self.read_u128();
                        self.regs128[idx] = val as i128;
                    }
                }
            }
            Opcode::ADD => {
                let reg1 = self.next_bytes(1)[0];
                let reg2 = self.next_bytes(1)[0];
                let idx1 = (reg1 & 0x3F) as usize;
                let idx2 = (reg2 & 0x3F) as usize;
                match RegLocal::from(reg1) {
                    RegLocal::REG32 => {
                        self.regs32[idx1] += self.regs32[idx2];
                    }
                    RegLocal::REG64 => {
                        self.regs64[idx1] += self.regs64[idx2];
                    }
                    RegLocal::REG128 => {
                        self.regs128[idx1] += self.regs128[idx2];
                    }
                }
            }
            Opcode::SUB => {
                let reg1 = self.next_bytes(1)[0];
                let reg2 = self.next_bytes(1)[0];
                let idx1 = (reg1 & 0x3F) as usize;
                let idx2 = (reg2 & 0x3F) as usize;
                match RegLocal::from(reg1) {
                    RegLocal::REG32 => {
                        self.regs32[idx1] -= self.regs32[idx2];
                    }
                    RegLocal::REG64 => {
                        self.regs64[idx1] -= self.regs64[idx2];
                    }
                    RegLocal::REG128 => {
                        self.regs128[idx1] -= self.regs128[idx2];
                    }
                }
            }
            Opcode::MUL => {
                let reg1 = self.next_bytes(1)[0];
                let reg2 = self.next_bytes(1)[0];
                let idx1 = (reg1 & 0x3F) as usize;
                let idx2 = (reg2 & 0x3F) as usize;
                match RegLocal::from(reg1) {
                    RegLocal::REG32 => {
                        self.regs32[idx1] *= self.regs32[idx2];
                    }
                    RegLocal::REG64 => {
                        self.regs64[idx1] *= self.regs64[idx2];
                    }
                    RegLocal::REG128 => {
                        self.regs128[idx1] *= self.regs128[idx2];
                    }
                }
            }
            Opcode::DIV => {
                let reg1 = self.next_bytes(1)[0];
                let reg2 = self.next_bytes(1)[0];
                let idx1 = (reg1 & 0x3F) as usize;
                let idx2 = (reg2 & 0x3F) as usize;
                match RegLocal::from(reg1) {
                    RegLocal::REG32 => {
                        self.rem32 = (self.regs32[idx1] % self.regs32[idx2]) as u32;
                        self.regs32[idx1] /= self.regs32[idx2];
                    }
                    RegLocal::REG64 => {
                        self.rem64 = (self.regs64[idx1] % self.regs64[idx2]) as u64;
                        self.regs64[idx1] /= self.regs64[idx2];
                    }
                    RegLocal::REG128 => {
                        self.rem128 = (self.regs128[idx1] % self.regs128[idx2]) as u128;
                        self.regs128[idx1] /= self.regs128[idx2];
                    }
                }
            }
            Opcode::MOD => {
                let reg1 = self.next_bytes(1)[0];
                let reg2 = self.next_bytes(1)[0];
                let idx1 = (reg1 & 0x3F) as usize;
                let idx2 = (reg2 & 0x3F) as usize;
                match RegLocal::from(reg1) {
                    RegLocal::REG32 => {
                        self.regs32[idx1] %= self.regs32[idx2];
                    }
                    RegLocal::REG64 => {
                        self.regs64[idx1] %= self.regs64[idx2];
                    }
                    RegLocal::REG128 => {
                        self.regs128[idx1] %= self.regs128[idx2];
                    }
                }
            }
            Opcode::SHR => {
                let reg = self.next_bytes(1)[0];
                let idx = (reg & 0x3F) as usize;
                let shft = self.next_bytes(1)[0];
                match RegLocal::from(reg) {
                    RegLocal::REG32 => {
                        self.regs32[idx] >>= shft;
                    }
                    RegLocal::REG64 => {
                        self.regs64[idx] >>= shft;
                    }
                    RegLocal::REG128 => {
                        self.regs128[idx] >>= shft;
                    }
                }
            }
            Opcode::SHL => {
                let reg = self.next_bytes(1)[0];
                let idx = (reg & 0x3F) as usize;
                let shft = self.next_bytes(1)[0];
                match RegLocal::from(reg) {
                    RegLocal::REG32 => {
                        self.regs32[idx] <<= shft;
                    }
                    RegLocal::REG64 => {
                        self.regs64[idx] <<= shft;
                    }
                    RegLocal::REG128 => {
                        self.regs128[idx] <<= shft;
                    }
                }
            }
            Opcode::CMP => {
                let reg1 = self.next_bytes(1)[0];
                let reg2 = self.next_bytes(1)[0];
                let idx1 = (reg1 & 0x3F) as usize;
                let idx2 = (reg2 & 0x3F) as usize;
                match RegLocal::from(reg1) {
                    RegLocal::REG32 => {
                        // Intentionally not done with three statements
                        // to minimize effective operations
                        if self.regs32[idx1] == self.regs32[idx2] {
                            self.f_eq = true;
                            self.f_gt = false;
                            self.f_lt = false;
                        } else if self.regs32[idx1] > self.regs32[idx2] {
                            self.f_eq = false;
                            self.f_gt = true;
                            self.f_lt = false;
                        } else if self.regs32[idx1] < self.regs32[idx2] {
                            self.f_eq = false;
                            self.f_gt = false;
                            self.f_lt = true;
                        }
                    }
                    RegLocal::REG64 => {
                        if self.regs64[idx1] == self.regs64[idx2] {
                            self.f_eq = true;
                            self.f_gt = false;
                            self.f_lt = false;
                        } else if self.regs64[idx1] > self.regs64[idx2] {
                            self.f_eq = false;
                            self.f_gt = true;
                            self.f_lt = false;
                        } else if self.regs64[idx1] < self.regs64[idx2] {
                            self.f_eq = false;
                            self.f_gt = false;
                            self.f_lt = true;
                        }
                    }
                    RegLocal::REG128 => {
                        if self.regs128[idx1] == self.regs128[idx2] {
                            self.f_eq = true;
                            self.f_gt = false;
                            self.f_lt = false;
                        } else if self.regs128[idx1] > self.regs128[idx2] {
                            self.f_eq = false;
                            self.f_gt = true;
                            self.f_lt = false;
                        } else if self.regs128[idx1] < self.regs128[idx2] {
                            self.f_eq = false;
                            self.f_gt = false;
                            self.f_lt = true;
                        }
                    }
                }
            }
            Opcode::CAL => {
                // We're going to pass off execution to another script
                // Get the index as the next param

                // Call that script if it exists
                match self.libs {
                    Some(libs) => {
                        let idx = (self.next_bytes(1)[0] + 1)as usize;
                        let mut cal_script = VMScript::new(&libs[idx], Some(libs));
                        self.regs32[0] = cal_script.run();
                    }
                    None => {
                        panic!("Attempted to call empty libs!");
                    }
                }
            }
            _ => {
                println!("Unknown opcode! {:?}", o);
                return false;
            }
        }
        true
    }

    fn next_bytes(&mut self, numbytes: usize) -> Bytes {
        self.pc += numbytes;
        if self.pc > self.script.len() {
            panic!("Program counter overrun! Are you missing a 'HLT'?");
        }
        self.script.slice(self.pc - numbytes, self.pc)
    }

    fn read_u32(&mut self) -> u32 {
        let sz = size_of::<u32>();
        let b = self.next_bytes(sz);
        let mut val = u32::from(b[sz - 1]);
        for (i, v) in b.iter().enumerate().take(sz - 1) {
            val += (u32::from(*v)) << ((sz - 1 - i) * 8);
        }
        val
    }

    fn read_u64(&mut self) -> u64 {
        let sz = size_of::<u64>();
        let b = self.next_bytes(sz);
        let mut val = u64::from(b[sz - 1]);
        for (i, v) in b.iter().enumerate().take(sz - 1) {
            val += (u64::from(*v)) << ((sz - 1 - i) * 8);
        }
        val
    }

    fn read_u128(&mut self) -> u128 {
        let sz = size_of::<u128>();
        let b = self.next_bytes(sz);
        let mut val = u128::from(b[sz - 1]);
        for (i, v) in b.iter().enumerate().take(sz - 1) {
            val += (u128::from(*v)) << ((sz - 1 - i) * 8);
        }
        val
    }
}
// /*
#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    #![allow(unused_parens)]
    #![allow(overflowing_literals)]
    use super::*;

    #[test]
    fn test_lod32() {
        let reg = 0 + 1;
        let script = Bytes::from(&[Opcode::LOD as u8, reg, 0xFF, 0xFF, 0xFF, 0xFF, 0][..]);
        let mut test_vm = VMScript::new(&script, None);
        test_vm.run();
        assert_eq!(test_vm.regs32[1], -1);
    }

    #[test]
    fn test_shr32() {
        let reg = 0 + 1;
        let script = Bytes::from(
            &[
                Opcode::LOD as u8,
                reg,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                Opcode::SHR as u8,
                reg,
                3,
                0,
            ][..],
        );
        let mut test_vm = VMScript::new(&script, None);
        test_vm.run();
        assert_eq!(test_vm.regs32[1], 0xFFFFFFFF >> 3);
    }

    #[test]
    fn test_shl32() {
        let reg = 0 + 1;
        let script = Bytes::from(
            &[
                Opcode::LOD as u8,
                reg,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                Opcode::SHL as u8,
                reg,
                3,
                0,
            ][..],
        );
        let mut test_vm = VMScript::new(&script, None);
        test_vm.run();
        assert_eq!(test_vm.regs32[1], 0xFFFFFFFF << 3);
    }

    #[test]
    fn test_lod64() {
        let reg = (1 << 6) + 1;
        let script = Bytes::from(
            &[
                Opcode::LOD as u8,
                reg,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0,
            ][..],
        );
        let mut test_vm = VMScript::new(&script, None);
        test_vm.run();
        assert_eq!(test_vm.regs64[1], -1);
    }

    #[test]
    fn test_shr64() {
        let reg = (1 << 6) + 1;
        let script = Bytes::from(
            &[
                Opcode::LOD as u8,
                reg,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                Opcode::SHR as u8,
                reg,
                3,
                0,
            ][..],
        );
        let mut test_vm = VMScript::new(&script, None);
        test_vm.run();
        assert_eq!(test_vm.regs64[1], 0xFFFFFFFFFFFFFFFF >> 3);
    }

    #[test]
    fn test_shl64() {
        let reg = (1 << 6) + 1;
        let script = Bytes::from(
            &[
                Opcode::LOD as u8,
                reg,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                Opcode::SHL as u8,
                reg,
                3,
                0,
            ][..],
        );
        let mut test_vm = VMScript::new(&script, None);
        test_vm.run();
        assert_eq!(test_vm.regs64[1], 0xFFFFFFFFFFFFFFFF << 3);
    }

    #[test]
    fn test_lod128() {
        let reg = (1 << 7) + 1;
        let script = Bytes::from(
            &[
                Opcode::LOD as u8,
                reg,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0,
            ][..],
        );
        let mut test_vm = VMScript::new(&script, None);
        test_vm.run();
        assert_eq!(test_vm.regs128[1], -1);
    }

    #[test]
    fn test_shr128() {
        let reg = (1 << 7) + 1;
        let script = Bytes::from(
            &[
                Opcode::LOD as u8,
                reg,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                Opcode::SHR as u8,
                reg,
                3,
                0,
            ][..],
        );
        let mut test_vm = VMScript::new(&script, None);
        test_vm.run();
        assert_eq!(test_vm.regs128[1], (-1 as i128) >> 3);
    }

    #[test]
    fn test_shl128() {
        let reg = (1 << 7) + 1;
        let script = Bytes::from(
            &[
                Opcode::LOD as u8,
                reg,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                Opcode::SHL as u8,
                reg,
                3,
                0,
            ][..],
        );
        let mut test_vm = VMScript::new(&script, None);
        test_vm.run();
        assert_eq!(test_vm.regs128[1], (-1 as i128) << 3);
    }

    #[test]
    fn test_add32() {
        let reg1: u8 = 0;
        let reg2: u8 = 0 + 1;
        let script = Bytes::from(
            &[
                Opcode::LOD as u8,
                reg1,
                0x0,
                0x0,
                0x0,
                0x01,
                Opcode::LOD as u8,
                reg2,
                0,
                0,
                0,
                100,
                Opcode::ADD as u8,
                reg1,
                reg2,
                0,
            ][..],
        );
        let mut test_vm = VMScript::new(&script, None);
        test_vm.run();
        assert_eq!(test_vm.regs32[0], 1 + 100);
    }

    #[test]
    fn test_sub32() {
        let reg1: u8 = 0;
        let reg2: u8 = 0 + 1;
        let script = Bytes::from(
            &[
                Opcode::LOD as u8,
                reg1,
                0x0,
                0x0,
                0x0,
                0x01,
                Opcode::LOD as u8,
                reg2,
                0,
                0,
                0,
                100,
                Opcode::SUB as u8,
                reg1,
                reg2,
                0,
            ][..],
        );
        let mut test_vm = VMScript::new(&script, None);
        test_vm.run();
        assert_eq!(test_vm.regs32[0], 1 - 100);
    }

    #[test]
    fn test_mul32() {
        let reg1: u8 = 0;
        let reg2: u8 = 0 + 1;
        let script = Bytes::from(
            &[
                Opcode::LOD as u8,
                reg1,
                0x0,
                0x0,
                0x0,
                0x02,
                Opcode::LOD as u8,
                reg2,
                0,
                0,
                0,
                100,
                Opcode::MUL as u8,
                reg1,
                reg2,
                0,
            ][..],
        );
        let mut test_vm = VMScript::new(&script, None);
        test_vm.run();
        assert_eq!(test_vm.regs32[0], 2 * 100);
    }

    #[test]
    fn test_div32() {
        let reg1: u8 = 0;
        let reg2: u8 = 0 + 1;
        let script = Bytes::from(
            &[
                Opcode::LOD as u8,
                reg1,
                0x0,
                0x0,
                0x0,
                100,
                Opcode::LOD as u8,
                reg2,
                0,
                0,
                0,
                3,
                Opcode::DIV as u8,
                reg1,
                reg2,
                0,
            ][..],
        );
        let mut test_vm = VMScript::new(&script, None);
        test_vm.run();
        assert_eq!(test_vm.regs32[0], 100 / 3);
        assert_eq!(test_vm.rem32, 100 % 3);
    }

    #[test]
    fn test_mod32() {
        let reg1: u8 = 0;
        let reg2: u8 = 0 + 1;
        let script = Bytes::from(
            &[
                Opcode::LOD as u8,
                reg1,
                0x0,
                0x0,
                0x0,
                100,
                Opcode::LOD as u8,
                reg2,
                0,
                0,
                0,
                3,
                Opcode::MOD as u8,
                reg1,
                reg2,
                0,
            ][..],
        );
        let mut test_vm = VMScript::new(&script, None);
        test_vm.run();
        assert_eq!(test_vm.regs32[0], 100 % 3);
    }

    #[test]
    fn test_many32() {
        let reg1: u8 = 0;
        let reg2: u8 = 0 + 1;
        let script = Bytes::from(
            &[
                Opcode::LOD as u8,
                reg1,
                0x0,
                0x0,
                0x0,
                100,
                Opcode::LOD as u8,
                reg2,
                0,
                0,
                0,
                3,
                Opcode::DIV as u8,
                reg1,
                reg2,
                Opcode::MUL as u8,
                reg1,
                reg2,
                Opcode::SUB as u8,
                reg1,
                reg2,
                0,
            ][..],
        );
        let mut test_vm = VMScript::new(&script, None);
        test_vm.run();
        assert_eq!(test_vm.regs32[0], (100 / 3) * 3 - 3);
    }

    #[test]
    fn test_add64() {
        let reg1: u8 = (1 << 6);
        let reg2: u8 = (1 << 6) + 1;
        let script = Bytes::from(
            &[
                Opcode::LOD as u8,
                reg1,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                1,
                Opcode::LOD as u8,
                reg2,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                100,
                Opcode::ADD as u8,
                reg1,
                reg2,
                0,
            ][..],
        );
        let mut test_vm = VMScript::new(&script, None);
        test_vm.run();
        assert_eq!(test_vm.regs64[0], 1 + 100);
    }

    #[test]
    fn test_sub64() {
        let reg1: u8 = (1 << 6);
        let reg2: u8 = (1 << 6) + 1;
        let script = Bytes::from(
            &[
                Opcode::LOD as u8,
                reg1,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                1,
                Opcode::LOD as u8,
                reg2,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                100,
                Opcode::SUB as u8,
                reg1,
                reg2,
                0,
            ][..],
        );
        let mut test_vm = VMScript::new(&script, None);
        test_vm.run();
        assert_eq!(test_vm.regs64[0], 1 - 100);
    }

    #[test]
    fn test_mul64() {
        let reg1: u8 = (1 << 6);
        let reg2: u8 = (1 << 6) + 1;
        let script = Bytes::from(
            &[
                Opcode::LOD as u8,
                reg1,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                2,
                Opcode::LOD as u8,
                reg2,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                100,
                Opcode::MUL as u8,
                reg1,
                reg2,
                0,
            ][..],
        );
        let mut test_vm = VMScript::new(&script, None);
        test_vm.run();
        assert_eq!(test_vm.regs64[0], 2 * 100);
    }

    #[test]
    fn test_div64() {
        let reg1: u8 = (1 << 6);
        let reg2: u8 = (1 << 6) + 1;
        let val1 = 100;
        let val2 = 3;
        let script = Bytes::from(
            &[
                Opcode::LOD as u8,
                reg1,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                val1 as u8,
                Opcode::LOD as u8,
                reg2,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                val2 as u8,
                Opcode::DIV as u8,
                reg1,
                reg2,
                0,
            ][..],
        );
        let mut test_vm = VMScript::new(&script, None);
        test_vm.run();
        assert_eq!(test_vm.regs64[0], 100 / 3);
        assert_eq!(test_vm.rem64, 100 % 3);
    }

    #[test]
    fn test_mod64() {
        let reg1: u8 = (1 << 6);
        let reg2: u8 = (1 << 6) + 1;
        let val1 = 100;
        let val2 = 3;
        let script = Bytes::from(
            &[
                Opcode::LOD as u8,
                reg1,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                val1 as u8,
                Opcode::LOD as u8,
                reg2,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                val2 as u8,
                Opcode::MOD as u8,
                reg1,
                reg2,
                0,
            ][..],
        );
        let mut test_vm = VMScript::new(&script, None);
        test_vm.run();
        assert_eq!(test_vm.regs64[0], 100 % 3);
    }

    #[test]
    fn test_add128() {
        let reg1: u8 = (1 << 7);
        let reg2: u8 = (1 << 7) + 1;
        let script = Bytes::from(
            &[
                Opcode::LOD as u8,
                reg1,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                1,
                Opcode::LOD as u8,
                reg2,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                100,
                Opcode::ADD as u8,
                reg1,
                reg2,
                0,
            ][..],
        );
        let mut test_vm = VMScript::new(&script, None);
        test_vm.run();
        assert_eq!(test_vm.regs128[0], 1 + 100);
    }

    #[test]
    fn test_sub128() {
        let reg1: u8 = (1 << 7);
        let reg2: u8 = (1 << 7) + 1;
        let script = Bytes::from(
            &[
                Opcode::LOD as u8,
                reg1,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                1,
                Opcode::LOD as u8,
                reg2,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                100,
                Opcode::SUB as u8,
                reg1,
                reg2,
                0,
            ][..],
        );
        let mut test_vm = VMScript::new(&script, None);
        test_vm.run();
        assert_eq!(test_vm.regs128[0], 1 - 100);
    }

    #[test]
    fn test_mul128() {
        let reg1: u8 = (1 << 7);
        let reg2: u8 = (1 << 7) + 1;
        let script = Bytes::from(
            &[
                Opcode::LOD as u8,
                reg1,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                2,
                Opcode::LOD as u8,
                reg2,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                100,
                Opcode::MUL as u8,
                reg1,
                reg2,
                0,
            ][..],
        );
        let mut test_vm = VMScript::new(&script, None);
        test_vm.run();
        assert_eq!(test_vm.regs128[0], 2 * 100);
    }

    #[test]
    fn test_div128() {
        let reg1: u8 = (1 << 7);
        let reg2: u8 = (1 << 7) + 1;
        let script = Bytes::from(
            &[
                Opcode::LOD as u8,
                reg1,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                100,
                Opcode::LOD as u8,
                reg2,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                3,
                Opcode::DIV as u8,
                reg1,
                reg2,
                0,
            ][..],
        );
        let mut test_vm = VMScript::new(&script, None);
        test_vm.run();
        assert_eq!(test_vm.regs128[0], 100 / 3);
        assert_eq!(test_vm.rem128, 100 % 3);
    }

    #[test]
    fn test_mod128() {
        let reg1: u8 = (1 << 7);
        let reg2: u8 = (1 << 7) + 1;
        let script = Bytes::from(
            &[
                Opcode::LOD as u8,
                reg1,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                100,
                Opcode::LOD as u8,
                reg2,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                3,
                Opcode::MOD as u8,
                reg1,
                reg2,
                0,
            ][..],
        );
        let mut test_vm = VMScript::new(&script, None);
        test_vm.run();
        assert_eq!(test_vm.regs128[0], 100 % 3);
    }

    // */
}
