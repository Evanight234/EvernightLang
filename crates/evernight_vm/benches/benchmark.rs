use evernight_core::{Lexer, Parser};
use evernight_vm::{Compiler, Vm};
use std::time::{Duration, Instant};

fn compile_source(source: &str) -> evernight_vm::Chunk {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    let mut compiler = Compiler::new();
    compiler.compile(&ast).unwrap();
    compiler.chunk
}

fn ukur_eksekusi<F: FnMut()>(nama: &str, mut func: F, iterasi: usize) -> (Duration, Duration) {
    // Warm-up
    for _ in 0..10.min(iterasi) {
        func();
    }

    let mut samples = Vec::with_capacity(iterasi);
    for _ in 0..iterasi {
        let start = Instant::now();
        func();
        let elapsed = start.elapsed();
        samples.push(elapsed);
    }

    samples.sort();
    let total: Duration = samples.iter().sum();
    let avg = total / iterasi as u32;
    let median = samples[iterasi / 2];
    let min = samples[0];
    let max = samples[iterasi - 1];

    println!(
        "{nama:<32} | avg: {avg_us:>8.2} us | med: {med_us:>8.2} us | min: {min_us:>8.2} us | max: {max_us:>8.2} us (N={iterasi})",
        nama = nama,
        avg_us = avg.as_nanos() as f64 / 1_000.0,
        med_us = median.as_nanos() as f64 / 1_000.0,
        min_us = min.as_nanos() as f64 / 1_000.0,
        max_us = max.as_nanos() as f64 / 1_000.0,
        iterasi = iterasi
    );

    (avg, median)
}

fn main() {
    println!("=== EVERNIGHTLANGUAGE BENCHMARK BASELINE (Fase 5) ===");
    println!("--------------------------------------------------------------------------------------------------");

    // 1. Pipeline Kompilasi (Lex + Parse + Compile)
    let program_kompilasi = r#"
        fungsi hitung(a, b) {
            variabel hasil = a * 2 + b * 3
            jika hasil > 100 {
                kembali hasil / 2
            } lainnya {
                kembali hasil * 2
            }
        }
        variabel total = 0
        untuk i dari 1 sampai 50 {
            total += hitung(i, i + 1)
        }
    "#;

    ukur_eksekusi(
        "Kompilasi (Lex + Parse + Codegen)",
        || {
            let mut lexer = Lexer::new(program_kompilasi);
            let tokens = lexer.scan_tokens().unwrap();
            let mut parser = Parser::new(tokens);
            let ast = parser.parse().unwrap();
            let mut compiler = Compiler::new();
            let _ = compiler.compile(&ast);
        },
        500,
    );

    // 2. Eksekusi Rekursi (Faktorial 20)
    let prog_faktorial = r#"
        fungsi faktorial(n) {
            jika n <= 1 { kembali 1 }
            kembali n * faktorial(n - 1)
        }
        kembali faktorial(20)
    "#;
    let chunk_faktorial = compile_source(prog_faktorial);
    ukur_eksekusi(
        "Eksekusi Rekursi Faktorial(20)",
        || {
            let mut vm = Vm::new();
            let _ = vm.run(chunk_faktorial.clone());
        },
        1000,
    );

    // 3. Eksekusi Loop Ketat (10.000 iterasi)
    let prog_loop = r#"
        variabel sum = 0
        untuk i dari 1 sampai 10000 {
            sum += i
        }
        kembali sum
    "#;
    let chunk_loop = compile_source(prog_loop);
    ukur_eksekusi(
        "Eksekusi Loop (10k iterasi)",
        || {
            let mut vm = Vm::new();
            let _ = vm.run(chunk_loop.clone());
        },
        100,
    );

    // 4. Operasi List (peta + saring 1.000 elemen)
    let prog_koleksi = r#"
        fungsi kali(x) { kembali x * 2 }
        fungsi genap(x) { kembali x % 2 == 0 }
        variabel list = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        variabel p = peta(list, kali)
        variabel s = saring(p, genap)
        kembali s.panjang
    "#;
    let chunk_koleksi = compile_source(prog_koleksi);
    ukur_eksekusi(
        "Operasi List (peta + saring)",
        || {
            let mut vm = Vm::new();
            let _ = vm.run(chunk_koleksi.clone());
        },
        1000,
    );

    println!("--------------------------------------------------------------------------------------------------");
    println!("Benchmark baseline selesai dieksekusi dengan sukses.");
}
