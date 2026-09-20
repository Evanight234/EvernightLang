use evernight_core::{Lexer, Parser};
use serde_json::{json, Value};

pub fn diagnose(text: &str) -> Vec<Value> {
    let mut diagnostics = Vec::new();

    let tokens = match Lexer::new(text).scan_tokens() {
        Ok(t) => t,
        Err(e) => {
            diagnostics.push(make_diagnostic(&e));
            return diagnostics;
        }
    };

    match Parser::new(tokens).parse() {
        Ok(_) => {}
        Err(e) => {
            diagnostics.push(make_diagnostic(&e));
        }
    }

    diagnostics
}

fn make_diagnostic(err: &evernight_core::errors::EvernightError) -> Value {
    let severity = match err.level {
        evernight_core::errors::ErrorLevel::Bahaya => 1,
        evernight_core::errors::ErrorLevel::Peringatan => 2,
    };

    let start_line = (err.line.max(1) - 1) as u32;
    let start_col = (err.column.max(1) - 1) as u32;

    json!({
        "range": {
            "start": { "line": start_line, "character": start_col },
            "end": { "line": start_line, "character": start_col + 1 }
        },
        "severity": severity,
        "code": err.code,
        "source": "evernight",
        "message": err.message
    })
}
