// #![feature(test)]
// extern crate test;
#[macro_use]
extern crate nom;
extern crate bytes;

pub mod asm;
pub mod instruction;
pub mod vm;
pub mod vm_script;

use self::bytes::Bytes;
use instruction::Opcode;
use vm::VM;

fn main() {
    let reg = 0;
    let script = &[
        Bytes::from(&[Opcode::CAL as u8, 0x0, 0][..]),
        Bytes::from(&[Opcode::CAL as u8, 0x1, 0][..]),
        Bytes::from(
            &[
                Opcode::LOD as u8,
                reg,
                0xFF,
                0xFF,
                0xFF,
                0xFF,
                Opcode::PSH as u8,
                reg,
                0,
            ][..],
        ),
        Bytes::from(
            &[
                Opcode::LOD as u8,
                reg,
                0x0,
                0x0,
                0x0,
                0xFF,
                Opcode::PSH as u8,
                reg,
                0,
            ][..],
        ),
    ];
    let mut test_vm = VM::new(script);
    let ret = test_vm.run();
    assert_eq!(ret, true);
}

#[cfg(test)]

mod tests {
    use super::*;
    /*
    use test::Bencher;

    #[bench]
    fn test_benchmark_vm_add(b: &mut Bencher) {
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
        let script_arr = [script];
        let mut test_vm = VM::new(&script_arr);

        b.iter(|| test_vm.run())
    }

    #[bench]
    fn test_benchmark_vm_sub(b: &mut Bencher) {
        let reg1: u8 = 0;
        let reg2: u8 = 0 + 1;
        let script = &[
            Bytes::from(&[Opcode::CAL as u8, 0x0, 0][..]),
            // Bytes::from(&[Opcode::CAL as u8, 0x1, 0][..]),
            // Bytes::from(&[Opcode::LOD as u8, reg1, 0xFF, 0xFF, 0xFF, 0xFF, Opcode::RET as u8, reg1, 0][..]),
            Bytes::from(
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
            ),
        ];
        let mut test_vm = VM::new(script);

        b.iter(|| test_vm.run())
    }
    // */
}
