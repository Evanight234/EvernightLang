use std::env;
use std::fs;
use std::path::Path;
use std::process;

use evernight_core::{Lexer, Parser};
use evernight_vm::{Compiler, Value, Vm};

mod json;
mod pkg;
mod printer;
mod repl;

use printer::{disassemble_chunk, format_error};

const VERSION: &str = "0.1.0";

fn print_help() {
    println!("EvernightLanguage CLI (v{})", VERSION);
    println!("Bahasa pemrograman modern dengan sintaksis Bahasa Indonesia.\n");
    println!("PENGGUNAAN:");
    println!("    evernight                         Jalankan REPL interaktif");
    println!("    evernight run <berkas.eve> [opsi] Jalankan berkas .eve (subcommand eksplisit)");
    println!("    evernight <berkas.eve> [opsi]     Jalankan berkas .eve (default-run)");
    println!("    evernight <berkas.eve> -- [arg..] Jalankan berkas dengan argumen program\n");
    println!("SUBCOMMAND:");
    println!("    run <berkas.eve>      Jalankan berkas .eve secara eksplisit");
    println!("    format <berkas.eve>   Rapikan format berkas (indentasi & spasi)");
    println!("    lint <berkas.eve>     Periksa gaya & kerapian kode (aturan WK*)");
    println!("    pkg <subperintah>     Kelola proyek: init | jalankan | daftar");
    println!("    system <aksi>         Info instalasi sistem: info\n");
    println!("OPSI FORMAT:");
    println!(
        "    --cek                 Periksa saja; keluar dengan kode 1 bila belum rapi (untuk CI)"
    );
    println!("    --keluar <path>       Tulis hasil ke berkas lain (tidak menimpa aslinya)\n");
    println!("KODE PERINGATAN LINT:");
    println!("    WKHURUF   Nama bukan snake_case        WKIMPOR   Modul diimpor tapi tak dipakai");
    println!("    WKPANJANG Fungsi > 50 baris            WKPARAM   Fungsi > 4 parameter");
    println!("    WKSARANG  Blok kosong                  WKMATI    Kode tak terjangkau");
    println!("    WKMAGIS   Angka literal 'magic'        WKVAR/WKREACH/WKFUNG  (dari compiler)\n");
    println!("OPSI:");
    println!(
        "    --run             Jalankan program pada VM (opsi eksplisit, sama dengan default)"
    );
    println!("    --cek, --check    Periksa sintaksis dan kompilasi tanpa menjalankan VM");
    println!("    --tokens          Tampilkan daftar token hasil lexing");
    println!("    --ast             Tampilkan Pohon Sintaksis Abstrak (AST)");
    println!("    --bytecode        Tampilkan disassembly instruksi bytecode VM");
    println!("    --debug           Trace setiap instruksi bytecode saat eksekusi (ke stderr)");
    println!("    --waktu           Profil: hitung frekuensi opcode + waktu eksekusi");
    println!("    --tanpa-warna     Nonaktifkan output berwarna pada terminal");
    println!("    -v, --versi       Tampilkan versi EvernightLanguage");
    println!("    -h, --bantuan     Tampilkan panduan bantuan ini\n");
    println!("CONTOH:");
    println!("    evernight run program.eve");
    println!("    evernight program.eve");
    println!("    evernight program.eve --cek");
    println!("    evernight program.eve --tokens --ast");
    println!("    evernight program.eve -- arg1 arg2 arg3");
}

