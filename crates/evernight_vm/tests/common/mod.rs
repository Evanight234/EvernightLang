#![allow(dead_code)]

use evernight_core::{Lexer, Parser};
use evernight_vm::{Compiler, Value, Vm};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

static FIXTURE_IMPOR: OnceLock<PathBuf> = OnceLock::new();

pub fn eksekusi(sumber: &str) -> Result<Value, String> {
    let tokens = Lexer::new(sumber)
        .scan_tokens()
        .map_err(|e| e.to_string())?;
    let ast = Parser::new(tokens).parse().map_err(|e| e.to_string())?;
    let mut compiler = Compiler::new();
    compiler.compile(&ast).map_err(|e| e.to_string())?;
    let mut vm = Vm::new();
    vm.run(compiler.chunk).map_err(|e| e.to_string())
}

pub fn eksekusi_input(sumber: &str, input: &[&str]) -> Result<Value, String> {
    let tokens = Lexer::new(sumber)
        .scan_tokens()
        .map_err(|e| e.to_string())?;
    let ast = Parser::new(tokens).parse().map_err(|e| e.to_string())?;
    let mut compiler = Compiler::new();
    compiler.compile(&ast).map_err(|e| e.to_string())?;
    let mut vm = Vm::new();
    for baris in input {
        vm.push_input(baris.to_string());
    }
    vm.run(compiler.chunk).map_err(|e| e.to_string())
}

pub fn anggap_false_if_err(program: &str) -> bool {
    eksekusi(program).is_err()
}

pub fn buat_fixture_impor() -> &'static PathBuf {
    FIXTURE_IMPOR.get_or_init(|| {
        let dir = std::env::temp_dir().join(format!("evn_import_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("sub")).unwrap();
        fs::write(
            dir.join("helper_a.eve"),
            "fungsi tambah(a, b) { kembali a + b }\nfungsi kali(a, b) { kembali a * b }\n",
        )
        .unwrap();
        fs::write(
            dir.join("helper_c.eve"),
            "fungsi lipat(x) { kembali kali(x, 2) }\nfungsi kali(a, b) { kembali a * b }\n",
        )
        .unwrap();
        fs::write(
            dir.join("sub/helper_b.eve"),
            "fungsi kurang(a, b) { kembali a - b }\n",
        )
        .unwrap();
        fs::write(dir.join("a.eve"), "impor \"b\"\n").unwrap();
        fs::write(dir.join("b.eve"), "impor \"a\"\n").unwrap();
        dir
    })
}

pub fn eksekusi_dir(dir: &Path, sumber: &str) -> Result<Value, String> {
    let tokens = Lexer::new(sumber)
        .scan_tokens()
        .map_err(|e| e.to_string())?;
    let ast = Parser::new(tokens).parse().map_err(|e| e.to_string())?;
    let mut compiler = Compiler::new();
    compiler.compile(&ast).map_err(|e| e.to_string())?;
    let mut vm = Vm::with_dir(dir.to_path_buf());
    vm.run(compiler.chunk).map_err(|e| e.to_string())
}

pub fn teks_equal(expected: &str, program: &str) -> bool {
    match eksekusi(program) {
        Ok(Value::Teks(s)) => s == expected,
        other => {
            eprintln!("teks_equal({}) -> {:?}", program, other);
            false
        }
    }
}

pub fn angka_equal(expected: f64, program: &str) -> bool {
    match eksekusi(program) {
        Ok(Value::Angka(n)) => (n - expected).abs() < 1e-9,
        other => {
            eprintln!("angka_equal({}) -> {:?}", program, other);
            false
        }
    }
}

pub fn bolean_equal(expected: bool, program: &str) -> bool {
    match eksekusi(program) {
        Ok(Value::Bolean(b)) => b == expected,
        other => {
            eprintln!("bolean_equal({}) -> {:?}", program, other);
            false
        }
    }
}

pub fn cek_daftar_teks(expected: &[&str], program: &str) -> bool {
    match eksekusi(program) {
        Ok(Value::Daftar(d)) => {
            let v: Vec<String> = d.borrow().iter().map(|x| x.to_string()).collect();
            let want: Vec<String> = expected.iter().map(|s| s.to_string()).collect();
            if v != want {
                eprintln!(
                    "cek_daftar_teks({}) -> got {:?}, want {:?}",
                    program, v, want
                );
            }
            v == want
        }
        other => {
            eprintln!("cek_daftar_teks({}) -> {:?}", program, other);
            false
        }
    }
}
