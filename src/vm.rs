extern crate bytes;

use self::bytes::{Bytes, BytesMut};
use vm_script::VMScript;

//#[derive(Debug)]
pub struct VM<'a> {
    scripts: &'a [Bytes],
    pub heap: BytesMut,
}

impl<'a> VM<'a> {
    pub fn new(scripts: &'a [Bytes]) -> VM {
        VM {
            scripts,
            heap: BytesMut::with_capacity(0xFF),
        }
    }

    pub fn run(&mut self) -> bool {
        let mut vm_scr = VMScript::new(self.scripts, &mut self.heap);
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
            Bytes::from(&[Opcode::CAL as u8, 0x0, 0][..]),  // Call script at relative offset 0, so the next script in the list
            Bytes::from(&[Opcode::LOD as u8, reg, 0xFF, 0xFF, 0xFF, 0xFF, Opcode::PSH as u8, reg, Opcode::CAL as u8, 0x0, 0][..]),  // Load 0xFFFFFFFF into reg0, push reg0, call script at offset 0
            Bytes::from(&[Opcode::CAL as u8, 0x0, 0][..]),  // Call script at offset 0
            Bytes::from(&[Opcode::LOD as u8, reg, 0x0, 0x0, 0x0, 0xFF, Opcode::PSH as u8, reg, 0][..]), // Load 0xFF into reg0, push reg0
        ];
        let mut test_vm = VM::new(script);
        assert_eq!(test_vm.run(), true);
        assert_eq!(test_vm.heap, Bytes::from(&[0xFF,0xFF,0xFF,0xFF,0xFF,0x0,0x0,0x0][..])); // Verify memory is what it should be
    }

   
}