fn main() {
    let raw_args: Vec<String> = env::args().collect();

    // Zero args -> start REPL
    if raw_args.len() <= 1 {
        let use_color = env::var("NO_COLOR").is_err();
        repl::start_repl(use_color);
        return;
    }

    // Check for global flags like help and version.
    // Dilewati bila argumen pertama adalah subperintah, agar
    // `evernight pkg --bantuan` menampilkan bantuan pkg (bukan bantuan utama).
    let ada_subperintah = matches!(
        raw_args.get(1).map(|s| s.as_str()),
        Some("run" | "format" | "lint" | "pkg" | "system")
    );
    if !ada_subperintah {
        for arg in raw_args.iter().skip(1) {
            if arg == "-h" || arg == "--bantuan" || arg == "--help" {
                print_help();
                return;
            }
            if arg == "-v" || arg == "--versi" || arg == "--version" {
                println!("EvernightLanguage v{}", VERSION);
                return;
            }
        }
    }

    let mut start_idx = 1;
    let mut explicit_run = false;

    // Handle `evernight run <file>` subcommand
    if raw_args.len() > 1 && raw_args[1] == "run" {
        explicit_run = true;
        start_idx = 2;

        // Check if no file provided after `run`
        if raw_args.len() == 2 || (raw_args.len() > 2 && raw_args[2].starts_with("--")) {
            eprintln!("BAHAYA [ARG]: Subcommand 'run' membutuhkan path berkas .eve!");
            eprintln!("Penggunaan: evernight run <berkas.eve> [opsi]\n");
            print_help();
            process::exit(1);
        }
    }

    // Subcommand `format` — formatter berkas .eve (Fase 6D-3)
    if raw_args.len() > 1 && raw_args[1] == "format" {
        process::exit(perintah_format(&raw_args[2..]));
    }

    // Subcommand `lint` — pemeriksa gaya berkas .eve (Fase 6D-4)
    if raw_args.len() > 1 && raw_args[1] == "lint" {
        process::exit(perintah_lint(&raw_args[2..]));
    }

    // Subcommand `pkg` — package manager minimal (Fase 6D-6)
    if raw_args.len() > 1 && raw_args[1] == "pkg" {
        process::exit(pkg::jalankan(&raw_args[2..]));
    }

    // Subcommand `system` — info instalasi sistem (Fase 6I)
    if raw_args.len() > 1 && raw_args[1] == "system" {
        process::exit(perintah_system(&raw_args[2..]));
    }

    let mut filename: Option<String> = None;
    let mut show_tokens = false;
    let mut show_ast = false;
    let mut show_bytecode = false;
    let mut check_only = false;
    let mut use_color = env::var("NO_COLOR").is_err();
    let mut program_args: Vec<String> = Vec::new();
    let mut debug_trace = false;
    let mut profile_mode = false;

    let mut i = start_idx;
    while i < raw_args.len() {
        let arg = &raw_args[i];
        if arg == "--" {
            // Everything after -- is passed to the script as program arguments
            program_args.extend(raw_args.iter().skip(i + 1).cloned());
            break;
        } else if arg == "--tokens" {
            show_tokens = true;
        } else if arg == "--ast" {
            show_ast = true;
        } else if arg == "--bytecode" {
            show_bytecode = true;
        } else if arg == "--cek" || arg == "--check" {
            check_only = true;
        } else if arg == "--debug" {
            debug_trace = true;
        } else if arg == "--waktu" {
            profile_mode = true;
        } else if arg == "--run" {
            explicit_run = true;
        } else if arg == "--tanpa-warna" || arg == "--no-color" {
            use_color = false;
        } else if arg.starts_with("--") {
            eprintln!(
                "Opsi tidak dikenal: '{}'. Gunakan --bantuan untuk bantuan.",
                arg
            );
            process::exit(1);
        } else if filename.is_none() {
            filename = Some(arg.clone());
        } else {
            eprintln!(
                "Argumen berlebih: '{}'. Gunakan -- sebelum argumen program.",
                arg
            );
            process::exit(1);
        }
        i += 1;
    }

    let filepath = match filename {
        Some(f) => f,
        None => {
            print_help();
            process::exit(1);
        }
    };

    let source = match fs::read_to_string(&filepath) {
        Ok(content) => content,
        Err(e) => {
            let err = evernight_core::errors::EvernightError::bahaya(
                "FILE",
                0,
                0,
                format!("Gagal membaca berkas '{}': {}", filepath, e),
            );
            eprint!("{}", format_error(&err, None, use_color));
            process::exit(1);
        }
    };

    let mut lexer = Lexer::new(&source);
    let tokens = match lexer.scan_tokens() {
        Ok(t) => t,
        Err(err) => {
            eprint!("{}", format_error(&err, Some(&source), use_color));
            process::exit(1);
        }
    };

    if show_tokens {
        println!("=== HASIL TOKENISASI ({}) ===", filepath);
        for token in &tokens {
            println!("{}", token);
        }
    }

    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(tree) => tree,
        Err(err) => {
            eprint!("{}", format_error(&err, Some(&source), use_color));
            process::exit(1);
        }
    };

    if show_ast {
        println!("=== POHON SINTAKSIS ABSTRAK (AST) ({}) ===", filepath);
        for stmt in &ast {
            println!("{:#?}", stmt);
        }
    }

    let mut compiler = Compiler::new();
    if let Err(err_str) = compiler.compile(&ast) {
        let err = evernight_core::errors::EvernightError::bahaya("COMPILE", 1, 0, err_str);
        eprint!("{}", format_error(&err, Some(&source), use_color));
        process::exit(1);
    }

    for warning in &compiler.warnings {
        eprint!("{}", format_error(warning, Some(&source), use_color));
    }

    if show_bytecode {
        println!("{}", disassemble_chunk(&compiler.chunk, &filepath));
    }

    if check_only {
        // Syntax and compilation check completed with no errors
        let green = if use_color { "\x1b[1;32m" } else { "" };
        let reset = if use_color { "\x1b[0m" } else { "" };
        println!(
            "{}Pemeriksaan berhasil: berkas '{}' valid.{}",
            green, filepath, reset
        );
        return;
    }

    // Default-run or explicit run:
    // If only inspection flags (--tokens, --ast, --bytecode) were requested and NOT run, do not run VM.
    // --debug / --waktu selalu menjalankan program (bukan sekadar inspeksi).
    let is_inspection_only = (show_tokens || show_ast || show_bytecode)
        && !explicit_run
        && !debug_trace
        && !profile_mode;
    if is_inspection_only {
        return;
    }

    let mut vm = Vm::new();
    if let Some(parent) = Path::new(&filepath).parent() {
        if !parent.as_os_str().is_empty() {
            vm = Vm::with_dir(parent);
        }
    }
    vm.set_args(program_args);
    vm.debug_trace = debug_trace;
    vm.profile_mode = profile_mode;

    match vm.run(compiler.chunk) {
        Ok(result) => {
            if !matches!(result, Value::Kosong) {
                println!("Hasil: {}", result);
            }
            if profile_mode {
                print_profile(&vm, use_color);
            }
        }
        Err(err) => {
            eprint!("{}", format_error(&err, Some(&source), use_color));
            process::exit(1);
        }
    }
}

