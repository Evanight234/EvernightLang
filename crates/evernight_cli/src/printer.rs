use evernight_core::errors::{ErrorLevel, EvernightError};
use evernight_vm::{Chunk, OpCode};

pub fn format_error(err: &EvernightError, source: Option<&str>, use_color: bool) -> String {
    let (level_str, level_color) = match err.level {
        ErrorLevel::Bahaya => ("BAHAYA", "\x1b[1;31m"),
        ErrorLevel::Peringatan => ("PERINGATAN", "\x1b[1;33m"),
    };

    let reset = if use_color { "\x1b[0m" } else { "" };
    let bold = if use_color { "\x1b[1m" } else { "" };
    let cyan = if use_color { "\x1b[36m" } else { "" };
    let color = if use_color { level_color } else { "" };

    let mut out = String::new();
    out.push_str(&format!(
        "{}{} [{}] Baris {} (Kolom {}) - {}{}\n",
        color, level_str, err.code, err.line, err.column, err.message, reset
    ));

    if let Some(src) = source {
        if err.line > 0 {
            let lines: Vec<&str> = src.lines().collect();
            if err.line <= lines.len() {
                let line_str = lines[err.line - 1];
                let line_num_prefix = format!("{:4} | ", err.line);
                out.push_str(&format!(
                    "{}{}{}{}\n",
                    cyan, line_num_prefix, reset, line_str
                ));

                let pad = if err.column > 0 { err.column - 1 } else { 0 };
                let spaces = " ".repeat(line_num_prefix.len() + pad);
                out.push_str(&format!("{}{}{}^{}\n", spaces, color, bold, reset));
            }
        }
    }

    out
}

pub fn disassemble_chunk(chunk: &Chunk, name: &str) -> String {
    let mut out = String::new();
    out.push_str(&format!("=== BYTECODE DISASSEMBLY ({}) ===\n", name));
    out.push_str(&format!("Konstanta ({}):\n", chunk.constants.len()));
    for (i, c) in chunk.constants.iter().enumerate() {
        out.push_str(&format!("  [{:04}] {}\n", i, c));
    }
    out.push_str("Instruksi:\n");

    let mut offset = 0;
    while offset < chunk.code.len() {
        let line = chunk.lines[offset];
        let op_byte = chunk.code[offset];
        let line_info = if offset > 0 && chunk.lines[offset - 1] == line {
            "   |".to_string()
        } else {
            format!("{:4}", line)
        };

        if let Some(opcode) = OpCode::from_u8(op_byte) {
            let (op_name, size) = match opcode {
                OpCode::Konstanta
                | OpCode::AmbilLokal
                | OpCode::SimpanLokal
                | OpCode::AmbilGlobal
                | OpCode::SimpanGlobal
                | OpCode::AmbilGlobalNama
                | OpCode::SimpanGlobalNama
                | OpCode::Lompat
                | OpCode::LompatJikaSalah
                | OpCode::LompatJikaBenar
                | OpCode::Loop
                | OpCode::ImporModul => {
                    let operand = if offset + 2 < chunk.code.len() {
                        let high = chunk.code[offset + 1] as u16;
                        let low = chunk.code[offset + 2] as u16;
                        (high << 8) | low
                    } else {
                        0
                    };
                    (format!("{:?} {}", opcode, operand), 3)
                }
                OpCode::Panggil | OpCode::PanggilBuiltin | OpCode::Baca | OpCode::BacaAngka => {
                    let arg1 = chunk.code.get(offset + 1).copied().unwrap_or(0);
                    let arg2 = chunk.code.get(offset + 2).copied().unwrap_or(0);
                    if matches!(opcode, OpCode::PanggilBuiltin) {
                        (format!("{:?} id:{} args:{}", opcode, arg1, arg2), 3)
                    } else {
                        (format!("{:?} {}", opcode, arg1), 2)
                    }
                }
                _ => (format!("{:?}", opcode), 1),
            };

            out.push_str(&format!("{:04} {} {}\n", offset, line_info, op_name));
            offset += size;
        } else {
            out.push_str(&format!(
                "{:04} {} UNKNOWN (0x{:02X})\n",
                offset, line_info, op_byte
            ));
            offset += 1;
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use evernight_core::errors::EvernightError;
    use evernight_vm::Value;

    #[test]
    fn test_format_error_no_color() {
        let err = EvernightError::bahaya("DIV0", 2, 5, "Pembagian dengan nol!");
        let src = "variabel a = 1\nvariabel b = a / 0\ncetak(b)";
        let res = format_error(&err, Some(src), false);
        assert!(res.contains("BAHAYA [DIV0] Baris 2 (Kolom 5) - Pembagian dengan nol!"));
        assert!(res.contains("   2 | variabel b = a / 0"));
        assert!(res.contains("^"));
        assert!(!res.contains("\x1b["));
    }

    #[test]
    fn test_format_error_with_color() {
        let err = EvernightError::peringatan("WKVAR", 1, 1, "Variabel tidak digunakan");
        let res = format_error(&err, None, true);
        assert!(res.contains("\x1b[1;33m"));
        assert!(res.contains("PERINGATAN"));
    }

    #[test]
    fn test_disassemble_chunk() {
        let mut chunk = Chunk::new();
        let idx = chunk.add_constant(Value::Angka(42.0));
        chunk.write(OpCode::Konstanta as u8, 1);
        chunk.write_u16(idx, 1);
        chunk.write(OpCode::Kembali as u8, 1);

        let dis = disassemble_chunk(&chunk, "test");
        assert!(dis.contains("Konstanta (1):"));
        assert!(dis.contains("42"));
        assert!(dis.contains("Konstanta 0"));
        assert!(dis.contains("Kembali"));
    }
}
