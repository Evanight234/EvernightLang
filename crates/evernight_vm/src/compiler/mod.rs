use crate::bytecode::{Chunk, OpCode};
use crate::value::Value;
use evernight_core::ast::*;
use evernight_core::errors::EvernightError;
use std::collections::HashSet;
use std::rc::Rc;

mod codegen;
mod expressions;
mod statements;

pub struct Compiler {
    pub chunk: Chunk,
    locals: Vec<Local>,
    loop_stack: Vec<LoopInfo>,
    pub warnings: Vec<EvernightError>,
    pub(crate) declared_fns: HashSet<String>,
    pub(crate) is_function: bool,
}

struct Local {
    name: String,
    used: bool,
    line: usize,
}

struct LoopInfo {
    continue_ip: usize,
    depth: usize,
    break_jumps: Vec<usize>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            chunk: Chunk::new(),
            locals: Vec::new(),
            loop_stack: Vec::new(),
            warnings: Vec::new(),
            declared_fns: HashSet::new(),
            is_function: false,
        }
    }

    pub fn compile(&mut self, statements: &[Statement]) -> Result<(), String> {
        self.compile_statement_list(statements, 1)?;
        self.warn_unused_locals();
        self.emit_u8(OpCode::Henti as u8, 1);
        Ok(())
    }

    pub(crate) fn define_local(&mut self, name: String, line: usize) -> usize {
        self.locals.push(Local {
            name,
            used: false,
            line,
        });
        self.locals.len() - 1
    }

    pub(crate) fn resolve_local(&self, name: &str) -> Option<u16> {
        self.locals
            .iter()
            .position(|l| l.name == name)
            .map(|i| i as u16)
    }

    pub(crate) fn note_read(&mut self, name: &str) {
        for local in self.locals.iter_mut().rev() {
            if local.name == name {
                local.used = true;
                break;
            }
        }
    }

    pub(crate) fn mark_used(&mut self, name: &str) {
        for local in self.locals.iter_mut() {
            if local.name == name {
                local.used = true;
            }
        }
    }

    pub(crate) fn pop_locals_until(&mut self, depth: usize, line: usize) {
        while self.locals.len() > depth {
            let local = self.locals.pop().unwrap();
            if !local.used {
                self.warnings.push(EvernightError::peringatan(
                    "WKVAR",
                    local.line,
                    0,
                    format!(
                        "Variabel '{}' dideklarasikan tetapi tidak pernah digunakan!",
                        local.name
                    ),
                ));
            }
            self.emit_u8(OpCode::Pop as u8, line);
        }
    }

    pub(crate) fn warn_unused_locals(&mut self) {
        for local in &self.locals {
            if !local.used {
                self.warnings.push(EvernightError::peringatan(
                    "WKVAR",
                    local.line,
                    0,
                    format!(
                        "Variabel '{}' dideklarasikan tetapi tidak pernah digunakan!",
                        local.name
                    ),
                ));
            }
        }
    }

    pub(crate) fn emit_warning(&mut self, code: &str, line: usize, message: String) {
        self.warnings
            .push(EvernightError::peringatan(code, line, 0, message));
    }

    pub(crate) fn compile_statement_list(
        &mut self,
        stmts: &[Statement],
        default_line: usize,
    ) -> Result<(), String> {
        let mut unreachable = false;
        for stmt in stmts {
            if unreachable {
                let line = statement_line(stmt).unwrap_or(default_line);
                self.emit_warning(
                    "WKREACH",
                    line,
                    "Kode setelah pernyataan 'kembali' tidak akan pernah dieksekusi!".to_string(),
                );
            }
            self.compile_statement(stmt)?;
            if Self::is_exit_stmt(stmt) {
                unreachable = true;
            }
        }
        Ok(())
    }

    fn is_exit_stmt(stmt: &Statement) -> bool {
        matches!(
            stmt,
            Statement::ReturnStatement { .. }
                | Statement::BreakStatement { .. }
                | Statement::ContinueStatement { .. }
                | Statement::ThrowStatement { .. }
        )
    }

    pub(crate) fn compile_function(
        &mut self,
        name: &str,
        params: &[Param],
        body: &Block,
        line: usize,
    ) -> Result<Value, String> {
        let mut func_compiler = Compiler::new();
        func_compiler.is_function = true;

        for param in params {
            func_compiler.define_local(param.name.clone(), line);
        }

        if body.statements.is_empty() {
            func_compiler.emit_warning(
                "WKFUNG",
                line,
                format!("Fungsi '{}' tidak memiliki isi!", name),
            );
        }

        func_compiler.compile_statement_list(&body.statements, line)?;
        func_compiler.warn_unused_locals();

        func_compiler.emit_constant(Value::Kosong, line);
        func_compiler.emit_u8(OpCode::Kembali as u8, line);
        func_compiler.emit_u8(OpCode::Henti as u8, line);

        self.warnings.append(&mut func_compiler.warnings);

        let chunk = Rc::new(func_compiler.chunk);
        Ok(Value::Fungsi {
            name: name.to_string(),
            arity: params.len(),
            chunk,
        })
    }
}

fn statement_line(stmt: &Statement) -> Option<usize> {
    match stmt {
        Statement::VarDecl { line, .. }
        | Statement::FuncDecl { line, .. }
        | Statement::IfStatement { line, .. }
        | Statement::MatchStatement { line, .. }
        | Statement::WhileStatement { line, .. }
        | Statement::ForStatement { line, .. }
        | Statement::ReturnStatement { line, .. }
        | Statement::BreakStatement { line, .. }
        | Statement::ContinueStatement { line, .. }
        | Statement::HapusStatement { line, .. }
        | Statement::TryCatchStatement { line, .. }
        | Statement::ThrowStatement { line, .. }
        | Statement::AssertStatement { line, .. }
        | Statement::ImportStatement { line, .. }
        | Statement::ExprStatement { line, .. } => Some(*line),
        Statement::Block(block) => Some(block.line),
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}
