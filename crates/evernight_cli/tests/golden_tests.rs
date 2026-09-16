use evernight_core::{Lexer, Parser};
use evernight_vm::{Compiler, Vm};
use std::fs;
use std::path::{Path, PathBuf};

fn normalize(s: &str) -> Vec<String> {
    s.lines()
        .map(|l| l.trim_end().to_string())
        .filter(|l| !l.is_empty())
        .collect()
}

fn jalankan_source(sumber: &str, dir: Option<&Path>) -> Result<Vec<String>, String> {
    let tokens = Lexer::new(sumber)
        .scan_tokens()
        .map_err(|e| e.to_string())?;
    let ast = Parser::new(tokens).parse().map_err(|e| e.to_string())?;
    let mut compiler = Compiler::new();
    compiler.compile(&ast).map_err(|e| e.to_string())?;

    let mut vm = match dir {
        Some(d) => Vm::with_dir(d),
        None => Vm::new(),
    };
    vm.set_capture_output(true);
    vm.run(compiler.chunk).map_err(|e| e.to_string())?;
    Ok(vm.take_captured_output())
}

#[test]
fn test_seluruh_golden_tests() {
    let golden_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("tests")
        .join("golden");

    assert!(
        golden_dir.exists(),
        "Folder golden tidak ditemukan di {:?}",
        golden_dir
    );

    let entries = fs::read_dir(&golden_dir).expect("Gagal membaca folder golden");
    let mut count = 0;

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("eve") {
            let harapan_path = path.with_extension("harapan");
            assert!(
                harapan_path.exists(),
                "Berkas harapan {:?} tidak ditemukan untuk {:?}",
                harapan_path,
                path
            );

            let sumber = fs::read_to_string(&path).unwrap();
            let harapan_str = fs::read_to_string(&harapan_path).unwrap();

            let parent = path.parent();
            let actual_lines = jalankan_source(&sumber, parent).unwrap_or_else(|e| {
                panic!("Eksekusi gagal pada {:?}: {}", path, e);
            });

            let expected_lines = normalize(&harapan_str);
            let actual_norm: Vec<String> = actual_lines
                .into_iter()
                .map(|l| l.trim_end().to_string())
                .filter(|l| !l.is_empty())
                .collect();

            assert_eq!(
                actual_norm,
                expected_lines,
                "Golden test gagal pada berkas {:?}\nGot: {:#?}\nWant: {:#?}",
                path.file_name().unwrap(),
                actual_norm,
                expected_lines
            );
            count += 1;
        }
    }

    assert!(
        count >= 6,
        "Minimal harus ada 6 golden test, tapi hanya dijalankan {}",
        count
    );
}
