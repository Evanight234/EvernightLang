use super::*;

impl Compiler {
    pub(crate) fn compile_statement(&mut self, stmt: &Statement) -> Result<(), String> {
        match stmt {
            Statement::VarDecl {
                name, init, line, ..
            } => {
                if let Some(expr) = init {
                    self.compile_expression(expr)?;
                } else {
                    self.emit_constant(Value::Kosong, *line);
                }
                if !self.is_function && self.locals.is_empty() {
                    let name_idx = self.chunk.add_constant(Value::Teks(name.clone()));
                    self.emit_u8(OpCode::SimpanGlobalNama as u8, *line);
                    self.emit_u16(name_idx, *line);
                } else {
                    let slot = self.define_local(name.clone(), *line);
                    self.emit_u8(OpCode::SimpanLokal as u8, *line);
                    self.emit_u16(slot as u16, *line);
                }
                Ok(())
            }
            Statement::ExprStatement { expr, line } => {
                self.compile_expression(expr)?;
                self.emit_u8(OpCode::Pop as u8, *line);
                Ok(())
            }
            Statement::ReturnStatement { value, line } => {
                if let Some(val) = value {
                    self.compile_expression(val)?;
                } else {
                    self.emit_constant(Value::Kosong, *line);
                }
                self.emit_u8(OpCode::Kembali as u8, *line);
                Ok(())
            }
            Statement::Block(block) => {
                let scope = self.locals.len();
                self.compile_statement_list(&block.statements, block.line)?;
                self.pop_locals_until(scope, block.line);
                Ok(())
            }
            Statement::IfStatement {
                condition,
                then_branch,
                elif_branches,
                else_branch,
                line,
            } => {
                self.compile_expression(condition)?;
                let then_jump = self.emit_jump(OpCode::LompatJikaSalah as u8, *line);

                let then_scope = self.locals.len();
                self.compile_statement_list(&then_branch.statements, *line)?;
                self.pop_locals_until(then_scope, *line);

                let mut end_jumps = vec![];
                if !elif_branches.is_empty() || else_branch.is_some() {
                    end_jumps.push(self.emit_jump(OpCode::Lompat as u8, *line));
                }
                self.patch_jump(then_jump);

                for (elif_cond, elif_body) in elif_branches {
                    self.compile_expression(elif_cond)?;
                    let elif_jump = self.emit_jump(OpCode::LompatJikaSalah as u8, *line);

                    let elif_scope = self.locals.len();
                    self.compile_statement_list(&elif_body.statements, *line)?;
                    self.pop_locals_until(elif_scope, *line);

                    end_jumps.push(self.emit_jump(OpCode::Lompat as u8, *line));
                    self.patch_jump(elif_jump);
                }

                if let Some(else_body) = else_branch {
                    let else_scope = self.locals.len();
                    self.compile_statement_list(&else_body.statements, *line)?;
                    self.pop_locals_until(else_scope, *line);
                }

                for jump in end_jumps {
                    self.patch_jump(jump);
                }
                Ok(())
            }
            Statement::WhileStatement {
                condition,
                body,
                line,
            } => {
                let start_ip = self.chunk.code_len();
                self.loop_stack.push(LoopInfo {
                    continue_ip: start_ip,
                    depth: self.locals.len(),
                    break_jumps: Vec::new(),
                });

                self.compile_expression(condition)?;
                let exit_jump = self.emit_jump(OpCode::LompatJikaSalah as u8, *line);

                let body_scope = self.locals.len();
                self.compile_statement_list(&body.statements, *line)?;
                self.pop_locals_until(body_scope, *line);

                self.emit_loop(start_ip, *line);

                let loop_info = self.loop_stack.pop().unwrap();
                self.patch_jump(exit_jump);
                for jump in &loop_info.break_jumps {
                    self.patch_jump(*jump);
                }
                Ok(())
            }
            Statement::ForStatement {
                var_name,
                iter,
                body,
                line,
            } => match iter {
                ForIter::Range { start, end } => {
                    let scope = self.locals.len();
                    let slot = self.define_local(var_name.clone(), *line);

                    self.compile_expression(start)?;
                    self.emit_u8(OpCode::SimpanLokal as u8, *line);
                    self.emit_u16(slot as u16, *line);

                    let start_ip = self.chunk.code_len();
                    self.loop_stack.push(LoopInfo {
                        continue_ip: start_ip,
                        depth: scope,
                        break_jumps: Vec::new(),
                    });

                    self.emit_u8(OpCode::AmbilLokal as u8, *line);
                    self.emit_u16(slot as u16, *line);
                    self.compile_expression(end)?;
                    self.emit_binary_op(&BinaryOp::KurangSama, *line);
                    let exit_jump = self.emit_jump(OpCode::LompatJikaSalah as u8, *line);

                    let body_scope = self.locals.len();
                    self.compile_statement_list(&body.statements, *line)?;
                    self.pop_locals_until(body_scope, *line);

                    let inc_ip = self.chunk.code_len();
                    if let Some(info) = self.loop_stack.last_mut() {
                        info.continue_ip = inc_ip;
                    }
                    self.emit_u8(OpCode::AmbilLokal as u8, *line);
                    self.emit_u16(slot as u16, *line);
                    self.emit_constant(Value::Angka(1.0), *line);
                    self.emit_binary_op(&BinaryOp::Tambah, *line);
                    self.emit_u8(OpCode::SimpanLokal as u8, *line);
                    self.emit_u16(slot as u16, *line);

                    self.emit_loop(start_ip, *line);

                    let loop_info = self.loop_stack.pop().unwrap();
                    self.patch_jump(exit_jump);
                    for jump in &loop_info.break_jumps {
                        self.patch_jump(*jump);
                    }
                    self.pop_locals_until(scope, *line);
                    Ok(())
                }
                ForIter::Collection(collection) => {
                    let scope = self.locals.len();
                    let tmp = self.define_local("(koleksi)".to_string(), *line);
                    self.compile_expression(collection)?;
                    self.emit_u8(OpCode::SimpanLokal as u8, *line);
                    self.emit_u16(tmp as u16, *line);

                    let idx = self.define_local("(indeks)".to_string(), *line);
                    self.emit_constant(Value::Angka(0.0), *line);
                    self.emit_u8(OpCode::SimpanLokal as u8, *line);
                    self.emit_u16(idx as u16, *line);

                    let slot = self.define_local(var_name.clone(), *line);

                    let start_ip = self.chunk.code_len();
                    self.loop_stack.push(LoopInfo {
                        continue_ip: start_ip,
                        depth: scope,
                        break_jumps: Vec::new(),
                    });

                    self.emit_u8(OpCode::AmbilLokal as u8, *line);
                    self.emit_u16(idx as u16, *line);
                    self.emit_u8(OpCode::AmbilLokal as u8, *line);
                    self.emit_u16(tmp as u16, *line);
                    self.emit_u8(OpCode::PanjangDaftar as u8, *line);
                    self.emit_binary_op(&BinaryOp::KurangDari, *line);
                    let exit_jump = self.emit_jump(OpCode::LompatJikaSalah as u8, *line);

                    self.emit_u8(OpCode::AmbilLokal as u8, *line);
                    self.emit_u16(tmp as u16, *line);
                    self.emit_u8(OpCode::AmbilLokal as u8, *line);
                    self.emit_u16(idx as u16, *line);
                    self.emit_u8(OpCode::AmbilIndeks as u8, *line);
                    self.emit_u8(OpCode::SimpanLokal as u8, *line);
                    self.emit_u16(slot as u16, *line);

                    let body_scope = self.locals.len();
                    self.compile_statement_list(&body.statements, *line)?;
                    self.pop_locals_until(body_scope, *line);

                    let inc_ip = self.chunk.code_len();
                    if let Some(info) = self.loop_stack.last_mut() {
                        info.continue_ip = inc_ip;
                    }
                    self.emit_u8(OpCode::AmbilLokal as u8, *line);
                    self.emit_u16(idx as u16, *line);
                    self.emit_constant(Value::Angka(1.0), *line);
                    self.emit_binary_op(&BinaryOp::Tambah, *line);
                    self.emit_u8(OpCode::SimpanLokal as u8, *line);
                    self.emit_u16(idx as u16, *line);

                    self.emit_loop(start_ip, *line);

                    let loop_info = self.loop_stack.pop().unwrap();
                    self.patch_jump(exit_jump);
                    for jump in &loop_info.break_jumps {
                        self.patch_jump(*jump);
                    }
                    self.pop_locals_until(scope, *line);
                    Ok(())
                }
            },
            Statement::FuncDecl {
                name,
                params,
                body,
                line,
            } => {
                self.declared_fns.insert(name.clone());
                let func_val = self.compile_function(name, params, body, *line)?;
                let name_idx = self.chunk.add_constant(Value::Teks(name.clone()));
                self.emit_constant(func_val, *line);
                self.emit_u8(OpCode::SimpanGlobalNama as u8, *line);
                self.emit_u16(name_idx, *line);
                Ok(())
            }
            Statement::BreakStatement { line } => {
                let Some(depth) = self.loop_stack.last().map(|l| l.depth) else {
                    return Err(format!("Baris {}: 'berhenti' di luar loop", line));
                };
                self.pop_locals_until(depth, *line);
                let jump = self.emit_jump(OpCode::Lompat as u8, *line);
                if let Some(info) = self.loop_stack.last_mut() {
                    info.break_jumps.push(jump);
                }
                Ok(())
            }
            Statement::ContinueStatement { line } => {
                let Some((depth, continue_ip)) =
                    self.loop_stack.last().map(|l| (l.depth, l.continue_ip))
                else {
                    return Err(format!("Baris {}: 'lanjut' di luar loop", line));
                };
                self.pop_locals_until(depth, *line);
                self.emit_loop(continue_ip, *line);
                Ok(())
            }
            Statement::HapusStatement { target, line } => match target {
                Expr::Index { target, index, .. } => {
                    self.compile_expression(target)?;
                    self.compile_expression(index)?;
                    self.emit_u8(OpCode::HapusIndeks as u8, *line);
                    Ok(())
                }
                Expr::Property { target, name, .. } => {
                    self.compile_expression(target)?;
                    self.emit_constant(Value::Teks(name.clone()), *line);
                    self.emit_u8(OpCode::HapusKunci as u8, *line);
                    Ok(())
                }
                _ => Err(format!(
                    "Baris {}: Target 'hapus' harus berupa indeks daftar atau kunci kamus",
                    line
                )),
            },
            Statement::MatchStatement {
                expr,
                cases,
                wildcard,
                default,
                line,
            } => {
                self.compile_expression(expr)?;

                let mut end_jumps = vec![];
                for (pattern, body) in cases {
                    self.emit_u8(OpCode::Dup as u8, *line);
                    self.compile_expression(pattern)?;
                    self.emit_u8(OpCode::SamaDengan as u8, *line);
                    let skip = self.emit_jump(OpCode::LompatJikaSalah as u8, *line);
                    self.emit_u8(OpCode::Pop as u8, *line);

                    let body_scope = self.locals.len();
                    self.compile_statement_list(&body.statements, *line)?;
                    self.pop_locals_until(body_scope, *line);

                    end_jumps.push(self.emit_jump(OpCode::Lompat as u8, *line));
                    self.patch_jump(skip);
                }

                self.emit_u8(OpCode::Pop as u8, *line);

                let fallback = wildcard.as_ref().or(default.as_ref());
                if let Some(fb) = fallback {
                    let fb_scope = self.locals.len();
                    self.compile_statement_list(&fb.statements, *line)?;
                    self.pop_locals_until(fb_scope, *line);
                }

                for jump in end_jumps {
                    self.patch_jump(jump);
                }
                Ok(())
            }
            Statement::TryCatchStatement {
                try_block,
                catch_var,
                catch_block,
                finally_block,
                line,
            } => {
                let handler = self.emit_jump(OpCode::BuatHandler as u8, *line);
                let try_scope = self.locals.len();
                self.compile_statement_list(&try_block.statements, *line)?;
                self.pop_locals_until(try_scope, *line);
                self.emit_u8(OpCode::PopHandler as u8, *line);

                let skip = self.emit_jump(OpCode::Lompat as u8, *line);
                self.patch_jump(handler);

                let catch_scope = self.locals.len();
                let slot = self.define_local(catch_var.clone(), *line);
                self.mark_used(catch_var);
                self.emit_u8(OpCode::SimpanLokal as u8, *line);
                self.emit_u16(slot as u16, *line);
                self.compile_statement_list(&catch_block.statements, *line)?;
                self.pop_locals_until(catch_scope, *line);
                self.patch_jump(skip);

                if let Some(fb) = finally_block {
                    let fb_scope = self.locals.len();
                    self.compile_statement_list(&fb.statements, *line)?;
                    self.pop_locals_until(fb_scope, *line);
                }
                Ok(())
            }
            Statement::ThrowStatement { expr, line } => {
                self.compile_expression(expr)?;
                self.emit_u8(OpCode::Lempar as u8, *line);
                Ok(())
            }
            Statement::AssertStatement {
                condition,
                message,
                line,
            } => {
                if let Some(msg) = message {
                    self.compile_expression(msg)?;
                }
                self.compile_expression(condition)?;
                let flag = if message.is_some() { 1u16 } else { 0xFFFF };
                self.emit_u8(OpCode::Pastikan as u8, *line);
                self.emit_u16(flag, *line);
                Ok(())
            }
            Statement::ImportStatement {
                module,
                alias,
                line,
                ..
            } => {
                match module.as_str() {
                    "konsol" | "string" | "matematika" | "daftar" | "kamus" | "sistem"
                    | "utilitas" => {}
                    _ => {
                        // Modul berkas — di-resolve di VM (lazy-load + cache).
                        let mount = alias.clone().unwrap_or_else(|| {
                            let base = module.rsplit(['/', '\\']).next().unwrap_or(module);
                            base.strip_suffix(".eve").unwrap_or(base).to_string()
                        });
                        let mount_idx = self.chunk.add_constant(Value::Teks(mount.clone()));
                        self.emit_constant(Value::Teks(module.clone()), *line);
                        self.emit_u8(OpCode::ImporModul as u8, *line);
                        self.emit_u8(OpCode::SimpanGlobalNama as u8, *line);
                        self.emit_u16(mount_idx, *line);
                    }
                }
                Ok(())
            }
        }
    }
}
