use super::*;

fn builtin_id(name: &str) -> Option<u8> {
    Some(match name {
        // String functions (0-11)
        "besar" => 0,
        "kecil" => 1,
        "bersih" => 2,
        "potong" => 3,
        "pecah" => 4,
        "gabung" => 5,
        "ganti" => 6,
        "mengandung" => 7,
        "mulai_dengan" => 8,
        "akhir_dengan" => 9,
        "ulang_teks" => 10,
        "format" => 11,
        // Math functions (12-27)
        "akar" => 12,
        "pangkat" => 13,
        "bulat_bawah" => 14,
        "bulat_atas" => 15,
        "pembulatan" => 16,
        "bundar" => 17,
        "mutlak" => 18,
        "abs" => 19,
        "acak_antara" => 20,
        "log" => 21,
        "faktorial" => 22,
        "min" => 23,
        "max" => 24,
        "sin" => 25,
        "cos" => 26,
        "tan" => 27,
        // List functions (28-43)
        "tambah" => 28,
        "sisip" => 29,
        "hapus" => 30,
        "urutkan" => 31,
        "balik" => 32,
        "unik" => 33,
        "jumlah" => 34,
        "rata_rata" => 35,
        "gabung_larik" => 36,
        "iris" => 37,
        "cari" => 38,
        "ada" => 39,
        "peta" => 40,
        "saring" => 41,
        "lipat" => 42,
        "setiap" => 43,
        // Dictionary functions (44-51)
        "kunci" => 44,
        "nilai" => 45,
        "pasangan" => 46,
        "ada_kunci" => 47,
        "hapus_kunci" => 48,
        "dapatkan" => 49,
        "setel" => 50,
        "gabung_objek" => 51,
        // System functions (52-61)
        "baca_file" => 52,
        "tulis_file" => 53,
        "ada_file" => 54,
        "env" => 55,
        "atur_env" => 56,
        "waktu_sekarang" => 57,
        "tanggal_sekarang" => 58,
        "format_tanggal" => 59,
        "tunda" => 60,
        "selisih_waktu" => 61,
        // Utility functions (62-74)
        "adalah_angka" => 62,
        "adalah_teks" => 63,
        "adalah_daftar" => 64,
        "adalah_kamus" => 65,
        "ke_boolean" => 66,
        "e_boolean" => 67,
        "ke_larik" => 68,
        "salin" => 69,
        "angka" => 70,
        "teks" => 71,
        "bolean" => 72,
        "daftar" => 73,
        "kamus" => 74,
        "argumen" => 75,
        _ => return None,
    })
}

impl Compiler {
    pub(crate) fn compile_expression(&mut self, expr: &Expr) -> Result<(), String> {
        match expr {
            Expr::Literal { value: lit, line } => {
                let val = match lit {
                    LiteralValue::Angka(n) => Value::Angka(*n),
                    LiteralValue::Teks(s) => Value::Teks(s.clone()),
                    LiteralValue::Bolean(b) => Value::Bolean(*b),
                    LiteralValue::Kosong => Value::Kosong,
                };
                self.emit_constant(val, *line);
                Ok(())
            }
            Expr::Identifier { name, line } => {
                // Constants pi & e (global, before local resolution)
                if (name == "pi" || name == "e")
                    && self.resolve_local(name).is_none()
                    && !self.declared_fns.contains(name)
                {
                    let val = if name == "pi" {
                        std::f64::consts::PI
                    } else {
                        std::f64::consts::E
                    };
                    self.emit_constant(Value::Angka(val), *line);
                    return Ok(());
                }

                if let Some(slot) = self.resolve_local(name) {
                    self.note_read(name);
                    self.emit_u8(OpCode::AmbilLokal as u8, *line);
                    self.emit_u16(slot, *line);
                } else {
                    let name_idx = self.chunk.add_constant(Value::Teks(name.clone()));
                    self.emit_u8(OpCode::AmbilGlobalNama as u8, *line);
                    self.emit_u16(name_idx, *line);
                }
                Ok(())
            }
            Expr::Binary {
                left,
                op,
                right,
                line,
            } => {
                self.compile_expression(left)?;
                self.compile_expression(right)?;
                self.emit_binary_op(op, *line);
                Ok(())
            }
            Expr::Unary {
                op,
                expr: inner,
                line,
            } => {
                self.compile_expression(inner)?;
                match op {
                    UnaryOp::Minus => {
                        self.emit_u8(OpCode::Negatif as u8, *line);
                    }
                    UnaryOp::Bukan => {
                        self.emit_u8(OpCode::Bukan as u8, *line);
                    }
                }
                Ok(())
            }
            Expr::Assign {
                target,
                op,
                value,
                line,
            } => self.compile_assign(target, op, value, *line),
            Expr::Call { callee, args, line } => {
                if let Expr::Identifier { name, .. } = callee.as_ref() {
                    if name == "cetak" {
                        for arg in args {
                            self.compile_expression(arg)?;
                        }
                        self.emit_u8(OpCode::Cetak as u8, *line);
                        self.emit_u8(args.len() as u8, *line);
                        return Ok(());
                    }
                    if name == "baca" || name == "baca_angka" {
                        for arg in args {
                            self.compile_expression(arg)?;
                        }
                        let op = if name == "baca" {
                            OpCode::Baca
                        } else {
                            OpCode::BacaAngka
                        };
                        self.emit_u8(op as u8, *line);
                        self.emit_u8(args.len() as u8, *line);
                        return Ok(());
                    }
                    if name == "bersihkan" {
                        self.emit_u8(OpCode::Bersihkan as u8, *line);
                        return Ok(());
                    }
                    if let Some(id) = builtin_id(name) {
                        if !self.declared_fns.contains(name) {
                            for arg in args {
                                self.compile_expression(arg)?;
                            }
                            self.emit_u8(OpCode::PanggilBuiltin as u8, *line);
                            self.emit_u8(id, *line);
                            self.emit_u8(args.len() as u8, *line);
                            return Ok(());
                        }
                    }
                }

                self.compile_expression(callee)?;
                for arg in args {
                    self.compile_expression(arg)?;
                }
                self.emit_u8(OpCode::Panggil as u8, *line);
                self.emit_u8(args.len() as u8, *line);
                Ok(())
            }
            Expr::Index {
                target,
                index,
                end,
                step,
                line,
            } => {
                if end.is_some() || step.is_some() {
                    return Err(format!(
                        "Baris {}: Slicing daftar/teks belum didukung",
                        line
                    ));
                }
                self.compile_expression(target)?;
                self.compile_expression(index)?;
                self.emit_u8(OpCode::AmbilIndeks as u8, *line);
                Ok(())
            }
            Expr::Property { target, name, line } => {
                self.compile_expression(target)?;
                self.emit_constant(Value::Teks(name.clone()), *line);
                self.emit_u8(OpCode::AmbilKunci as u8, *line);
                Ok(())
            }
            Expr::ArrayLiteral { elements, line } => {
                for elem in elements {
                    self.compile_expression(elem)?;
                }
                self.emit_u8(OpCode::BuatDaftar as u8, *line);
                self.emit_u16(elements.len() as u16, *line);
                Ok(())
            }
            Expr::DictLiteral { entries, line } => {
                for (key, value) in entries {
                    self.compile_expression(key)?;
                    self.compile_expression(value)?;
                }
                self.emit_u8(OpCode::BuatKamus as u8, *line);
                self.emit_u16(entries.len() as u16, *line);
                Ok(())
            }
            Expr::FuncExpr { params, body, line } => {
                let func = self.compile_function("<anonim>", params, body, *line)?;
                self.emit_constant(func, *line);
                Ok(())
            }
        }
    }

