#[macro_use]
extern crate nom;
extern crate bytes;

pub mod instruction;
pub mod vm;
pub mod vm_script;
pub mod asm;

use self::bytes::{Buf, BufMut, Bytes, BytesMut};
use vm_script::*;

fn main() {
    let script = Bytes::from_static(&[2, 0, 0, 0, 0, 0xFF]);
    let mut test_vm = VMScript::new(script);
    test_vm.run();
}
