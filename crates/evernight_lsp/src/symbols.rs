use evernight_core::{Lexer, Parser};
use evernight_core::ast::Statement;
use serde_json::{json, Value};

pub fn goto_definition(text: &str, line: usize, col: usize, file_uri: &str) -> Option<Value> {
    let current_line = text.lines().nth(line)?;
    let safe_col = col.min(current_line.len());

    let before = &current_line[..safe_col];
    let after = &current_line[safe_col..];
    let word_start = before.rfind(|c: char| !c.is_alphanumeric() && c != '_').map_or(0, |i| i + 1);
    let word_end = after.find(|c: char| !c.is_alphanumeric() && c != '_').unwrap_or(after.len());
    let word = &current_line[word_start..safe_col + word_end];

    if word.is_empty() {
        return None;
    }

    let tokens = Lexer::new(text).scan_tokens().ok()?;
    let ast = Parser::new(tokens).parse().ok()?;

    for stmt in &ast {
        match stmt {
            Statement::FuncDecl { name, line: def_line, .. } if name == word => {
                let pos_line = (*def_line).max(1) as u32 - 1;
                return Some(json!({
                    "uri": file_uri,
                    "range": {
                        "start": { "line": pos_line, "character": 0 },
                        "end": { "line": pos_line, "character": 0 }
                    }
                }));
            }
            Statement::VarDecl { name, line: def_line, .. } if name == word => {
                let pos_line = (*def_line).max(1) as u32 - 1;
                return Some(json!({
                    "uri": file_uri,
                    "range": {
                        "start": { "line": pos_line, "character": 0 },
                        "end": { "line": pos_line, "character": 0 }
                    }
                }));
            }
            _ => {}
        }
    }

    None
}

pub fn document_symbols(text: &str) -> Vec<Value> {
    let tokens = match Lexer::new(text).scan_tokens() {
        Ok(t) => t,
        Err(_) => return Vec::new(),
    };
    let ast = match Parser::new(tokens).parse() {
        Ok(a) => a,
        Err(_) => return Vec::new(),
    };

    let mut symbols = Vec::new();
    for stmt in &ast {
        match stmt {
            Statement::FuncDecl { name, line, params, .. } => {
                let start = (*line).max(1) as u32 - 1;
                let param_names: Vec<&str> = params.iter().map(|p| p.name.as_str()).collect();
                let detail = format!("fungsi({})", param_names.join(", "));
                symbols.push(json!({
                    "name": name,
                    "detail": detail,
                    "kind": 12,
                    "range": {
                        "start": { "line": start, "character": 0 },
                        "end": { "line": start, "character": 50 }
                    },
                    "selectionRange": {
                        "start": { "line": start, "character": 0 },
                        "end": { "line": start, "character": name.len() }
                    }
                }));
            }
            Statement::VarDecl { name, line, is_const, .. } => {
                let start = (*line).max(1) as u32 - 1;
                let (detail, kind) = if *is_const {
                    ("konstanta", 14)
                } else {
                    ("variabel", 13)
                };
                symbols.push(json!({
                    "name": name,
                    "detail": detail,
                    "kind": kind,
                    "range": {
                        "start": { "line": start, "character": 0 },
                        "end": { "line": start, "character": 50 }
                    },
                    "selectionRange": {
                        "start": { "line": start, "character": 0 },
                        "end": { "line": start, "character": name.len() }
                    }
                }));
            }
            _ => {}
        }
    }

    symbols
}
