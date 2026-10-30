//! Subkomando `evernight march` untuk menjalankan Web Notebook interaktif March (.emlbn).
//!
//! Perintah:
//! - `evernight march`                       — jalankan server notebook di folder kerja saat ini
//! - `evernight march <berkas.emlbn>`        — buka berkas notebook tertentu
//! - `evernight march --port <port>`         — tentukan port web server kustom (bawaan: 8888)
//! - `evernight march --tanpa-browser`       — jalankan server tanpa otomatis membuka browser
//! - `evernight march ekspor <berkas.emlbn>` — ekspor konten notebook menjadi skrip `.eve` biasa

pub mod kernel;
pub mod notebook;
pub mod server;

use notebook::MarchNotebook;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

/// Titik masuk perintah `evernight march <args>`.
pub fn jalankan(args: &[String]) -> i32 {
    let mut berkas_target: Option<String> = None;
    let mut folder_kustom: Option<String> = None;
    let mut port: u16 = 8888;
    let mut buka_browser = true;
    let mut mode_ekspor = false;
    let mut target_keluaran_eve: Option<String> = None;

    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        match arg.as_str() {
            "-h" | "--bantuan" | "--help" => {
                cetak_bantuan();
                return 0;
            }
            "ekspor" | "export" => {
                mode_ekspor = true;
            }
            "--ke-eve" => {
                // flag eksplisit untuk mode ekspor
                mode_ekspor = true;
            }
            "-o" | "--keluar" => {
                if i + 1 < args.len() {
                    i += 1;
                    target_keluaran_eve = Some(args[i].clone());
                } else {
                    eprintln!("BAHAYA [ARG]: Opsi '-o' membutuhkan nama berkas luaran.");
                    return 1;
                }
            }
            "--folder" | "--dir" => {
                if i + 1 < args.len() {
                    i += 1;
                    folder_kustom = Some(args[i].clone());
                } else {
                    eprintln!("BAHAYA [ARG]: Opsi '--folder' membutuhkan path direktori.");
                    return 1;
                }
            }
            "--port" => {
                if i + 1 < args.len() {
                    i += 1;
                    match args[i].parse::<u16>() {
                        Ok(p) => port = p,
                        Err(_) => {
                            eprintln!("BAHAYA [ARG]: Nomor port '{}' tidak valid.", args[i]);
                            return 1;
                        }
                    }
                } else {
                    eprintln!("BAHAYA [ARG]: Opsi '--port' membutuhkan angka port (contoh: 8888).");
                    return 1;
                }
            }
            "--tanpa-browser" => {
                buka_browser = false;
            }
            lain if !lain.starts_with('-') => {
                if berkas_target.is_none() {
                    berkas_target = Some(lain.to_string());
                }
            }
            opsi_asing => {
                eprintln!(
                    "BAHAYA [ARG]: Opsi '{}' tidak dikenal untuk 'evernight march'.",
                    opsi_asing
                );
                cetak_bantuan();
                return 1;
            }
        }
        i += 1;
    }

    let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

    // Mode Ekspor: Mengonversi .emlbn ke .eve
    if mode_ekspor {
        let path_str = match berkas_target {
            Some(p) => p,
            None => {
                eprintln!(
                    "BAHAYA [ARG]: Harap tentukan berkas notebook '.emlbn' yang ingin diekspor."
                );
                println!("Contoh: evernight march ekspor analisis.emlbn");
                return 1;
            }
        };

        let path = Path::new(&path_str);
        let notebook = match MarchNotebook::muat_dari_berkas(path) {
            Ok(nb) => nb,
            Err(e) => {
                eprintln!("BAHAYA [FILE]: {}", e);
                return 1;
            }
        };

        let skrip = notebook.ekspor_ke_eve();
        let target_path = target_keluaran_eve.unwrap_or_else(|| {
            let stem = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("ekspor");
            format!("{}.eve", stem)
        });

        match fs::write(&target_path, skrip) {
            Ok(_) => {
                println!(
                    "[OK] Berkas notebook berhasil diekspor ke skrip Evernight: '{}'",
                    target_path
                );
                0
            }
            Err(e) => {
                eprintln!(
                    "BAHAYA [FILE]: Gagal menulis skrip hasil ekspor ke '{}': {}",
                    target_path, e
                );
                1
            }
        }
    } else {
        // Mode Server Web Notebook
        let (notebook_path, working_dir) = if let Some(f) = folder_kustom {
            let dir = PathBuf::from(&f);
            let nb = dir.join("notebook_march.emlbn");
            (nb, dir)
        } else {
            match berkas_target {
                Some(p) => {
                    let path = PathBuf::from(&p);
                    if path.is_dir() {
                        let nb = path.join("notebook_march.emlbn");
                        (nb, path)
                    } else {
                        let mut file_path = path;
                        if file_path.extension().is_none() {
                            file_path.set_extension("emlbn");
                        }
                        let dir = file_path
                            .parent()
                            .map(|p| p.to_path_buf())
                            .unwrap_or_else(|| cwd.clone());
                        (file_path, dir)
                    }
                }
                None => (cwd.join("notebook_march.emlbn"), cwd),
            }
        };

        match server::mulai_server(notebook_path, working_dir, port, buka_browser) {
            Ok(_) => 0,
            Err(e) => {
                eprintln!("BAHAYA [MARCH]: {}", e);
                1
            }
        }
    }
}

fn cetak_bantuan() {
    println!("evernight march — Web Notebook Interaktif Sains Data & ML Evernight (Fase 9H)\n");
    println!("PENGGUNAAN:");
    println!("    evernight march [berkas.emlbn | folder] [OPSI]");
    println!("    evernight march ekspor <berkas.emlbn> [-o target.eve]\n");
    println!("OPSI:");
    println!("    --folder <DIR>       Tentukan direktori folder penyimpanan notebook (.emlbn)");
    println!("    --port <PORT>        Tentukan nomor port server HTTP lokal (bawaan: 8888)");
    println!("    --tanpa-browser      Jalankan server tanpa otomatis membuka browser");
    println!("    ekspor               Konversi sel kode notebook menjadi berkas skrip .eve");
    println!("    -o, --keluar <PATH>  Tentukan nama berkas luaran untuk mode ekspor");
    println!("    -h, --bantuan        Tampilkan panduan bantuan ini\n");
    println!("CONTOH:");
    println!("    evernight march");
    println!("    evernight march --folder D:\\ProyekData");
    println!("    evernight march analisis_data.emlbn");
    println!("    evernight march --port 9000");
    println!("    evernight march ekspor eksplorasi.emlbn -o skrip_utama.eve");
}
