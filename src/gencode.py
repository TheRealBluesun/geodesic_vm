
from os import path, getcwd

enumStr = 'pub enum Opcode{\n'
u8Str = "impl From<u8> for Opcode {\n\tfn from(v: u8) -> Self {\n\t\tmatch v {\n"
#vmStr = "match self.decode_opcode() {\n"
vmStr = ""


idx = 0
with open("./src/instructions.codegen.txt") as f:
    for ins in f:
        tok = ins.strip()
        enumStr += "\t{0},\n".format(tok)
        u8Str += "\t\t\t0x{0:X} => Opcode::{1},\n".format(idx, tok)
        vmStr += "Opcode::{0} => {{}}\n".format(tok)
        idx +=1

enumStr += '\tERR,\n}'
u8Str += "\t\t\t_=> Opcode::ERR\n\t\t}\n\t}\n}"

# print(enumStr)
# print(u8Str)
# print(vmStr)

with open("./src/opcodes.rs", 'w') as f:
    f.write(enumStr)
    f.write('\n\n')
    f.write(u8Str)
    f.write('\n\n')
    f.write(vmStr)