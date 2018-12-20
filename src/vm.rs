extern crate bytes;

use self::bytes::{Bytes};
use vm_script::VMScript;

//#[derive(Debug)]
pub struct VM<'a> {
    scripts: &'a [Bytes],
}

impl<'a> VM<'a> {
    pub fn new(scripts: &'a [Bytes]) -> VM {
        VM {
            scripts,
        }
    }

    pub fn run(&mut self) -> i32 {
        let mut vm_scr = VMScript::new(self.scripts);
        vm_scr.run()
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
            Bytes::from(&[Opcode::CAL as u8, 0x0, 0][..]),
            Bytes::from(&[Opcode::CAL as u8, 0x1, 0][..]),
            Bytes::from(&[Opcode::LOD as u8, reg, 0xFF, 0xFF, 0xFF, 0xFF, Opcode::RET as u8, reg, 0][..]),
            Bytes::from(&[Opcode::LOD as u8, reg, 0x0, 0x0, 0x0, 0xFF, Opcode::RET as u8, reg, 0][..]),
        ];
        let mut test_vm = VM::new(script);
        let ret = test_vm.run();
        assert_eq!(ret, 0xFF);
    }

   
}
