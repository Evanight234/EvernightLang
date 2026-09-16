//! Package manager minimal EvernightLanguage (Fase 6D-6).
//!
//! Perintah:
//! - `ever pkg init`      — buat `eve.json` + kerangka `utama.eve`
//! - `ever pkg jalankan`  — jalankan berkas utama proyek
//! - `ever pkg daftar`    — tampilkan daftar berkas `.eve` di proyek
//!
//! Batas v1 (YAGNI): tanpa registry, tanpa resolver dependensi, tanpa lock file.
//! Ini murni orkestrasi proyek lokal.

use std::fs;
use std::path::{Path, PathBuf};

use evernight_core::{Lexer, Parser};
use evernight_vm::{Compiler, Value, Vm};

use crate::json::Json;
use crate::printer::format_error;

const NAMA_MANIFEST: &str = "eve.json";
const NAMA_UTAMA_DEFAULT: &str = "utama.eve";

/// Titik masuk `ever pkg <subperintah>`.
pub fn jalankan(args: &[String]) -> i32 {
    let (sub, sisa) = match args.split_first() {
        Some((s, r)) => (s.as_str(), r),
        None => {
            cetak_bantuan();
            return 1;
        }
    };

    match sub {
        "init" => perintah_init(sisa),
        "jalankan" | "run" => perintah_jalankan(sisa),
        "daftar" | "list" => perintah_daftar(sisa),
        "-h" | "--bantuan" | "--help" => {
            cetak_bantuan();
            0
        }
        lain => {
            eprintln!("BAHAYA [PKG]: Subperintah '{}' tidak dikenal.", lain);
            cetak_bantuan();
            1
        }
    }
}

fn cetak_bantuan() {
    println!("ever pkg — pengelola proyek EvernightLanguage (v1 minimal)\n");
    println!("PENGGUNAAN:");
    println!(
        "    ever pkg init        Buat {} + kerangka {}",
        NAMA_MANIFEST, NAMA_UTAMA_DEFAULT
    );
    println!("    ever pkg jalankan    Jalankan berkas utama proyek");
    println!("    ever pkg daftar      Tampilkan berkas .eve di proyek\n");
    println!("Catatan: `ever pkg ...` sama dengan `evernight pkg ...`.");
}

// ---------------------------------------------------------------------------
// init
// ---------------------------------------------------------------------------

fn perintah_init(args: &[String]) -> i32 {
    let mut nama: Option<String> = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--nama" | "--name" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("BAHAYA [ARG]: Opsi '--nama' membutuhkan nilai.");
                    return 1;
                }
                nama = Some(args[i].clone());
            }
            s if s.starts_with("--") => {
                eprintln!("Opsi tidak dikenal: '{}'.", s);
                return 1;
            }
            s => {
                if nama.is_none() {
                    nama = Some(s.to_string());
                } else {
                    eprintln!("Argumen berlebih: '{}'.", s);
                    return 1;
                }
            }
        }
        i += 1;
    }

    let manifest = Path::new(NAMA_MANIFEST);
    if manifest.exists() {
        eprintln!("BAHAYA [PKG]: '{}' sudah ada di folder ini.", NAMA_MANIFEST);
        return 1;
    }

    let nama_proyek = nama.unwrap_or_else(|| {
        std::env::current_dir()
            .ok()
            .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
            .unwrap_or_else(|| "proyek-evernight".to_string())
    });

    let isi = Json::Objek(vec![
        ("nama".to_string(), Json::Teks(nama_proyek.clone())),
        ("versi".to_string(), Json::Teks("0.1.0".to_string())),
        (
            "utama".to_string(),
            Json::Teks(NAMA_UTAMA_DEFAULT.to_string()),
        ),
        (
            "berkas".to_string(),
            Json::Larik(vec![Json::Teks(NAMA_UTAMA_DEFAULT.to_string())]),
        ),
    ]);

    if let Err(e) = fs::write(NAMA_MANIFEST, isi.tulis_rapi()) {
        eprintln!("BAHAYA [FILE]: Gagal menulis '{}': {}", NAMA_MANIFEST, e);
        return 1;
    }
    println!("Dibuat: {}", NAMA_MANIFEST);

    if Path::new(NAMA_UTAMA_DEFAULT).exists() {
        println!("Sudah ada: {} (dibiarkan)", NAMA_UTAMA_DEFAULT);
    } else {
        let kerangka = format!(
            "# Proyek: {}\n\nfungsi utama() {{\n    cetak(\"Halo dari {}\")\n}}\n\nutama()\n",
            nama_proyek, nama_proyek
        );
        if let Err(e) = fs::write(NAMA_UTAMA_DEFAULT, kerangka) {
            eprintln!(
                "BAHAYA [FILE]: Gagal menulis '{}': {}",
                NAMA_UTAMA_DEFAULT, e
            );
            return 1;
        }
        println!("Dibuat: {}", NAMA_UTAMA_DEFAULT);
    }

    println!("\nJalankan dengan: ever pkg jalankan");
    0
}

