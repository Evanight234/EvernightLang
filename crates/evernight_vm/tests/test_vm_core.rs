mod common;
use common::*;
use evernight_core::{Lexer, Parser};
use evernight_vm::{Compiler, Value, Vm};

#[test]
fn vm_run_incremental() {
    let mut lexer = Lexer::new("variabel x = 10");
    let tokens = lexer.scan_tokens().unwrap();
    let ast = Parser::new(tokens).parse().unwrap();
    let mut compiler = Compiler::new();
    compiler.compile(&ast).unwrap();

    let mut vm = Vm::new();
    let _ = vm.run_incremental(compiler.chunk).unwrap();

    let mut lexer2 = Lexer::new("kembali x + 5");
    let tokens2 = lexer2.scan_tokens().unwrap();
    let ast2 = Parser::new(tokens2).parse().unwrap();
    let mut compiler2 = Compiler::new();
    compiler2.compile(&ast2).unwrap();

    let res = vm.run_incremental(compiler2.chunk).unwrap();
    assert_eq!(res, Value::Angka(15.0));
}

#[test]
fn vm_argumen_builtin() {
    let mut lexer = Lexer::new("kembali argumen()");
    let tokens = lexer.scan_tokens().unwrap();
    let ast = Parser::new(tokens).parse().unwrap();
    let mut compiler = Compiler::new();
    compiler.compile(&ast).unwrap();

    let mut vm = Vm::new();
    vm.set_args(vec!["foo".to_string(), "bar".to_string()]);
    let res = vm.run(compiler.chunk).unwrap();
    match res {
        Value::Daftar(d) => {
            let list = d.borrow();
            assert_eq!(list.len(), 2);
            assert_eq!(list[0], Value::Teks("foo".to_string()));
            assert_eq!(list[1], Value::Teks("bar".to_string()));
        }
        other => panic!("Bukan daftar: {:?}", other),
    }
}

#[test]
fn user_fungsi_mengalahkan_builtin() {
    let prog = "fungsi max(a, b) { kembali a + b }\nkembali max(10, 20)";
    assert!(angka_equal(30.0, prog));
}

#[test]
fn vm_output_capture() {
    let mut lexer = Lexer::new("cetak(\"Baris 1\", 123)\ncetak(\"Baris 2\")\nbersihkan()");
    let tokens = lexer.scan_tokens().unwrap();
    let ast = Parser::new(tokens).parse().unwrap();
    let mut compiler = Compiler::new();
    compiler.compile(&ast).unwrap();

    let mut vm = Vm::new();
    vm.set_capture_output(true);
    let res = vm.run(compiler.chunk).unwrap();
    assert_eq!(res, Value::Kosong);

    let output = vm.take_captured_output();
    assert_eq!(output.len(), 3);
    assert_eq!(output[0], "Baris 1123");
    assert_eq!(output[1], "Baris 2");
    assert_eq!(output[2], "[BERSIHKAN]");
}
