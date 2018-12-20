#[macro_use]
extern crate nom;
extern crate bytes;

pub mod asm;
pub mod instruction;
pub mod vm;
pub mod vm_script;

use self::bytes::{Bytes};
use instruction::Opcode;
use vm::VM;

fn main() {
    let reg = 0;
    let script = &[
        Bytes::from(&[Opcode::CAL as u8, 0x0, 0][..]),
        Bytes::from(&[Opcode::LOD as u8, reg, 0xFF, 0xFF, 0xFF, 0xFF, 0][..]),
    ];
    let mut test_vm = VM::new(script);
    test_vm.run();
}