// ---------------------------------------------------------------------------
// jalankan
// ---------------------------------------------------------------------------

fn perintah_jalankan(args: &[String]) -> i32 {
    let (berkas, argumen_program, debug, profile, use_color) = pisah_argumen(args);

    let path = match berkas {
        Some(p) => p,
        None => match cari_berkas_utama() {
            Ok(p) => p,
            Err(e) => {
                eprintln!("BAHAYA [PKG]: {}", e);
                return 1;
            }
        },
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

    let tokens = match Lexer::new(&sumber).scan_tokens() {
        Ok(t) => t,
        Err(e) => {
            eprint!("{}", format_error(&e, Some(&sumber), use_color));
            return 1;
        }
    };
    let program = match Parser::new(tokens).parse() {
        Ok(a) => a,
        Err(e) => {
            eprint!("{}", format_error(&e, Some(&sumber), use_color));
            return 1;
        }
    };

    let mut compiler = Compiler::new();
    if let Err(pesan) = compiler.compile(&program) {
        let err = evernight_core::errors::EvernightError::bahaya("COMPILE", 1, 0, pesan);
        eprint!("{}", format_error(&err, Some(&sumber), use_color));
        return 1;
    }
    for w in &compiler.warnings {
        eprint!("{}", format_error(w, Some(&sumber), use_color));
    }

    let mut vm = Vm::new();
    if let Some(induk) = Path::new(&path).parent() {
        if !induk.as_os_str().is_empty() {
            vm = Vm::with_dir(induk);
        }
    }
    vm.set_args(argumen_program);
    vm.debug_trace = debug;
    vm.profile_mode = profile;

    match vm.run(compiler.chunk) {
        Ok(hasil) => {
            if !matches!(hasil, Value::Kosong) {
                println!("Hasil: {}", hasil);
            }
            if profile {
                let (ms, total, _) = vm.profile_summary();
                println!(
                    "\n=== PROFIL ===\nWaktu: {:.3} ms\nTotal instruksi: {}",
                    ms, total
                );
            }
            0
        }
        Err(e) => {
            eprint!("{}", format_error(&e, Some(&sumber), use_color));
            1
        }
    }
}

/// Tentukan berkas utama: dari `eve.json`, atau `utama.eve`, atau satu-satunya `.eve`.
fn cari_berkas_utama() -> Result<String, String> {
    if let Ok(isi) = fs::read_to_string(NAMA_MANIFEST) {
        if let Ok(json) = crate::json::parse(&isi) {
            if let Some(utama) = json.ambil("utama").and_then(|v| v.sebagai_teks()) {
                if Path::new(utama).exists() {
                    return Ok(utama.to_string());
                }
                return Err(format!(
                    "Berkas utama '{}' (dari {}) tidak ditemukan",
                    utama, NAMA_MANIFEST
                ));
            }
        }
    }

    if Path::new(NAMA_UTAMA_DEFAULT).exists() {
        return Ok(NAMA_UTAMA_DEFAULT.to_string());
    }

    let daftar = kumpulkan_berkas_eve(Path::new("."));
    match daftar.len() {
        0 => Err("Tidak ada berkas .eve di folder ini. Jalankan 'ever pkg init' dulu.".to_string()),
        1 => Ok(daftar[0].to_string_lossy().to_string()),
        _ => {
            let nama: Vec<String> = daftar
                .iter()
                .map(|p| p.to_string_lossy().to_string())
                .collect();
            Err(format!(
                "Tidak ada '{}' atau '{}'. Kandidat: {}. Tentukan berkas secara eksplisit.",
                NAMA_MANIFEST,
                NAMA_UTAMA_DEFAULT,
                nama.join(", ")
            ))
        }
    }
}

// ---------------------------------------------------------------------------
// daftar
// ---------------------------------------------------------------------------

fn perintah_daftar(args: &[String]) -> i32 {
    if args
        .iter()
        .any(|a| a.starts_with("--") && a != "--tanpa-warna")
    {
        eprintln!("Opsi tidak dikenal. Penggunaan: ever pkg daftar");
        return 1;
    }

    // Bila ada manifest, pakai daftar berkas dari dalamnya.
    if let Ok(isi) = fs::read_to_string(NAMA_MANIFEST) {
        if let Ok(json) = crate::json::parse(&isi) {
            let nama_proyek = json
                .ambil("nama")
                .and_then(|v| v.sebagai_teks())
                .unwrap_or("(tanpa nama)");
            let versi = json
                .ambil("versi")
                .and_then(|v| v.sebagai_teks())
                .unwrap_or("?");
            println!("Proyek: {} v{}", nama_proyek, versi);

            if let Some(daftar_manifest) = json.ambil("berkas").and_then(|v| v.sebagai_larik()) {
                if !daftar_manifest.is_empty() {
                    println!("\nBerkas terdaftar ({}):", daftar_manifest.len());
                    for item in daftar_manifest {
                        if let Some(nama) = item.sebagai_teks() {
                            let ada = if Path::new(nama).exists() {
                                ""
                            } else {
                                "  [TIDAK ADA]"
                            };
                            println!("  {}{}", nama, ada);
                        }
                    }
                }
            }
            println!();
        }
    }

    let daftar = kumpulkan_berkas_eve(Path::new("."));
    if daftar.is_empty() {
        println!("Tidak ada berkas .eve di folder ini.");
        return 0;
    }

    println!("Semua berkas .eve ({}):", daftar.len());
    let mut total = 0u64;
    for p in &daftar {
        let ukuran = fs::metadata(p).map(|m| m.len()).unwrap_or(0);
        total += ukuran;
        println!("  {:<40} {:>7} B", p.to_string_lossy(), ukuran);
    }
    println!("\nTotal: {} B", total);
    0
}

/// Kumpulkan berkas `.eve` secara rekursif, melewati folder umum yang tak relevan.
fn kumpulkan_berkas_eve(akar: &Path) -> Vec<PathBuf> {
    let mut hasil = Vec::new();
    telusuri(akar, &mut hasil, 0);
    hasil.sort();
    hasil
}

fn telusuri(dir: &Path, hasil: &mut Vec<PathBuf>, kedalaman: usize) {
    if kedalaman > 12 {
        return;
    }
    let isi = match fs::read_dir(dir) {
        Ok(i) => i,
        Err(_) => return,
    };
    for entri in isi.flatten() {
        let path = entri.path();
        let nama = entri.file_name().to_string_lossy().to_string();
        if nama.starts_with('.') || nama == "target" || nama == "node_modules" {
            continue;
        }
        if path.is_dir() {
            telusuri(&path, hasil, kedalaman + 1);
        } else if path.extension().map(|e| e == "eve").unwrap_or(false) {
            hasil.push(path);
        }
    }
}

/// Pisahkan opsi CLI dari berkas & argumen program (setelah `--`).
fn pisah_argumen(args: &[String]) -> (Option<String>, Vec<String>, bool, bool, bool) {
    let mut berkas = None;
    let mut argumen = Vec::new();
    let mut debug = false;
    let mut profile = false;
    let mut use_color = std::env::var("NO_COLOR").is_err();

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--" => {
                argumen.extend(args.iter().skip(i + 1).cloned());
                break;
            }
            "--debug" => debug = true,
            "--waktu" => profile = true,
            "--tanpa-warna" | "--no-color" => use_color = false,
            s if s.starts_with("--") => {
                eprintln!("Opsi tidak dikenal: '{}'.", s);
            }
            s => {
                if berkas.is_none() {
                    berkas = Some(s.to_string());
                } else {
                    argumen.push(s.to_string());
                }
            }
        }
        i += 1;
    }

    (berkas, argumen, debug, profile, use_color)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn telusuri_menemukan_berkas_eve() {
        let dir = std::env::temp_dir().join("eve_pkg_telusuri");
        let _ = fs::remove_dir_all(&dir);
        let _ = fs::create_dir_all(dir.join("sub"));
        fs::write(dir.join("a.eve"), "cetak(1)").unwrap();
        fs::write(dir.join("sub/b.eve"), "cetak(2)").unwrap();
        fs::write(dir.join("abaikan.txt"), "x").unwrap();

        let hasil = kumpulkan_berkas_eve(&dir);
        assert_eq!(hasil.len(), 2, "{:?}", hasil);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn telusuri_melewati_folder_tertentu() {
        let dir = std::env::temp_dir().join("eve_pkg_lewati");
        let _ = fs::remove_dir_all(&dir);
        let _ = fs::create_dir_all(dir.join("target"));
        let _ = fs::create_dir_all(dir.join(".git"));
        fs::write(dir.join("target/x.eve"), "cetak(1)").unwrap();
        fs::write(dir.join(".git/y.eve"), "cetak(1)").unwrap();
        fs::write(dir.join("z.eve"), "cetak(1)").unwrap();

        let hasil = kumpulkan_berkas_eve(&dir);
        assert_eq!(hasil.len(), 1, "{:?}", hasil);
        let _ = fs::remove_dir_all(&dir);
    }
}
