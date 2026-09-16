use evernight_core::{Lexer, Parser};
use evernight_vm::{Compiler, Value, Vm};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

use crate::printer::format_error;

pub fn start_repl(use_color: bool) {
    let reset = if use_color { "\x1b[0m" } else { "" };
    let bold = if use_color { "\x1b[1m" } else { "" };
    let cyan = if use_color { "\x1b[36m" } else { "" };
    let yellow = if use_color { "\x1b[33m" } else { "" };

    println!("{}EvernightLanguage REPL (v0.1.0){}", bold, reset);
    println!(
        "Ketik kode Evernight atau {} :bantuan {} untuk daftar perintah, {} :keluar {} untuk selesai.\n",
        yellow, reset, yellow, reset
    );

    let mut vm = Vm::new();
    let mut buffer = String::new();

    loop {
        let prompt = if buffer.is_empty() {
            format!("{}eve>{} ", cyan, reset)
        } else {
            format!("{}... {} ", cyan, reset)
        };

        print!("{}", prompt);
        let _ = io::stdout().flush();

        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(0) => {
                // EOF (Ctrl+D / Ctrl+Z)
                println!("\nSampai jumpa!");
                break;
            }
            Ok(_) => {
                let trimmed = line.trim();

                // Handle special commands when buffer is empty
                if buffer.is_empty() && trimmed.starts_with(':') {
                    let parts: Vec<&str> = trimmed.split_whitespace().collect();
                    match parts.first().copied() {
                        Some(":keluar") | Some(":exit") | Some(":quit") | Some(":q") => {
                            println!("Sampai jumpa!");
                            break;
                        }
                        Some(":bantuan") | Some(":help") | Some(":?") => {
                            print_help(use_color);
                            continue;
                        }
                        Some(":muat") | Some(":load") => {
                            if parts.len() < 2 {
                                eprintln!("Penggunaan: :muat <path/ke/berkas.eve>");
                            } else {
                                let filepath = parts[1];
                                load_file_into_vm(&mut vm, filepath, use_color);
                            }
                            continue;
                        }
                        Some(":bersihkan") | Some(":clear") => {
                            print!("\x1b[2J\x1b[1;1H");
                            let _ = io::stdout().flush();
                            continue;
                        }
                        _ => {
                            eprintln!(
                                "Perintah tidak dikenal: '{}'. Ketik :bantuan untuk bantuan.",
                                trimmed
                            );
                            continue;
                        }
                    }
                }

                buffer.push_str(&line);

                if needs_more_input(&buffer) {
                    continue;
                }

                let source = buffer.trim();
                if !source.is_empty() {
                    execute_line(&mut vm, source, use_color);
                }
                buffer.clear();
            }
            Err(err) => {
                eprintln!("Error membaca input: {}", err);
                break;
            }
        }
    }
}

fn needs_more_input(source: &str) -> bool {
    let mut brace_depth: i32 = 0;
    let mut paren_depth: i32 = 0;
    let mut bracket_depth: i32 = 0;
    let mut in_str: Option<char> = None;
    let mut escape = false;

    for ch in source.chars() {
        if let Some(quote) = in_str {
            if escape {
                escape = false;
            } else if ch == '\\' {
                escape = true;
            } else if ch == quote {
                in_str = None;
            }
            continue;
        }

        match ch {
            '"' | '\'' => in_str = Some(ch),
            '{' => brace_depth += 1,
            '}' => brace_depth = (brace_depth - 1).max(0),
            '(' => paren_depth += 1,
            ')' => paren_depth = (paren_depth - 1).max(0),
            '[' => bracket_depth += 1,
            ']' => bracket_depth = (bracket_depth - 1).max(0),
            _ => {}
        }
    }

    in_str.is_some() || brace_depth > 0 || paren_depth > 0 || bracket_depth > 0
}

fn execute_line(vm: &mut Vm, source: &str, use_color: bool) {
    let mut lexer = Lexer::new(source);
    let tokens = match lexer.scan_tokens() {
        Ok(t) => t,
        Err(err) => {
            eprint!("{}", format_error(&err, Some(source), use_color));
            return;
        }
    };

    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(tree) => tree,
        Err(err) => {
            eprint!("{}", format_error(&err, Some(source), use_color));
            return;
        }
    };

    let mut compiler = Compiler::new();
    if let Err(err_str) = compiler.compile(&ast) {
        let err = evernight_core::errors::EvernightError::bahaya("COMPILE", 1, 0, err_str);
        eprint!("{}", format_error(&err, Some(source), use_color));
        return;
    }

    for warning in &compiler.warnings {
        eprint!("{}", format_error(warning, Some(source), use_color));
    }

    match vm.run_incremental(compiler.chunk) {
        Ok(result) => {
            if !matches!(result, Value::Kosong) {
                let bold = if use_color { "\x1b[1;32m" } else { "" };
                let reset = if use_color { "\x1b[0m" } else { "" };
                println!("{}=> {}{}", bold, result, reset);
            }
        }
        Err(err) => {
            eprint!("{}", format_error(&err, Some(source), use_color));
        }
    }
}

fn load_file_into_vm(vm: &mut Vm, filepath: &str, use_color: bool) {
    let content = match fs::read_to_string(filepath) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("BAHAYA [FILE]: Gagal memuat berkas '{}': {}", filepath, e);
            return;
        }
    };

    if let Some(parent) = Path::new(filepath).parent() {
        if !parent.as_os_str().is_empty() {
            *vm = Vm::with_dir(parent);
        }
    }

    execute_line(vm, &content, use_color);
    println!("Berkas '{}' berhasil dimuat.", filepath);
}

fn print_help(use_color: bool) {
    let bold = if use_color { "\x1b[1m" } else { "" };
    let reset = if use_color { "\x1b[0m" } else { "" };
    let yellow = if use_color { "\x1b[33m" } else { "" };

    println!("{}Perintah REPL EvernightLanguage:{}", bold, reset);
    println!(
        "  {}:bantuan, :help, :?{}       Menampilkan bantuan ini",
        yellow, reset
    );
    println!(
        "  {}:keluar, :exit, :q{}        Keluar dari REPL",
        yellow, reset
    );
    println!(
        "  {}:muat <berkas.eve>{}        Mengeksekusi berkas .eve ke sesi REPL",
        yellow, reset
    );
    println!(
        "  {}:bersihkan, :clear{}        Membersihkan layar terminal",
        yellow, reset
    );
    println!();
    println!("Contoh sintaksis:");
    println!("  variabel nama = \"Dunia\"");
    println!("  cetak(\"Halo \", nama)");
    println!("  fungsi kuadrat(x) {{ kembali x * x }}");
    println!("  kuadrat(5)");
}
