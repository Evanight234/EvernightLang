use super::*;

impl Compiler {
    pub(crate) fn emit_u8(&mut self, byte: u8, line: usize) {
        self.chunk.write(byte, line);
    }

    pub(crate) fn emit_u16(&mut self, value: u16, line: usize) {
        self.chunk.write_u16(value, line);
    }

    pub(crate) fn emit_i16(&mut self, value: i16, line: usize) {
        self.chunk.write_i16(value, line);
    }

    pub(crate) fn emit_constant(&mut self, value: Value, line: usize) {
        let idx = self.chunk.add_constant(value);
        self.emit_u8(OpCode::Konstanta as u8, line);
        self.emit_u16(idx, line);
    }

    pub(crate) fn emit_binary_op(&mut self, op: &BinaryOp, line: usize) {
        let opcode = match op {
            BinaryOp::Tambah => OpCode::Tambah,
            BinaryOp::Kurang => OpCode::Kurang,
            BinaryOp::Kali => OpCode::Kali,
            BinaryOp::Bagi => OpCode::Bagi,
            BinaryOp::Modulo => OpCode::Modulo,
            BinaryOp::Pangkat => OpCode::Pangkat,
            BinaryOp::SamaDengan => OpCode::SamaDengan,
            BinaryOp::TidakSama => OpCode::TidakSama,
            BinaryOp::KurangDari => OpCode::KurangDari,
            BinaryOp::KurangSama => OpCode::KurangSama,
            BinaryOp::LebihDari => OpCode::LebihDari,
            BinaryOp::LebihSama => OpCode::LebihSama,
            BinaryOp::Dan => OpCode::Dan,
            BinaryOp::Atau => OpCode::Atau,
            BinaryOp::Dalam => OpCode::Dalam,
        };
        self.emit_u8(opcode as u8, line);
    }

    pub(crate) fn emit_jump(&mut self, opcode: u8, line: usize) -> usize {
        self.emit_u8(opcode, line);
        self.emit_u16(0, line);
        self.chunk.code_len() - 2
    }

    pub(crate) fn patch_jump(&mut self, offset: usize) {
        let jump = (self.chunk.code_len() - offset - 2) as i16;
        self.chunk.code[offset] = (jump >> 8) as u8;
        self.chunk.code[offset + 1] = (jump & 0xFF) as u8;
    }

    pub(crate) fn emit_loop(&mut self, start_ip: usize, line: usize) {
        self.emit_u8(OpCode::Loop as u8, line);
        let offset = (self.chunk.code_len() - start_ip + 2) as i16;
        self.emit_i16(offset, line);
    }
}
