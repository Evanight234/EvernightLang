use evernight_core::{Lexer, Parser};
use evernight_vm::{Compiler, Vm};

fn jalankan_cek_error(sumber: &str) -> String {
    let mut lexer = Lexer::new(sumber);
    let tokens = match lexer.scan_tokens() {
        Ok(t) => t,
        Err(e) => return e.to_string(),
    };

    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(t) => t,
        Err(e) => return e.to_string(),
    };

    let mut compiler = Compiler::new();
    if let Err(e) = compiler.compile(&ast) {
        return format!("BAHAYA [COMPILE]: {}", e);
    }

    let mut vm = Vm::new();
    match vm.run(compiler.chunk) {
        Ok(_) => "OK".to_string(),
        Err(e) => e.to_string(),
    }
}

fn jalankan_cek_warning(sumber: &str) -> Vec<String> {
    let mut lexer = Lexer::new(sumber);
    let tokens = lexer.scan_tokens().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    let mut compiler = Compiler::new();
    compiler.compile(&ast).unwrap();
    compiler
        .warnings
        .into_iter()
        .map(|w| w.to_string())
        .collect()
}

#[test]
fn test_error_division_by_zero() {
    let err = jalankan_cek_error("variabel x = 10 / 0");
    assert!(err.contains("BAHAYA [DIVISION]"), "Actual: {}", err);
}

#[test]
fn test_error_type_mismatch() {
    let err = jalankan_cek_error("variabel x = \"teks\" - 5");
    assert!(err.contains("BAHAYA [TYPE]"), "Actual: {}", err);
}

#[test]
fn test_error_index_out_of_bounds() {
    let err = jalankan_cek_error("variabel a = [1, 2]\nkembali a[10]");
    assert!(err.contains("BAHAYA [INDEX]"), "Actual: {}", err);
}

#[test]
fn test_error_key_not_found() {
    let err = jalankan_cek_error("variabel k = {\"a\": 1}\nkembali dapatkan(k, \"b\")");
    assert!(err.contains("BAHAYA [KEY]"), "Actual: {}", err);
}

#[test]
fn test_error_call_non_function() {
    let err = jalankan_cek_error("variabel x = 42\nx()");
    assert!(err.contains("BAHAYA [FUNGSI]"), "Actual: {}", err);
}

#[test]
fn test_error_variable_undefined() {
    let err = jalankan_cek_error("cetak(variabel_ghaib)");
    assert!(err.contains("BAHAYA [VARIABLE]"), "Actual: {}", err);
}

#[test]
fn test_error_math_domain() {
    let err_akar = jalankan_cek_error("kembali akar(-4)");
    assert!(err_akar.contains("BAHAYA [MATH]"), "Actual: {}", err_akar);

    let err_log = jalankan_cek_error("kembali log(0)");
    assert!(err_log.contains("BAHAYA [MATH]"), "Actual: {}", err_log);
}

#[test]
fn test_error_jumlah_faktorial_dan_rata() {
    let err_fak = jalankan_cek_error("kembali faktorial(-1)");
    assert!(err_fak.contains("BAHAYA [JUMLAH]"), "Actual: {}", err_fak);

    let err_rata = jalankan_cek_error("kembali rata_rata([])");
    assert!(err_rata.contains("BAHAYA [JUMLAH]"), "Actual: {}", err_rata);
}

#[test]
fn test_error_assert_pastikan() {
    let err = jalankan_cek_error("pastikan(1 == 2, \"Nilai tidak cocok\")");
    assert!(err.contains("BAHAYA [ASSERT]"), "Actual: {}", err);
}

#[test]
fn test_error_runtime_lempar() {
    let err = jalankan_cek_error("lempar \"Galat fatal dari pengguna\"");
    assert!(err.contains("BAHAYA [RUNTIME]"), "Actual: {}", err);
    assert!(err.contains("Galat fatal dari pengguna"), "Actual: {}", err);
}

#[test]
fn test_error_syntax_parser() {
    let err = jalankan_cek_error("fungsi (a, b) { }");
    assert!(err.contains("BAHAYA [SYNTAX]"), "Actual: {}", err);
}

#[test]
fn test_warning_kompilasi_wkvar_dan_wkreach() {
    let warnings = jalankan_cek_warning(
        "fungsi f() {\n  variabel menganggur = 10\n  kembali 1\n  cetak(\"tak terjangkau\")\n}",
    );
    assert!(!warnings.is_empty());
    let all_w = warnings.join("\n");
    assert!(all_w.contains("PERINGATAN [WKVAR]"), "Actual: {}", all_w);
    assert!(all_w.contains("PERINGATAN [WKREACH]"), "Actual: {}", all_w);
}