/// Laporan profil 6D-2: waktu eksekusi + frekuensi opcode.
fn print_profile(vm: &Vm, use_color: bool) {
    let (ms, total, daftar) = vm.profile_summary();
    let bold = if use_color { "\x1b[1m" } else { "" };
    let cyan = if use_color { "\x1b[36m" } else { "" };
    let reset = if use_color { "\x1b[0m" } else { "" };

    println!();
    println!("{}=== PROFIL EvernightLanguage ==={}", bold, reset);
    println!("Waktu eksekusi : {:.3} ms", ms);
    println!("Total instruksi: {}", total);
    if daftar.is_empty() {
        return;
    }
    println!();
    let lebar = daftar
        .iter()
        .map(|(n, _)| n.len())
        .max()
        .unwrap_or(4)
        .max(4);
    println!(
        "{:<lebar$} {:>10}  {:>6}",
        format!("{}Opcode{}", cyan, reset),
        "Hit",
        "%",
        lebar = lebar
    );
    println!("{}", "-".repeat(lebar + 20));
    for (nama, n) in &daftar {
        let persen = if total > 0 {
            (*n as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        println!(
            "{:<lebar$} {:>10}  {:>5.1}%",
            nama,
            n,
            persen,
            lebar = lebar
        );
    }
}

/// Subcommand `evernight format <berkas>` (Fase 6D-3).
///
/// - tanpa opsi      : rapikan lalu tulis kembali ke berkas
/// - `--cek`         : periksa saja; kode keluar 1 bila belum rapi (untuk CI)
/// - `--keluar PATH` : tulis hasil ke berkas lain
fn perintah_format(args: &[String]) -> i32 {
    let mut berkas: Option<String> = None;
    let mut cek = false;
    let mut keluar: Option<String> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--cek" | "--check" => cek = true,
            "--tanpa-warna" | "--no-color" => {}
            "--keluar" | "--out" | "-o" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("BAHAYA [ARG]: Opsi '--keluar' membutuhkan path tujuan!");
                    return 1;
                }
                keluar = Some(args[i].clone());
            }
            a if a.starts_with("--") => {
                eprintln!(
                    "Opsi tidak dikenal: '{}'. Gunakan --bantuan untuk bantuan.",
                    a
                );
                return 1;
            }
            a => {
                if berkas.is_none() {
                    berkas = Some(a.to_string());
                } else {
                    eprintln!("Argumen berlebih: '{}'.", a);
                    return 1;
                }
            }
        }
        i += 1;
    }

    let path = match berkas {
        Some(p) => p,
        None => {
            eprintln!("BAHAYA [ARG]: Subcommand 'format' membutuhkan path berkas .eve!");
            eprintln!("Penggunaan: evernight format <berkas.eve> [--cek] [--keluar <path>]");
            return 1;
        }
    };

    let sumber = match fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => {
            let err = evernight_core::errors::EvernightError::bahaya(
                "FILE",
                0,
                0,
                format!("Gagal membaca berkas '{}': {}", path, e),
            );
            eprint!("{}", format_error(&err, None, true));
            return 1;
        }
    };

    let hasil = match evernight_fmt::format(&sumber) {
        Ok(h) => h,
        Err(e) => {
            let err = evernight_core::errors::EvernightError::bahaya("FORMAT", 1, 0, e);
            eprint!("{}", format_error(&err, Some(&sumber), true));
            return 1;
        }
    };

    let berubah = hasil != sumber;

    if cek {
        if berubah {
            eprintln!(
                "PERINGATAN [FORMAT]: Berkas '{}' belum rapi. Jalankan 'evernight format {}'.",
                path, path
            );
            return 1;
        }
        println!("Sudah rapi: '{}'", path);
        return 0;
    }

    if let Some(tujuan) = keluar {
        if let Err(e) = fs::write(&tujuan, &hasil) {
            eprintln!("BAHAYA [FILE]: Gagal menulis '{}': {}", tujuan, e);
            return 1;
        }
        println!("Hasil format ditulis ke '{}'.", tujuan);
        return 0;
    }

    if !berubah {
        println!("Sudah rapi: '{}' (tidak ada perubahan).", path);
        return 0;
    }
    if let Err(e) = fs::write(&path, &hasil) {
        eprintln!("BAHAYA [FILE]: Gagal menulis '{}': {}", path, e);
        return 1;
    }
    println!("Diformat: '{}'", path);
    0
}

