use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use std::sync::Mutex;

use serde_json::{json, Value};

mod completions;
mod diagnostics;
mod hover;
mod symbols;

struct LspState {
    documents: Mutex<HashMap<String, String>>,
}

fn main() {
    let state = LspState {
        documents: Mutex::new(HashMap::new()),
    };

    let stdin = io::stdin();
    let mut reader = stdin.lock();

    loop {
        match read_message(&mut reader) {
            Ok(msg) => {
                let response = dispatch(&state, &msg);
                if let Some(resp) = response {
                    let _ = write_response(&resp);
                }
            }
            Err(e) => {
                eprintln!("[evernight-lsp] read error: {}", e);
                break;
            }
        }
    }
}

fn read_message(reader: &mut impl BufRead) -> Result<Value, io::Error> {
    let mut content_length: Option<usize> = None;

    loop {
        let mut line = String::new();
        reader.read_line(&mut line)?;
        if line.is_empty() {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "EOF"));
        }
        let line = line.trim();
        if line.is_empty() {
            break;
        }
        if let Some(val) = line.strip_prefix("Content-Length: ") {
            content_length = val.parse().ok();
        }
    }

    let len = content_length
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "missing Content-Length"))?;

    let mut buf = vec![0u8; len];
    reader.read_exact(&mut buf)?;

    let text = String::from_utf8(buf)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    serde_json::from_str(&text)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

fn write_response(resp: &Value) -> Result<(), io::Error> {
    let body = serde_json::to_string(resp).unwrap();
    let mut stdout = io::stdout();
    write!(stdout, "Content-Length: {}\r\n\r\n", body.len())?;
    stdout.write_all(body.as_bytes())?;
    stdout.flush()
}

fn dispatch(state: &LspState, msg: &Value) -> Option<Value> {
    let method = msg.get("method")?.as_str()?;
    let id = msg.get("id");
    let params = msg.get("params").cloned().unwrap_or(Value::Null);

    let result = match method {
        "initialize" => handle_initialize(&params),
        "initialized" => return None,
        "shutdown" => handle_shutdown(id?),
        "exit" => {
            std::process::exit(0);
        }
        "textDocument/didOpen" => {
            handle_did_open(state, &params);
            return None;
        }
        "textDocument/didChange" => {
            handle_did_change(state, &params);
            return None;
        }
        "textDocument/didClose" => {
            handle_did_close(state, &params);
            return None;
        }
        "textDocument/completion" => handle_completion(state, &params),
        "textDocument/hover" => handle_hover(state, &params),
        "textDocument/definition" => handle_definition(state, &params),
        "textDocument/documentSymbol" => handle_document_symbol(state, &params),
        _ => return None,
    };

    Some(json!({
        "jsonrpc": "2.0",
        "id": id,
        "result": result
    }))
}

fn handle_initialize(_params: &Value) -> Value {
    json!({
        "capabilities": {
            "textDocumentSync": {
                "openClose": true,
                "change": 1
            },
            "completionProvider": {
                "triggerCharacters": [".", "\""]
            },
            "hoverProvider": true,
            "definitionProvider": true,
            "documentSymbolProvider": true
        },
        "serverInfo": {
            "name": "evernight-lsp",
            "version": "0.1.0"
        }
    })
}

fn handle_shutdown(id: &Value) -> Value {
    json!({
        "jsonrpc": "2.0",
        "id": id,
        "result": null
    })
}

fn get_uri(params: &Value) -> String {
    params
        .get("textDocument")
        .and_then(|td| td.get("uri"))
        .and_then(|u| u.as_str())
        .unwrap_or("")
        .to_string()
}

fn handle_did_open(state: &LspState, params: &Value) {
    let uri = get_uri(params);
    let text = params
        .get("textDocument")
        .and_then(|td| td.get("text"))
        .and_then(|t| t.as_str())
        .unwrap_or("")
        .to_string();
    state.documents.lock().unwrap().insert(uri.clone(), text.clone());
    publish_diagnostics(&uri, &text);
}

fn handle_did_change(state: &LspState, params: &Value) {
    let uri = get_uri(params);
    let text = params
        .get("contentChanges")
        .and_then(|cc| cc.as_array())
        .and_then(|arr| arr.last())
        .and_then(|c| c.get("text"))
        .and_then(|t| t.as_str())
        .unwrap_or("")
        .to_string();
    state.documents.lock().unwrap().insert(uri.clone(), text.clone());
    publish_diagnostics(&uri, &text);
}

fn handle_did_close(state: &LspState, params: &Value) {
    let uri = get_uri(params);
    state.documents.lock().unwrap().remove(&uri);
}

fn publish_diagnostics(uri: &str, text: &str) {
    let diags = diagnostics::diagnose(text);
    let notification = json!({
        "jsonrpc": "2.0",
        "method": "textDocument/publishDiagnostics",
        "params": {
            "uri": uri,
            "diagnostics": diags
        }
    });
    let _ = write_response(&notification);
}

fn handle_completion(state: &LspState, params: &Value) -> Value {
    let uri = get_uri(params);
    let docs = state.documents.lock().unwrap();
    let text = docs.get(&uri).cloned().unwrap_or_default();
    drop(docs);

    let line = params
        .get("position")
        .and_then(|p| p.get("line"))
        .and_then(|l| l.as_u64())
        .unwrap_or(0) as usize;
    let col = params
        .get("position")
        .and_then(|p| p.get("character"))
        .and_then(|c| c.as_u64())
        .unwrap_or(0) as usize;

    let items = completions::complete(&text, line, col);
    json!(items)
}

fn handle_hover(state: &LspState, params: &Value) -> Value {
    let uri = get_uri(params);
    let docs = state.documents.lock().unwrap();
    let text = docs.get(&uri).cloned().unwrap_or_default();
    drop(docs);

    let line = params
        .get("position")
        .and_then(|p| p.get("line"))
        .and_then(|l| l.as_u64())
        .unwrap_or(0) as usize;
    let col = params
        .get("position")
        .and_then(|p| p.get("character"))
        .and_then(|c| c.as_u64())
        .unwrap_or(0) as usize;

    match hover::hover(&text, line, col) {
        Some(h) => h,
        None => json!(null),
    }
}

fn handle_definition(state: &LspState, params: &Value) -> Value {
    let uri = get_uri(params);
    let docs = state.documents.lock().unwrap();
    let text = docs.get(&uri).cloned().unwrap_or_default();
    drop(docs);

    let line = params
        .get("position")
        .and_then(|p| p.get("line"))
        .and_then(|l| l.as_u64())
        .unwrap_or(0) as usize;
    let col = params
        .get("position")
        .and_then(|p| p.get("character"))
        .and_then(|c| c.as_u64())
        .unwrap_or(0) as usize;

    match symbols::goto_definition(&text, line, col, &uri) {
        Some(loc) => loc,
        None => json!(null),
    }
}

fn handle_document_symbol(state: &LspState, params: &Value) -> Value {
    let uri = get_uri(params);
    let docs = state.documents.lock().unwrap();
    let text = docs.get(&uri).cloned().unwrap_or_default();
    drop(docs);

    json!(symbols::document_symbols(&text))
}
