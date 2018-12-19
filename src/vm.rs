extern crate bytes;

use self::bytes::{Bytes};
use vm_script::VMScript;

//#[derive(Debug)]
pub struct VM<'a> {
    pub regs: [i32; 100],
    scripts: &'a [Bytes],
}

impl<'a> VM<'a> {
    pub fn new(scripts: &'a [Bytes]) -> VM {
        VM {
            regs: [0; 100],
            scripts,
        }
    }

    pub fn run(&mut self) -> bool {
        let mut vm_scr = VMScript::new(self.scripts);
        self.regs[0] = vm_scr.run();
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use instruction::Opcode;
    #[test]
    fn test_vm_cal() {
        let reg = 0;
        let script = &[
            Bytes::from(&[0xA, 0x0, 0][..]),
            Bytes::from(&[Opcode::LOD as u8, reg, 0xFF, 0xFF, 0xFF, 0xFF, 0][..]),
        ];
        let mut test_vm = VM::new(script);
        test_vm.run();
        assert_eq!(test_vm.regs[0], -1);
    }
}