    fn compile_assign(
        &mut self,
        target: &Expr,
        op: &AssignOp,
        value: &Expr,
        line: usize,
    ) -> Result<(), String> {
        if op == &AssignOp::Assign {
            self.compile_expression(value)?;
            self.emit_u8(OpCode::Dup as u8, line);
            match target {
                Expr::Identifier { name, .. } => {
                    if let Some(slot) = self.resolve_local(name) {
                        self.emit_u8(OpCode::SimpanLokal as u8, line);
                        self.emit_u16(slot, line);
                    } else {
                        let name_idx = self.chunk.add_constant(Value::Teks(name.clone()));
                        self.emit_u8(OpCode::SimpanGlobalNama as u8, line);
                        self.emit_u16(name_idx, line);
                    }
                }
                Expr::Index { target, index, .. } => {
                    self.compile_expression(target)?;
                    self.compile_expression(index)?;
                    self.compile_expression(value)?;
                    self.emit_u8(OpCode::SimpanIndeks as u8, line);
                }
                Expr::Property { target, name, .. } => {
                    self.compile_expression(target)?;
                    self.emit_constant(Value::Teks(name.clone()), line);
                    self.compile_expression(value)?;
                    self.emit_u8(OpCode::SimpanKunci as u8, line);
                }
                _ => {
                    return Err(format!(
                        "Baris {}: Target penugasan harus berupa variabel, indeks, atau properti",
                        line
                    ))
                }
            }
            return Ok(());
        }

        let bin = match op {
            AssignOp::TambahAssign => Some(BinaryOp::Tambah),
            AssignOp::KurangAssign => Some(BinaryOp::Kurang),
            AssignOp::KaliAssign => Some(BinaryOp::Kali),
            AssignOp::BagiAssign => Some(BinaryOp::Bagi),
            AssignOp::Assign => None,
        };
        let Some(bin) = bin else {
            return Err(format!("Baris {}: Operator penugasan tidak dikenal", line));
        };

        let (is_local, name_opt) = match target {
            Expr::Identifier { name, .. } => {
                let slot = self.resolve_local(name);
                let is_local = slot.is_some();
                if let Some(slot) = slot {
                    self.note_read(name);
                    self.emit_u8(OpCode::AmbilLokal as u8, line);
                    self.emit_u16(slot, line);
                } else {
                    let name_idx = self.chunk.add_constant(Value::Teks(name.clone()));
                    self.emit_u8(OpCode::AmbilGlobalNama as u8, line);
                    self.emit_u16(name_idx, line);
                }
                (is_local, Some(name.clone()))
            }
            _ => {
                return Err(format!(
                    "Baris {}: Target penugasan harus berupa variabel",
                    line
                ))
            }
        };

        self.compile_expression(value)?;
        self.emit_binary_op(&bin, line);
        self.emit_u8(OpCode::Dup as u8, line);

        let name = name_opt.unwrap();
        if is_local {
            let slot = self.resolve_local(&name).unwrap();
            self.emit_u8(OpCode::SimpanLokal as u8, line);
            self.emit_u16(slot, line);
        } else {
            let name_idx = self.chunk.add_constant(Value::Teks(name));
            self.emit_u8(OpCode::SimpanGlobalNama as u8, line);
            self.emit_u16(name_idx, line);
        }
        Ok(())
    }
}
