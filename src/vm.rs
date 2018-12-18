extern crate bytes;

use self::bytes::{Buf, BufMut, Bytes, BytesMut};
use vm_script::VMScript;

//#[derive(Debug)]
pub struct VM<'a> {
    numscripts: usize,
    regs: [i32; 100],
    scripts: &'a [Bytes],
}

impl<'a> VM<'a> {
    pub fn new(scripts: &'a [Bytes]) -> VM {
        VM {
            numscripts: scripts.len(),
            regs: [0; 100],
            scripts: scripts,
        }
    }

    pub fn run(&mut self) -> bool {
        let mut vm_scr = VMScript::new(&self.scripts[0], self);
        self.regs[0] = vm_scr.regs32[0];

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use instruction::Opcode;
    #[test]
    fn test_vm_basics() {
        let reg = 0;
        let script = &[Bytes::from(
            &[Opcode::LOD as u8, reg, 0xFF, 0xFF, 0xFF, 0xFF, 0][..],
        )];
        let mut test_vm = VM::new(script);
        test_vm.run();
        assert_eq!(test_vm.regs[0], -1);
    }
}
