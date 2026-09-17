//! Installer GUI EvernightLanguage (Fase 6F).
//!
//! Satu berkas `Setup.exe` bermerek: wizard 6 halaman, panel maskot 28%,
//! animasi micro-interaction tombol, dan logika pemasangan (PATH, asosiasi
//! `.eve`, ekstensi editor, pintasan, Apps & Features).
//!
//! Penggunaan:
//!   evernight-installer                  Pemasangan (wizard)
//!   evernight-installer --diam           Pemasangan tanpa UI (otomatisasi)
//!   evernight-installer --uninstall      Pencopotan (wizard)
//!   evernight-installer --uninstall --diam  Pencopotan tanpa UI
//!   evernight-installer --mulai N        Buka wizard di halaman N (0..5)

// Tanpa ini, Windows menampilkan jendela console (CMD) hitam di belakang GUI.
// Saat build debug atribut ini TIDAK dipasang, supaya `eprintln!` masih
// terlihat di terminal (berguna untuk mode `--diam`).
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod anim;
mod app;
mod pasang;
mod tema;
mod ui;

use app::{Apl, Mode};

/// Muat berkas yang dibawa installer (di-embed saat kompilasi).
///
/// Bila berkas pendamping belum ada di `aset/`, dipakai byte kosong agar
/// build tetap berhasil (mis. saat aset belum dibundel).
pub fn muat_payload() -> pasang::Payload {
    pasang::Payload {
        evernight_exe: include_bytes!("../aset/payload/evernight.exe"),
        vsix: include_bytes!("../aset/payload/evernight-language-0.1.0.vsix"),
        panduan: include_bytes!("../aset/payload/PANDUAN.txt"),
        icon_ico: include_bytes!("../aset/logo/icon.ico"),
        logo_png: include_bytes!("../aset/logo/logo.png"),
        lisensi: include_bytes!("../aset/payload/LICENSE"),
    }
}

fn main() -> eframe::Result<()> {
    let arg: Vec<String> = std::env::args().collect();
    let ada = |t: &str| arg.iter().any(|a| a == t);

    let diam = ada("--diam") || ada("--silent") || ada("-q");
    let copot = ada("--uninstall") || ada("--copot");

    // `--mulai N` membuka wizard langsung pada halaman N (0..5).
    // Berguna untuk meninjau desain tiap halaman tanpa mengklik.
    let mulai = arg
        .iter()
        .position(|a| a == "--mulai")
        .and_then(|i| arg.get(i + 1))
        .and_then(|s| s.parse::<usize>().ok());

    let mode = match (copot, diam) {
        (true, true) => Mode::Diam,
        (true, false) => Mode::Copot,
        (false, true) => Mode::Diam,
        (false, false) => Mode::Pasang,
    };

    // Mode diam (otomatisasi): tanpa UI.
    if diam {
        // Saat mencopot, kita harus lepas dari folder instalasi dulu.
        // Windows mengunci executable yang sedang berjalan, sehingga folder
        // tidak dapat dihapus selama kita berada di dalamnya. Solusinya:
        // salin diri ke folder temp, lalu jalankan ulang dari sana.
        if copot && !ada("--dari-temp") {
            if let Some(()) = pindah_ke_temp() {
                return Ok(());
            }
        }
        let m = if copot { Mode::Copot } else { Mode::Pasang };
        std::process::exit(app::jalankan_diam(m));
    }

    let judul = if copot {
        "EvernightLanguage — Pencopotan"
    } else {
        "EvernightLanguage Setup"
    };

    let opsi = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([tema::LEBAR_WINDOW, tema::TINGGI_WINDOW])
            .with_min_inner_size([tema::LEBAR_MIN_WINDOW, tema::TINGGI_MIN_WINDOW])
            .with_title(judul)
            .with_icon(ikon_jendela())
            // Bilah judul kustom digambar sendiri agar tampil seperti desain
            // Stitch (ikon + judul + kontrol jendela menyatu dengan tema).
            .with_decorations(false)
            .with_resizable(true),
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        judul,
        opsi,
        Box::new(move |cc| {
            tema::pasang_font(&cc.egui_ctx);
            tema::pasang_gaya(&cc.egui_ctx);
            let mut apl = Apl::baru(mode);
            if let Some(i) = mulai {
                apl.halaman = ui::halaman::Halaman::dari_indeks(i);
            }
            Ok(Box::new(apl))
        }),
    )
}

/// Salin installer ini ke folder temp lalu jalankan ulang dari sana.
///
/// Dipakai saat mencopot: Windows mengunci executable yang sedang berjalan,
/// sehingga folder instalasi tidak dapat dihapus selama `uninstall.exe`
/// dijalankan dari dalamnya. Menjalankan salinan dari temp melepaskan kunci
/// itu, sehingga pencopotan dapat menghapus folder secara penuh.
///
/// Mengembalikan `Some(())` bila proses temp sudah selesai (pemanggil boleh
/// keluar), atau `None` bila gagal (pemanggil lanjut dengan cara biasa).
fn pindah_ke_temp() -> Option<()> {
    let diri = std::env::current_exe().ok()?;
    let temp = std::env::temp_dir().join(format!("evernight-uninstall-{}.exe", std::process::id()));

    std::fs::copy(&diri, &temp).ok()?;

    // Catat folder yang harus dihapus setelah kita keluar.
    let tujuan = pasang::baca_tujuan_terpasang();
    if let Some(t) = &tujuan {
        std::fs::write(
            std::env::temp_dir().join("evernight-hapus-folder.txt"),
            t.to_string_lossy().as_bytes(),
        )
        .ok();
    }

    let status = std::process::Command::new(&temp)
        .args(["--uninstall", "--diam", "--dari-temp"])
        .status()
        .ok()?;

    // Proses temp sudah selesai; sekarang kita (di dalam folder instalasi)
    // dapat menghapus folder sisa. Semua ISI sudah dibersihkan oleh proses
    // temp, jadi folder seharusnya kosong dan bisa langsung dihapus.
    if let Some(t) = tujuan {
        let _ = std::fs::remove_dir(&t);
        // Bila masih ada sisa (jarang), coba hapus menyeluruh.
        let _ = std::fs::remove_dir_all(&t);
    }
    let _ = std::fs::remove_file(std::env::temp_dir().join("evernight-hapus-folder.txt"));
    let _ = std::fs::remove_file(&temp);
    std::process::exit(if status.success() { 0 } else { 1 });
}

/// Ikon jendela dari `icon.ico` (di-embed).
fn ikon_jendela() -> egui::IconData {
    const ICO: &[u8] = include_bytes!("../aset/logo/icon.ico");
    match image::load_from_memory(ICO) {
        Ok(g) => {
            let rgba = g.to_rgba8();
            egui::IconData {
                width: rgba.width(),
                height: rgba.height(),
                rgba: rgba.into_raw(),
            }
        }
        Err(_) => egui::IconData {
            width: 0,
            height: 0,
            rgba: Vec::new(),
        },
    }
}