/// Subcommand `evernight lint <berkas>` (Fase 6D-4).
///
/// Menggabungkan peringatan compiler (WKVAR/WKREACH/WKFUNG) dengan aturan
/// gaya dari crate `evernight_lint`. Kode keluar 1 bila ada peringatan.
fn perintah_lint(args: &[String]) -> i32 {
    let mut berkas: Option<String> = None;
    let mut use_color = env::var("NO_COLOR").is_err();

    for a in args {
        match a.as_str() {
            "--tanpa-warna" | "--no-color" => use_color = false,
            "--bantuan" | "-h" | "--help" => {
                println!("Penggunaan: evernight lint <berkas.eve> [--tanpa-warna]");
                return 0;
            }
            s if s.starts_with("--") => {
                eprintln!(
                    "Opsi tidak dikenal: '{}'. Gunakan --bantuan untuk bantuan.",
                    s
                );
                return 1;
            }
            s => {
                if berkas.is_none() {
                    berkas = Some(s.to_string());
                } else {
                    eprintln!("Argumen berlebih: '{}'.", s);
                    return 1;
                }
            }
        }
    }

    let path = match berkas {
        Some(p) => p,
        None => {
            eprintln!("BAHAYA [ARG]: Subcommand 'lint' membutuhkan path berkas .eve!");
            eprintln!("Penggunaan: evernight lint <berkas.eve>");
            return 1;
        }
    };

    let sumber = match fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => {
            let err = evernight_core::errors::EvernightError::bahaya(
                "FILE",
                0,
                0,
                format!("Gagal membaca berkas '{}': {}", path, e),
            );
            eprint!("{}", format_error(&err, None, use_color));
            return 1;
        }
    };

    // Peringatan compiler (WKVAR/WKREACH/WKFUNG) — bila parse/kompilasi berhasil.
    let mut semua: Vec<evernight_core::errors::EvernightError> = Vec::new();

    if let Ok(tokens) = Lexer::new(&sumber).scan_tokens() {
        if let Ok(program) = Parser::new(tokens).parse() {
            let mut compiler = Compiler::new();
            if compiler.compile(&program).is_ok() {
                semua.extend(compiler.warnings.iter().cloned());
            }
        }
    }

    match evernight_lint::lint(&sumber) {
        Ok(hasil) => semua.extend(hasil.peringatan),
        Err(e) => {
            eprint!("{}", format_error(&e, Some(&sumber), use_color));
            return 1;
        }
    }

    semua.sort_by_key(|p| p.line);

    if semua.is_empty() {
        let hijau = if use_color { "\x1b[32m" } else { "" };
        let reset = if use_color { "\x1b[0m" } else { "" };
        println!(
            "{}Bersih: '{}' tidak memiliki peringatan.{}",
            hijau, path, reset
        );
        return 0;
    }

    for p in &semua {
        eprint!("{}", format_error(p, Some(&sumber), use_color));
    }
    eprintln!("\n{} peringatan pada '{}'.", semua.len(), path);
    1
}

