use crate::ast::*;
use crate::errors::EvernightError;
use crate::token::{Token, TokenType};

const MAX_RECURSION_DEPTH: usize = 64;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    depth: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            depth: 0,
        }
    }

    fn enter_depth(&mut self) -> Result<(), EvernightError> {
        if self.depth >= MAX_RECURSION_DEPTH {
            let line = self.peek().line;
            let col = self.peek().column;
            return Err(EvernightError::bahaya(
                "STACK",
                line,
                col,
                "Batas kedalaman rekursi sintaks tercapai (Stack Overflow)!",
            ));
        }
        self.depth += 1;
        Ok(())
    }

    fn leave_depth(&mut self) {
        self.depth = self.depth.saturating_sub(1);
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, EvernightError> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.statement()?);
        }
        Ok(statements)
    }

    // --- STATEMENTS ---

    fn statement(&mut self) -> Result<Statement, EvernightError> {
        self.enter_depth()?;
        let res = self.statement_inner();
        self.leave_depth();
        res
    }

    fn statement_inner(&mut self) -> Result<Statement, EvernightError> {
        if self.match_token(&TokenType::Variabel) {
            self.var_declaration(false)
        } else if self.match_token(&TokenType::Tetap) {
            self.var_declaration(true)
        } else if self.match_token(&TokenType::Fungsi) {
            self.func_declaration()
        } else if self.match_token(&TokenType::Jika) {
            self.if_statement()
        } else if self.match_token(&TokenType::Cocok) {
            self.match_statement()
        } else if self.match_token(&TokenType::Selama) {
            self.while_statement()
        } else if self.match_token(&TokenType::Untuk) {
            self.for_statement()
        } else if self.match_token(&TokenType::Kembali) {
            self.return_statement()
        } else if self.match_token(&TokenType::Berhenti) {
            let line = self.previous().line;
            Ok(Statement::BreakStatement { line })
        } else if self.match_token(&TokenType::Lanjut) {
            let line = self.previous().line;
            Ok(Statement::ContinueStatement { line })
        } else if self.match_token(&TokenType::Hapus) {
            self.hapus_statement()
        } else if self.match_token(&TokenType::Coba) {
            self.try_catch_statement()
        } else if self.match_token(&TokenType::Lempar) {
            self.throw_statement()
        } else if self.match_token(&TokenType::Pastikan) {
            self.assert_statement()
        } else if self.match_token(&TokenType::Impor) {
            self.import_statement()
        } else if self.check(&TokenType::KurawalBuka) {
            let block = self.block()?;
            Ok(Statement::Block(block))
        } else {
            self.expr_statement()
        }
    }

    fn var_declaration(&mut self, is_const: bool) -> Result<Statement, EvernightError> {
        let line = self.previous().line;
        let col = self.previous().column;

        let name = match self.advance().token_type {
            TokenType::Identifier(name) => name,
            _ => {
                return Err(EvernightError::bahaya(
                    "SYNTAX",
                    line,
                    col,
                    "Diharapkan nama pengenal setelah kata kunci deklarasi!",
                ))
            }
        };

        let init = if self.match_token(&TokenType::Sama) {
            Some(self.expression()?)
        } else {
            if is_const {
                return Err(EvernightError::bahaya(
                    "SYNTAX",
                    line,
                    col,
                    format!("Konstanta 'tetap {}' wajib memiliki nilai awal!", name),
                ));
            }
            None
        };

        Ok(Statement::VarDecl {
            name,
            init,
            is_const,
            line,
        })
    }

    fn func_declaration(&mut self) -> Result<Statement, EvernightError> {
        let line = self.previous().line;
        let col = self.previous().column;

        let name = match self.advance().token_type {
            TokenType::Identifier(name) => name,
            _ => {
                return Err(EvernightError::bahaya(
                    "SYNTAX",
                    line,
                    col,
                    "Diharapkan nama fungsi setelah kata kunci 'fungsi'!",
                ))
            }
        };

        self.consume(
            &TokenType::KurungBuka,
            "Diharapkan '(' setelah nama fungsi!",
        )?;
        let params = self.parameter_list()?;
        self.consume(
            &TokenType::KurungTutup,
            "Diharapkan ')' setelah daftar parameter!",
        )?;

        let body = self.block()?;

        Ok(Statement::FuncDecl {
            name,
            params,
            body,
            line,
        })
    }

    fn hapus_statement(&mut self) -> Result<Statement, EvernightError> {
        let line = self.previous().line;
        if self.check(&TokenType::KurungBuka) {
            let callee = Expr::Identifier {
                name: "hapus".to_string(),
                line,
            };
            self.advance();
            let mut args = Vec::new();
            if !self.check(&TokenType::KurungTutup) {
                loop {
                    args.push(self.expression()?);
                    if !self.match_token(&TokenType::Koma) {
                        break;
                    }
                }
            }
            self.consume(
                &TokenType::KurungTutup,
                "Diharapkan ')' setelah argumen hapus!",
            )?;
            return Ok(Statement::ExprStatement {
                expr: Expr::Call {
                    callee: Box::new(callee),
                    args,
                    line,
                },
                line,
            });
        }
        let target_expr = self.postfix()?;
        match target_expr {
            Expr::Index { .. } | Expr::Property { .. } => Ok(Statement::HapusStatement {
                target: target_expr,
                line,
            }),
            _ => Err(EvernightError::bahaya(
                "SYNTAX",
                line,
                self.previous().column,
                "Target 'hapus' harus berupa indeks daftar (a[i]) atau kunci kamus (k[\"x\"] / k.nama)",
            )),
        }
    }

    fn parameter_list(&mut self) -> Result<Vec<Param>, EvernightError> {
        let mut params = Vec::new();

        if !self.check(&TokenType::KurungTutup) {
            loop {
                let name = match self.advance().token_type {
                    TokenType::Identifier(name) => name,
                    _ => {
                        return Err(EvernightError::bahaya(
                            "SYNTAX",
                            self.previous().line,
                            self.previous().column,
                            "Diharapkan nama parameter!",
                        ))
                    }
                };

                let default = if self.match_token(&TokenType::Sama) {
                    Some(self.expression()?)
                } else {
                    None
                };

                params.push(Param { name, default });

                if !self.match_token(&TokenType::Koma) {
                    break;
                }
            }
        }

        Ok(params)
    }

    fn if_statement(&mut self) -> Result<Statement, EvernightError> {
        let line = self.previous().line;
        let condition = self.expression()?;
        let then_branch = self.block()?;

        let mut elif_branches = Vec::new();
        while self.match_token(&TokenType::LainnyaJika) {
            let elif_cond = self.expression()?;
            let elif_body = self.block()?;
            elif_branches.push((elif_cond, elif_body));
        }

        let else_branch = if self.match_token(&TokenType::Lainnya) {
            Some(self.block()?)
        } else {
            None
        };

        Ok(Statement::IfStatement {
            condition,
            then_branch,
            elif_branches,
            else_branch,
            line,
        })
    }

    fn match_statement(&mut self) -> Result<Statement, EvernightError> {
        let line = self.previous().line;
        let expr = self.expression()?;

        self.consume(&TokenType::KurawalBuka, "Diharapkan '{' pada blok 'cocok'!")?;

        let mut cases = Vec::new();
        let mut wildcard = None;
        let mut default = None;

        while !self.check(&TokenType::KurawalTutup) && !self.is_at_end() {
            if self.match_token(&TokenType::Kasus) {
                if self.match_token(&TokenType::Wildcard) {
                    wildcard = Some(self.block()?);
                } else {
                    let case_val = self.expression()?;
                    let case_body = self.block()?;
                    cases.push((case_val, case_body));
                }
            } else if self.match_token(&TokenType::Bawaan) {
                default = Some(self.block()?);
            } else {
                return Err(EvernightError::bahaya(
                    "SYNTAX",
                    self.peek().line,
                    self.peek().column,
                    "Diharapkan 'kasus', 'kasus _', atau 'bawaan' dalam blok 'cocok'!",
                ));
            }
        }

        self.consume(
            &TokenType::KurawalTutup,
            "Diharapkan '}' penutup blok 'cocok'!",
        )?;

        Ok(Statement::MatchStatement {
            expr,
            cases,
            wildcard,
            default,
            line,
        })
    }

    fn while_statement(&mut self) -> Result<Statement, EvernightError> {
        let line = self.previous().line;
        let condition = self.expression()?;
        let body = self.block()?;
        Ok(Statement::WhileStatement {
            condition,
            body,
            line,
        })
    }

    fn for_statement(&mut self) -> Result<Statement, EvernightError> {
        let line = self.previous().line;
        let col = self.previous().column;

        let var_name = match self.advance().token_type {
            TokenType::Identifier(name) => name,
            _ => {
                return Err(EvernightError::bahaya(
                    "SYNTAX",
                    line,
                    col,
                    "Diharapkan nama variabel loop setelah 'untuk'!",
                ))
            }
        };

        let iter = if self.match_token(&TokenType::Dari) {
            let start = self.expression()?;
            self.consume(
                &TokenType::Sampai,
                "Diharapkan 'sampai' setelah batas awal loop!",
            )?;
            let end = self.expression()?;
            ForIter::Range { start, end }
        } else if self.match_token(&TokenType::Dalam) {
            let col_expr = self.expression()?;
            ForIter::Collection(col_expr)
        } else {
            return Err(EvernightError::bahaya(
                "SYNTAX",
                self.peek().line,
                self.peek().column,
                "Diharapkan 'dari ... sampai ...' atau 'dalam ...' pada loop 'untuk'!",
            ));
        };

        let body = self.block()?;

        Ok(Statement::ForStatement {
            var_name,
            iter,
            body,
            line,
        })
    }

    fn return_statement(&mut self) -> Result<Statement, EvernightError> {
        let line = self.previous().line;
        let value = if !self.check(&TokenType::KurawalTutup)
            && !self.check(&TokenType::TitikKoma)
            && !self.is_at_end()
        {
            Some(self.expression()?)
        } else {
            None
        };

        self.match_token(&TokenType::TitikKoma);
        Ok(Statement::ReturnStatement { value, line })
    }

    fn try_catch_statement(&mut self) -> Result<Statement, EvernightError> {
        let line = self.previous().line;
        let try_block = self.block()?;

        self.consume(
            &TokenType::Tangkap,
            "Diharapkan blok 'tangkap' setelah blok 'coba'!",
        )?;
        self.consume(
            &TokenType::KurungBuka,
            "Diharapkan '(' setelah kata kunci 'tangkap'!",
        )?;

        let catch_var = match self.advance().token_type {
            TokenType::Identifier(name) => name,
            _ => {
                return Err(EvernightError::bahaya(
                    "SYNTAX",
                    self.previous().line,
                    self.previous().column,
                    "Diharapkan nama variabel penampung error dalam 'tangkap(...)!'",
                ))
            }
        };

        self.consume(
            &TokenType::KurungTutup,
            "Diharapkan ')' setelah nama variabel error!",
        )?;
        let catch_block = self.block()?;

        let finally_block = if self.match_token(&TokenType::Akhirnya) {
            Some(self.block()?)
        } else {
            None
        };

        Ok(Statement::TryCatchStatement {
            try_block,
            catch_var,
            catch_block,
            finally_block,
            line,
        })
    }

    fn throw_statement(&mut self) -> Result<Statement, EvernightError> {
        let line = self.previous().line;
        let expr = self.expression()?;
        Ok(Statement::ThrowStatement { expr, line })
    }

    fn assert_statement(&mut self) -> Result<Statement, EvernightError> {
        let line = self.previous().line;
        self.consume(&TokenType::KurungBuka, "Diharapkan '(' setelah 'pastikan'!")?;
        let condition = self.expression()?;

        let message = if self.match_token(&TokenType::Koma) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(
            &TokenType::KurungTutup,
            "Diharapkan ')' penutup 'pastikan(...)'!",
        )?;

        Ok(Statement::AssertStatement {
            condition,
            message,
            line,
        })
    }

    fn import_statement(&mut self) -> Result<Statement, EvernightError> {
        let line = self.previous().line;
        let col = self.previous().column;

        let module = match self.advance().token_type {
            TokenType::Identifier(name) => name,
            TokenType::Teks(name) => name,
            _ => {
                return Err(EvernightError::bahaya(
                    "SYNTAX",
                    line,
                    col,
                    "Diharapkan nama modul setelah kata kunci 'impor'!",
                ))
            }
        };

        let alias = if self.match_token(&TokenType::Sebagai) {
            match self.advance().token_type {
                TokenType::Identifier(name) => Some(name),
                _ => {
                    return Err(EvernightError::bahaya(
                        "SYNTAX",
                        line,
                        col,
                        "Diharapkan nama alias setelah 'sebagai'!",
                    ))
                }
            }
        } else {
            None
        };

        Ok(Statement::ImportStatement {
            module,
            alias,
            items: None,
            line,
        })
    }

    fn expr_statement(&mut self) -> Result<Statement, EvernightError> {
        let expr = self.expression()?;
        self.match_token(&TokenType::TitikKoma);
        let line = self.peek().line;
        Ok(Statement::ExprStatement { expr, line })
    }

    fn block(&mut self) -> Result<Block, EvernightError> {
        let line = self.peek().line;
        self.consume(&TokenType::KurawalBuka, "Diharapkan pembuka blok '{'!")?;

        let mut statements = Vec::new();
        while !self.check(&TokenType::KurawalTutup) && !self.is_at_end() {
            statements.push(self.statement()?);
        }

        self.consume(&TokenType::KurawalTutup, "Diharapkan penutup blok '}'!")?;
        Ok(Block { statements, line })
    }

    // --- EXPRESSIONS (Pratt / Precedence Climbing) ---

    pub fn expression(&mut self) -> Result<Expr, EvernightError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, EvernightError> {
        self.enter_depth()?;
        let res = self.assignment_inner();
        self.leave_depth();
        res
    }

    fn assignment_inner(&mut self) -> Result<Expr, EvernightError> {
        let expr = self.logical_or()?;

        if self.match_token(&TokenType::Sama) {
            let line = self.previous().line;
            let value = self.assignment()?;
            return Ok(Expr::Assign {
                target: Box::new(expr),
                op: AssignOp::Assign,
                value: Box::new(value),
                line,
            });
        } else if self.match_token(&TokenType::TambahSama) {
            let line = self.previous().line;
            let value = self.assignment()?;
            return Ok(Expr::Assign {
                target: Box::new(expr),
                op: AssignOp::TambahAssign,
                value: Box::new(value),
                line,
            });
        } else if self.match_token(&TokenType::KurangSama) {
            let line = self.previous().line;
            let value = self.assignment()?;
            return Ok(Expr::Assign {
                target: Box::new(expr),
                op: AssignOp::KurangAssign,
                value: Box::new(value),
                line,
            });
        } else if self.match_token(&TokenType::BintangSama) {
            let line = self.previous().line;
            let value = self.assignment()?;
            return Ok(Expr::Assign {
                target: Box::new(expr),
                op: AssignOp::KaliAssign,
                value: Box::new(value),
                line,
            });
        } else if self.match_token(&TokenType::GarisMiringSama) {
            let line = self.previous().line;
            let value = self.assignment()?;
            return Ok(Expr::Assign {
                target: Box::new(expr),
                op: AssignOp::BagiAssign,
                value: Box::new(value),
                line,
            });
        }

        Ok(expr)
    }

    fn logical_or(&mut self) -> Result<Expr, EvernightError> {
        let mut expr = self.logical_and()?;

        while self.match_token(&TokenType::Atau) || self.match_token(&TokenType::AtauSimbol) {
            let line = self.previous().line;
            let right = self.logical_and()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::Atau,
                right: Box::new(right),
                line,
            };
        }

        Ok(expr)
    }

    fn logical_and(&mut self) -> Result<Expr, EvernightError> {
        let mut expr = self.equality()?;

        while self.match_token(&TokenType::Dan) || self.match_token(&TokenType::DanSimbol) {
            let line = self.previous().line;
            let right = self.equality()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::Dan,
                right: Box::new(right),
                line,
            };
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, EvernightError> {
        let mut expr = self.relational()?;

        while self.match_token(&TokenType::SamaSama) || self.match_token(&TokenType::SamaDengan) {
            let line = self.previous().line;
            let right = self.relational()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::SamaDengan,
                right: Box::new(right),
                line,
            };
        }

        while self.match_token(&TokenType::SeruSama) {
            let line = self.previous().line;
            let right = self.relational()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::TidakSama,
                right: Box::new(right),
                line,
            };
        }

        Ok(expr)
    }

    fn relational(&mut self) -> Result<Expr, EvernightError> {
        let mut expr = self.addition()?;

        loop {
            if self.match_token(&TokenType::KurangDariSimbol)
                || self.match_token(&TokenType::KurangDari)
            {
                let line = self.previous().line;
                let right = self.addition()?;
                expr = Expr::Binary {
                    left: Box::new(expr),
                    op: BinaryOp::KurangDari,
                    right: Box::new(right),
                    line,
                };
            } else if self.match_token(&TokenType::KurangSamaSimbol) {
                let line = self.previous().line;
                let right = self.addition()?;
                expr = Expr::Binary {
                    left: Box::new(expr),
                    op: BinaryOp::KurangSama,
                    right: Box::new(right),
                    line,
                };
            } else if self.match_token(&TokenType::LebihDariSimbol)
                || self.match_token(&TokenType::LebihDari)
            {
                let line = self.previous().line;
                let right = self.addition()?;
                expr = Expr::Binary {
                    left: Box::new(expr),
                    op: BinaryOp::LebihDari,
                    right: Box::new(right),
                    line,
                };
            } else if self.match_token(&TokenType::LebihSamaSimbol) {
                let line = self.previous().line;
                let right = self.addition()?;
                expr = Expr::Binary {
                    left: Box::new(expr),
                    op: BinaryOp::LebihSama,
                    right: Box::new(right),
                    line,
                };
            } else if self.match_token(&TokenType::Dalam) {
                let line = self.previous().line;
                let right = self.addition()?;
                expr = Expr::Binary {
                    left: Box::new(expr),
                    op: BinaryOp::Dalam,
                    right: Box::new(right),
                    line,
                };
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn addition(&mut self) -> Result<Expr, EvernightError> {
        let mut expr = self.multiplication()?;

        loop {
            if self.match_token(&TokenType::Tambah) {
                let line = self.previous().line;
                let right = self.multiplication()?;
                expr = Expr::Binary {
                    left: Box::new(expr),
                    op: BinaryOp::Tambah,
                    right: Box::new(right),
                    line,
                };
            } else if self.match_token(&TokenType::Kurang) {
                let line = self.previous().line;
                let right = self.multiplication()?;
                expr = Expr::Binary {
                    left: Box::new(expr),
                    op: BinaryOp::Kurang,
                    right: Box::new(right),
                    line,
                };
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn multiplication(&mut self) -> Result<Expr, EvernightError> {
        let mut expr = self.power()?;

        loop {
            if self.match_token(&TokenType::Bintang) {
                let line = self.previous().line;
                let right = self.power()?;
                expr = Expr::Binary {
                    left: Box::new(expr),
                    op: BinaryOp::Kali,
                    right: Box::new(right),
                    line,
                };
            } else if self.match_token(&TokenType::GarisMiring) {
                let line = self.previous().line;
                let right = self.power()?;
                expr = Expr::Binary {
                    left: Box::new(expr),
                    op: BinaryOp::Bagi,
                    right: Box::new(right),
                    line,
                };
            } else if self.match_token(&TokenType::Persen) {
                let line = self.previous().line;
                let right = self.power()?;
                expr = Expr::Binary {
                    left: Box::new(expr),
                    op: BinaryOp::Modulo,
                    right: Box::new(right),
                    line,
                };
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn power(&mut self) -> Result<Expr, EvernightError> {
        self.enter_depth()?;
        let res = self.power_inner();
        self.leave_depth();
        res
    }

    fn power_inner(&mut self) -> Result<Expr, EvernightError> {
        let expr = self.unary()?;

        if self.match_token(&TokenType::BintangBintang) {
            let line = self.previous().line;
            let right = self.power()?; // Right-associative
            return Ok(Expr::Binary {
                left: Box::new(expr),
                op: BinaryOp::Pangkat,
                right: Box::new(right),
                line,
            });
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, EvernightError> {
        self.enter_depth()?;
        let res = self.unary_inner();
        self.leave_depth();
        res
    }

    fn unary_inner(&mut self) -> Result<Expr, EvernightError> {
        if self.match_token(&TokenType::Kurang) {
            let line = self.previous().line;
            let right = self.unary()?;
            return Ok(Expr::Unary {
                op: UnaryOp::Minus,
                expr: Box::new(right),
                line,
            });
        } else if self.match_token(&TokenType::Bukan) || self.match_token(&TokenType::Seru) {
            let line = self.previous().line;
            let right = self.unary()?;
            return Ok(Expr::Unary {
                op: UnaryOp::Bukan,
                expr: Box::new(right),
                line,
            });
        }

        self.postfix()
    }

    fn postfix(&mut self) -> Result<Expr, EvernightError> {
        let mut expr = self.primary()?;

        loop {
            if self.match_token(&TokenType::KurungBuka) {
                let line = self.previous().line;
                let mut args = Vec::new();
                if !self.check(&TokenType::KurungTutup) {
                    loop {
                        args.push(self.expression()?);
                        if !self.match_token(&TokenType::Koma) {
                            break;
                        }
                    }
                }
                self.consume(&TokenType::KurungTutup, "Diharapkan ')' setelah argumen!")?;
                expr = Expr::Call {
                    callee: Box::new(expr),
                    args,
                    line,
                };
            } else if self.match_token(&TokenType::SikuBuka) {
                let line = self.previous().line;
                let index = self.expression()?;

                let mut end = None;
                let mut step = None;

                if self.match_token(&TokenType::TitikDua) {
                    if !self.check(&TokenType::SikuTutup) && !self.check(&TokenType::TitikDua) {
                        end = Some(Box::new(self.expression()?));
                    }
                    if self.match_token(&TokenType::TitikDua) && !self.check(&TokenType::SikuTutup)
                    {
                        step = Some(Box::new(self.expression()?));
                    }
                }

                self.consume(
                    &TokenType::SikuTutup,
                    "Diharapkan ']' penutup pengindeksan!",
                )?;
                expr = Expr::Index {
                    target: Box::new(expr),
                    index: Box::new(index),
                    end,
                    step,
                    line,
                };
            } else if self.match_token(&TokenType::Titik) {
                let line = self.previous().line;
                let name = match self.advance().token_type {
                    TokenType::Identifier(name) => name,
                    _ => {
                        return Err(EvernightError::bahaya(
                            "SYNTAX",
                            line,
                            self.previous().column,
                            "Diharapkan nama properti setelah '.'!",
                        ))
                    }
                };
                expr = Expr::Property {
                    target: Box::new(expr),
                    name,
                    line,
                };
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expr, EvernightError> {
        let token = self.advance();
        let line = token.line;

        match token.token_type {
            TokenType::Angka(n) => Ok(Expr::Literal {
                value: LiteralValue::Angka(n),
                line,
            }),
            TokenType::Teks(s) => Ok(Expr::Literal {
                value: LiteralValue::Teks(s),
                line,
            }),
            TokenType::Benar => Ok(Expr::Literal {
                value: LiteralValue::Bolean(true),
                line,
            }),
            TokenType::Salah => Ok(Expr::Literal {
                value: LiteralValue::Bolean(false),
                line,
            }),
            TokenType::Kosong => Ok(Expr::Literal {
                value: LiteralValue::Kosong,
                line,
            }),
            TokenType::Identifier(name) => Ok(Expr::Identifier { name, line }),
            TokenType::Cetak => Ok(Expr::Identifier {
                name: "cetak".to_string(),
                line,
            }),
            TokenType::Baca => Ok(Expr::Identifier {
                name: "baca".to_string(),
                line,
            }),
            TokenType::Hapus => Ok(Expr::Identifier {
                name: "hapus".to_string(),
                line,
            }),
            TokenType::KurungBuka => {
                let expr = self.expression()?;
                self.consume(&TokenType::KurungTutup, "Diharapkan ')' penutup ekspresi!")?;
                Ok(expr)
            }
            TokenType::SikuBuka => {
                let mut elements = Vec::new();
                if !self.check(&TokenType::SikuTutup) {
                    loop {
                        elements.push(self.expression()?);
                        if !self.match_token(&TokenType::Koma) {
                            break;
                        }
                    }
                }
                self.consume(&TokenType::SikuTutup, "Diharapkan ']' penutup daftar!")?;
                Ok(Expr::ArrayLiteral { elements, line })
            }
            TokenType::KurawalBuka => {
                let mut entries = Vec::new();
                if !self.check(&TokenType::KurawalTutup) {
                    loop {
                        let key = self.expression()?;
                        self.consume(&TokenType::TitikDua, "Diharapkan ':' setelah kunci kamus!")?;
                        let val = self.expression()?;
                        entries.push((key, val));
                        if !self.match_token(&TokenType::Koma) {
                            break;
                        }
                    }
                }
                self.consume(&TokenType::KurawalTutup, "Diharapkan '}' penutup kamus!")?;
                Ok(Expr::DictLiteral { entries, line })
            }
            TokenType::Fungsi => {
                self.consume(
                    &TokenType::KurungBuka,
                    "Diharapkan '(' untuk fungsi anonim!",
                )?;
                let params = self.parameter_list()?;
                self.consume(
                    &TokenType::KurungTutup,
                    "Diharapkan ')' setelah daftar parameter!",
                )?;
                let body = self.block()?;
                Ok(Expr::FuncExpr { params, body, line })
            }
            _ => Err(EvernightError::bahaya(
                "SYNTAX",
                line,
                token.column,
                format!("Token tak terduga: '{}'", token.lexeme),
            )),
        }
    }

    // --- HELPERS ---

    fn match_token(&mut self, token_type: &TokenType) -> bool {
        if self.check(token_type) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            &self.peek().token_type == token_type
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn consume(&mut self, token_type: &TokenType, msg: &str) -> Result<Token, EvernightError> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            Err(EvernightError::bahaya(
                "SYNTAX",
                self.peek().line,
                self.peek().column,
                msg,
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    fn parse(source: &str) -> Result<Vec<Statement>, EvernightError> {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.scan_tokens()?;
        let mut parser = Parser::new(tokens);
        parser.parse()
    }

    #[test]
    fn test_var_declaration_dan_ekspresi() {
        let stmts = parse("variabel x = 10 + 5 * 2").unwrap();
        assert_eq!(stmts.len(), 1);

        match &stmts[0] {
            Statement::VarDecl {
                name,
                init,
                is_const,
                line,
            } => {
                assert_eq!(name, "x");
                assert!(!is_const);
                assert_eq!(*line, 1);

                // 10 + (5 * 2) — perkalian harus menang
                let Expr::Binary {
                    op, left, right, ..
                } = init.as_ref().unwrap()
                else {
                    panic!("init bukan binary");
                };
                assert_eq!(*op, BinaryOp::Tambah);
                assert!(
                    matches!(left.as_ref(), Expr::Literal { value: LiteralValue::Angka(a), .. } if *a == 10.0)
                );
                let Expr::Binary { op, .. } = right.as_ref() else {
                    panic!("right bukan binary");
                };
                assert_eq!(*op, BinaryOp::Kali);
            }
            _ => panic!("bukan VarDecl"),
        }
    }

    #[test]
    fn test_deklarasi_fungsi_faktorial() {
        let source = r#"fungsi faktorial(n) {
            jika n <= 1 {
                kembali 1
            } lainnya {
                kembali n * faktorial(n - 1)
            }
        }"#;
        let stmts = parse(source).unwrap();
        assert_eq!(stmts.len(), 1);

        match &stmts[0] {
            Statement::FuncDecl {
                name, params, body, ..
            } => {
                assert_eq!(name, "faktorial");
                assert_eq!(params.len(), 1);
                assert_eq!(params[0].name, "n");
                let Statement::IfStatement {
                    condition,
                    then_branch,
                    else_branch,
                    ..
                } = &body.statements[0]
                else {
                    panic!("body bukan if");
                };
                assert!(matches!(
                    condition,
                    Expr::Binary {
                        op: BinaryOp::KurangSama,
                        ..
                    }
                ));
                assert!(matches!(
                    &then_branch.statements[0],
                    Statement::ReturnStatement { value: Some(Expr::Literal { value: LiteralValue::Angka(a), .. }), .. } if *a == 1.0
                ));
                assert!(else_branch.is_some());
            }
            _ => panic!("bukan FuncDecl"),
        }
    }

    #[test]
    fn test_pemanggilan_fungsi_dan_properti() {
        let stmts = parse("cetak(pesan.panjang)").unwrap();
        let Statement::ExprStatement { expr, .. } = &stmts[0] else {
            panic!("bukan expr statement");
        };
        match expr {
            Expr::Call { callee, args, .. } => {
                assert!(
                    matches!(callee.as_ref(), Expr::Identifier { name, .. } if name == "cetak")
                );
                assert_eq!(args.len(), 1);
                assert!(matches!(&args[0], Expr::Property { name, .. } if name == "panjang"));
            }
            _ => panic!("bukan Call"),
        }
    }

    #[test]
    fn test_cocok_dengan_wildcard() {
        let source = r#"cocok nilai {
            kasus 1 { cetak("satu") }
            kasus _ { cetak("lainnya") }
        }"#;
        let stmts = parse(source).unwrap();
        let Statement::MatchStatement {
            cases,
            wildcard,
            default,
            ..
        } = &stmts[0]
        else {
            panic!("bukan Match");
        };
        assert_eq!(cases.len(), 1);
        assert!(wildcard.is_some());
        assert!(default.is_none());
    }

    #[test]
    fn test_syntax_error_baris_dan_kode() {
        let err = parse("fungsi f( {").unwrap_err();
        assert!(err.to_string().contains("BAHAYA"));
    }
}
