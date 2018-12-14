extern crate bytes;

use self::bytes::{Buf, BufMut, Bytes, BytesMut};

//#[derive(Debug)]
pub struct VM {
    numscripts: usize,
    regs: [i128; 100],
}

impl VM {
    pub fn new(scripts: &[Bytes]) -> VM {
        VM {
            numscripts: scripts.len(),
            regs: [0;100],
        }
    }

    pub fn run() -> bool {
        
        false
    }
}