/// Subcommand `system` — info instalasi sistem (Fase 6I).
///
/// `evernight system info`: menampilkan versi terpasang (dari registri,
/// fallback versi biner ini), lokasi instalasi, ukuran exe, dan isi versi
/// (diambil live dari folder `version/` GitHub; fallback bila offline).
fn perintah_system(args: &[String]) -> i32 {
    match args.first().map(|s| s.as_str()) {
        Some("info") => info_sistem(),
        _ => {
            eprintln!("BAHAYA [ARG]: Subcommand 'system' membutuhkan aksi!");
            eprintln!("Penggunaan: evernight system info");
            1
        }
    }
}

fn info_sistem() -> i32 {
    let versi = reg_baca("DisplayVersion").unwrap_or_else(|| VERSION.to_string());
    let lokasi = reg_baca("InstallLocation").unwrap_or_default();

    println!("EvernightLanguage v{}", versi);
    if lokasi.is_empty() {
        println!("Lokasi: (tidak terdaftar di sistem — mungkin dijalankan portabel)");
    } else {
        println!("Lokasi: {}", lokasi);
        let exe = Path::new(&lokasi).join("bin").join("evernight.exe");
        if let Ok(m) = fs::metadata(&exe) {
            println!("Ukuran evernight.exe: {} KB", m.len() / 1024);
        }
    }

    println!("\nIsi versi {}:", versi);
    match unduh_catatan(&versi) {
        Some(t) if !t.is_empty() => println!("{}", t),
        _ => println!("(catatan tidak dapat diambil — offline?)"),
    }
    0
}

/// Baca satu nilai dari kunci uninstall EvernightLanguage di registri.
fn reg_baca(nama: &str) -> Option<String> {
    let keluar = std::process::Command::new("reg")
        .args([
            "query",
            r"HKCU\Software\Microsoft\Windows\CurrentVersion\Uninstall\EvernightLanguage",
            "/v",
            nama,
        ])
        .output()
        .ok()?;
    if !keluar.status.success() {
        return None;
    }
    let teks = String::from_utf8_lossy(&keluar.stdout);
    let baris = teks.lines().find(|l| l.contains(nama))?;
    let nilai = baris.split("REG_SZ").nth(1)?.trim();
    if nilai.is_empty() {
        None
    } else {
        Some(nilai.to_string())
    }
}

/// Unduh `version/catatan-<versi>.txt` dari GitHub. `None` bila gagal.
fn unduh_catatan(versi: &str) -> Option<String> {
    let url = format!(
        "https://raw.githubusercontent.com/Evanight234/EvernightLang/main/version/catatan-{}.txt",
        versi
    );
    let keluar = std::process::Command::new("powershell")
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            &format!(
                "(New-Object System.Net.WebClient).DownloadString('{}')",
                url
            ),
        ])
        .output()
        .ok()?;
    if !keluar.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&keluar.stdout).trim().to_string())
}
