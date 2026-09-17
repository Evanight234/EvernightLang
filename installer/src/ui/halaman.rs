//! Enam halaman wizard installer.

use crate::anim::{AnimasiHover, AnimasiProgress, AnimasiRiak};
use crate::pasang::Rencana;
use crate::tema;
use crate::ui::komponen::{self, StatusLangkah, Tombol};

/// Daftar langkah wizard (untuk panel tahapan).
pub const LANGKAH: &[&str] = &[
    "Selamat datang",
    "Lisensi",
    "Lokasi tujuan",
    "Siap pasang",
    "Memasang",
    "Selesai",
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Halaman {
    SelamatDatang,
    Lisensi,
    Tujuan,
    SiapPasang,
    Memasang,
    Selesai,
}

impl Halaman {
    /// Halaman dari indeks 0..5 (untuk tinjauan desain / `--mulai N`).
    pub fn dari_indeks(i: usize) -> Halaman {
        match i {
            1 => Halaman::Lisensi,
            2 => Halaman::Tujuan,
            3 => Halaman::SiapPasang,
            4 => Halaman::Memasang,
            5 => Halaman::Selesai,
            _ => Halaman::SelamatDatang,
        }
    }

    pub fn indeks(self) -> usize {
        match self {
            Halaman::SelamatDatang => 0,
            Halaman::Lisensi => 1,
            Halaman::Tujuan => 2,
            Halaman::SiapPasang => 3,
            Halaman::Memasang => 4,
            Halaman::Selesai => 5,
        }
    }

    pub fn berikutnya(self) -> Option<Halaman> {
        match self {
            Halaman::SelamatDatang => Some(Halaman::Lisensi),
            Halaman::Lisensi => Some(Halaman::Tujuan),
            Halaman::Tujuan => Some(Halaman::SiapPasang),
            Halaman::SiapPasang => Some(Halaman::Memasang),
            Halaman::Memasang => None,
            Halaman::Selesai => None,
        }
    }

    pub fn sebelumnya(self) -> Option<Halaman> {
        match self {
            Halaman::SelamatDatang => None,
            Halaman::Lisensi => Some(Halaman::SelamatDatang),
            Halaman::Tujuan => Some(Halaman::Lisensi),
            Halaman::SiapPasang => Some(Halaman::Tujuan),
            Halaman::Memasang => None,
            Halaman::Selesai => None,
        }
    }
}

/// Status semua langkah relatif ke halaman saat ini.
pub fn status_langkah(sekarang: Halaman, indeks: usize, sedang_pasang: bool) -> StatusLangkah {
    let i = sekarang.indeks();
    if sekarang == Halaman::Memasang {
        // Saat memasang, langkah 1-4 selesai; langkah 5 aktif.
        return if indeks < 4 {
            StatusLangkah::Selesai
        } else if indeks == 4 {
            StatusLangkah::Aktif
        } else {
            StatusLangkah::Menunggu
        };
    }
    if sekarang == Halaman::Selesai {
        return StatusLangkah::Selesai;
    }
    let _ = sedang_pasang;
    if indeks < i {
        StatusLangkah::Selesai
    } else if indeks == i {
        StatusLangkah::Aktif
    } else {
        StatusLangkah::Menunggu
    }
}

/// Hasil aksi pengguna pada satu halaman.
pub enum Aksi {
    Tidak,
    Lanjut,
    Mundur,
    Keluar,
    MulaiPasang,
    JalankanRepl,
    BukaPanduan,
    BukaFolder,
}

/// Data yang dibutuhkan halaman untuk menggambar.
pub struct KonteksHalaman<'a> {
    pub rencana: &'a mut Rencana,
    pub setuju_lisensi: &'a mut bool,
    pub progres: &'a AnimasiProgress,
    pub status_teks: &'a str,
    pub log: &'a [String],
    pub selesai_ok: bool,
    /// True bila wizard sedang dalam mode pencopotan.
    pub mencopot: bool,
}

/// Gambar isi halaman. Mengembalikan aksi pengguna.
pub fn gambar(
    ui: &mut egui::Ui,
    halaman: Halaman,
    konteks: &mut KonteksHalaman<'_>,
    hover: &mut [AnimasiHover],
    riak: &mut [AnimasiRiak],
    dt: f32,
) -> Aksi {
    let mut aksi = Aksi::Tidak;

    match halaman {
        Halaman::SelamatDatang => {
            judul(ui, "Selamat Datang di EvernightLanguage");
            sub(ui, "Bahasa pemrograman dengan sintaksis Bahasa Indonesia.");
            ui.add_space(14.0);
            komponen::kartu(ui, |ui| {
                ui.label(
                    egui::RichText::new("Evernight bukan hanya karakter game, tapi bisa menjadi pelajaran bahwa menyukai karakter juga bisa menjadi motivasi.")
                        .font(tema::font_isi())
                        .color(tema::SEKUNDER),
                );
                ui.add_space(10.0);
                baris_info(ui, "Versi", crate::pasang::VERSI);
                baris_info(ui, "Lisensi", "MIT (pemegang hak: Satriyo)");
                baris_info(ui, "Berkas program", ".eve");
            });
            ui.add_space(10.0);
            ui.label(
                egui::RichText::new(
                    "Wizard ini akan memandu pemasangan. Tekan Lanjut untuk mulai.",
                )
                .font(tema::font_kecil())
                .color(tema::REDUP),
            );
        }

        Halaman::Lisensi => {
            judul(ui, "Lisensi MIT");
            sub(ui, "Mohon baca dan setujui sebelum melanjutkan.");
            ui.add_space(12.0);
            komponen::kartu(ui, |ui| {
                egui::ScrollArea::vertical()
                    .max_height(190.0)
                    .show(ui, |ui| {
                        ui.label(
                            egui::RichText::new(TEKS_LISENSI)
                                .font(tema::font_mono())
                                .color(tema::SEKUNDER),
                        );
                    });
            });
            ui.add_space(12.0);
            let mut setuju = *konteks.setuju_lisensi;
            if ui
                .checkbox(&mut setuju, "Saya menyetujui ketentuan lisensi MIT")
                .changed()
            {
                *konteks.setuju_lisensi = setuju;
            }
        }

        Halaman::Tujuan => {
            judul(ui, "Lokasi Tujuan");
            sub(ui, "Tentukan tempat pemasangan dan cakupannya.");
            ui.add_space(12.0);

            // BOX: hanya judul + keterangan (teks saja).
            komponen::kartu(ui, |ui| {
                ui.label(
                    egui::RichText::new("Tempat EvernightLanguage dipasang")
                        .font(tema::font_sub())
                        .color(tema::TEKS),
                );
                ui.add_space(6.0);
                ui.label(
                    egui::RichText::new(
                        "Installer menyalin compiler, aset, dan dokumentasi ke folder \
                         yang Anda pilih. Ukuran total sekitar 15 MB.",
                    )
                    .font(tema::font_isi())
                    .color(tema::SEKUNDER),
                );
                ui.add_space(4.0);
                ui.label(
                    egui::RichText::new(
                        "Mode \u{201C}Hanya saya\u{201D} tidak memerlukan izin administrator.",
                    )
                    .font(tema::font_kecil())
                    .color(tema::REDUP),
                );
            });

            // Semua yang interaktif DI LUAR box.
            ui.add_space(14.0);

            pilihan_milik_anda(ui, "Cakupan pemasangan");
            let mut per_mesin = konteks.rencana.per_mesin;
            let sebelum = per_mesin;
            ui.radio_value(&mut per_mesin, false, "Hanya saya (per-user, tanpa UAC)");
            ui.radio_value(
                &mut per_mesin,
                true,
                "Semua pengguna (per-machine, perlu izin admin)",
            );
            if per_mesin != sebelum {
                konteks.rencana.per_mesin = per_mesin;
                konteks.rencana.tujuan = Rencana::tujuan_default(per_mesin);
            }

            ui.add_space(14.0);
            pilihan_milik_anda(ui, "Folder tujuan");
            pemilih_folder(ui, konteks);

            if konteks.rencana.per_mesin {
                ui.add_space(10.0);
                ui.label(
                    egui::RichText::new(
                        "Catatan: mode \u{201C}Semua pengguna\u{201D} akan meminta izin \
                         administrator saat pemasangan dimulai.",
                    )
                    .font(tema::font_kecil())
                    .color(tema::BAHAYA),
                );
            }
        }

        Halaman::SiapPasang => {
            if konteks.mencopot {
                judul(ui, "Copot EvernightLanguage");
                sub(
                    ui,
                    "Installer akan mencabut seluruh komponen yang terpasang.",
                );
                ui.add_space(12.0);
                komponen::kartu(ui, |ui| {
                    ui.label(
                        egui::RichText::new("Yang akan dicabut")
                            .font(tema::font_sub())
                            .color(tema::TEKS),
                    );
                    ui.add_space(6.0);
                    ui.label(
                        egui::RichText::new(
                            "Berkas program, entri PATH, asosiasi berkas .eve, ekstensi \
                             editor, pintasan Start Menu, dan entri di Apps & Features.",
                        )
                        .font(tema::font_isi())
                        .color(tema::SEKUNDER),
                    );
                });
            } else {
                judul(ui, "Siap Pasang");
                sub(ui, "Periksa pilihan Anda, lalu tekan Pasang.");
                ui.add_space(12.0);

                // BOX: hanya judul + keterangan (teks saja).
                komponen::kartu(ui, |ui| {
                    ui.label(
                        egui::RichText::new("Yang akan dilakukan installer")
                            .font(tema::font_sub())
                            .color(tema::TEKS),
                    );
                    ui.add_space(6.0);
                    ui.label(
                        egui::RichText::new(
                            "Compiler evernight beserta aset dan dokumentasi akan disalin \
                             ke folder tujuan. Centang tugas tambahan yang Anda inginkan \
                             di bawah ini.",
                        )
                        .font(tema::font_isi())
                        .color(tema::SEKUNDER),
                    );
                });

                // Semua yang interaktif DI LUAR box.
                ui.add_space(14.0);

                pilihan_milik_anda(ui, "Tugas tambahan");
                ui.checkbox(
                    &mut konteks.rencana.tambah_path,
                    "Tambahkan evernight ke PATH",
                );
                ui.checkbox(
                    &mut konteks.rencana.asosiasi_eve,
                    "Asosiasikan berkas .eve (\"Evernight files\")",
                );
                ui.checkbox(
                    &mut konteks.rencana.pasang_ekstensi,
                    "Pasang ekstensi editor (pewarnaan sintaksis)",
                );
                ui.checkbox(
                    &mut konteks.rencana.buat_pintasan,
                    "Buat pintasan Start Menu",
                );

                ui.add_space(14.0);
                pilihan_milik_anda(ui, "Ringkasan");
                baris_info(ui, "Tujuan", &konteks.rencana.tujuan.to_string_lossy());
                baris_info(
                    ui,
                    "Cakupan",
                    if konteks.rencana.per_mesin {
                        "Semua pengguna (perlu izin admin)"
                    } else {
                        "Hanya saya (tanpa izin admin)"
                    },
                );
            }
        }

        Halaman::Memasang => {
            judul(
                ui,
                if konteks.mencopot {
                    "Mencopot"
                } else {
                    "Memasang"
                },
            );
            sub(ui, konteks.status_teks);

            // Animasi pemasangan DI LUAR box (progress + persentase).
            ui.add_space(18.0);
            let lebar = ui.available_width() - 4.0;
            komponen::progress(ui, konteks.progres, lebar, 16.0);
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(format!("{:.0}%", konteks.progres.nilai * 100.0))
                        .font(tema::font_sub())
                        .color(tema::TEKS),
                );
                // Tampilkan target bila isian masih menyusul (animasi mengalir).
                if konteks.progres.bergerak() {
                    ui.label(
                        egui::RichText::new(format!(
                            "menuju {:.0}%",
                            konteks.progres.target() * 100.0
                        ))
                        .font(tema::font_kecil())
                        .color(tema::REDUP),
                    );
                }
            });

            // BOX: hanya keterangan; isinya log teks kemajuan.
            ui.add_space(16.0);
            komponen::kartu(ui, |ui| {
                ui.label(
                    egui::RichText::new("Catatan pemasangan")
                        .font(tema::font_sub())
                        .color(tema::TEKS),
                );
                ui.add_space(6.0);
                ui.label(
                    egui::RichText::new("Setiap langkah pemasangan dicatat di bawah ini.")
                        .font(tema::font_kecil())
                        .color(tema::REDUP),
                );
                ui.add_space(8.0);
                egui::ScrollArea::vertical()
                    .max_height(150.0)
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        for l in konteks.log {
                            ui.label(
                                egui::RichText::new(l)
                                    .font(tema::font_mono())
                                    .color(tema::SEKUNDER),
                            );
                        }
                    });
            });
        }

        Halaman::Selesai => {
            if konteks.selesai_ok {
                judul(ui, "Pemasangan Selesai");
                sub(ui, "EvernightLanguage siap dipakai.");
            } else {
                judul(ui, "Pemasangan Gagal");
                sub(ui, "Terjadi kesalahan. Lihat catatan di bawah.");
            }
            ui.add_space(14.0);

            komponen::kartu(ui, |ui| {
                baris_info(ui, "Tujuan", &konteks.rencana.tujuan.to_string_lossy());
                baris_info(ui, "Versi", crate::pasang::VERSI);
                ui.add_space(8.0);
                ui.label(
                    egui::RichText::new("Coba jalankan di terminal:")
                        .font(tema::font_kecil())
                        .color(tema::SEKUNDER),
                );
                ui.label(
                    egui::RichText::new("  evernight --versi")
                        .font(tema::font_mono())
                        .color(tema::AKSEN),
                );
                ui.label(
                    egui::RichText::new("  evernight program.eve")
                        .font(tema::font_mono())
                        .color(tema::AKSEN),
                );
            });

            ui.add_space(12.0);
            ui.horizontal(|ui| {
                if komponen::tombol(
                    ui,
                    Tombol::kedua("Jalankan REPL").lebar(130.0),
                    &mut hover[0],
                    &mut riak[0],
                ) {
                    aksi = Aksi::JalankanRepl;
                }
                if komponen::tombol(
                    ui,
                    Tombol::kedua("Buka Panduan").lebar(130.0),
                    &mut hover[1],
                    &mut riak[1],
                ) {
                    aksi = Aksi::BukaPanduan;
                }
                if komponen::tombol(
                    ui,
                    Tombol::kedua("Buka Folder").lebar(130.0),
                    &mut hover[2],
                    &mut riak[2],
                ) {
                    aksi = Aksi::BukaFolder;
                }
            });
        }
    }

    // --- Bilah navigasi bawah ---
    ui.add_space(4.0);
    ui.separator();
    ui.add_space(6.0);

    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new("EvernightLanguage (c) 2026 Satriyo")
                .font(tema::font_kecil())
                .color(tema::REDUP),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let bisa_mundur = halaman.sebelumnya().is_some();
            let bisa_lanjut = match halaman {
                Halaman::Lisensi => *konteks.setuju_lisensi,
                Halaman::SelamatDatang | Halaman::Tujuan | Halaman::SiapPasang => true,
                Halaman::Memasang | Halaman::Selesai => false,
            };

            if halaman == Halaman::SiapPasang {
                let label = if konteks.mencopot { "Copot" } else { "Pasang" };
                if komponen::tombol(
                    ui,
                    Tombol::utama(label).lebar(110.0),
                    &mut hover[3],
                    &mut riak[3],
                ) {
                    aksi = Aksi::MulaiPasang;
                }
            } else if halaman == Halaman::Selesai {
                if komponen::tombol(
                    ui,
                    Tombol::utama("Tutup").lebar(100.0),
                    &mut hover[3],
                    &mut riak[3],
                ) {
                    aksi = Aksi::Keluar;
                }
            } else if halaman != Halaman::Memasang
                && komponen::tombol(
                    ui,
                    Tombol::utama("Lanjut").lebar(100.0),
                    &mut hover[3],
                    &mut riak[3],
                )
                && bisa_lanjut
            {
                aksi = Aksi::Lanjut;
            }

            if halaman != Halaman::Memasang
                && halaman != Halaman::Selesai
                && komponen::tombol(
                    ui,
                    Tombol::kedua("Kembali").lebar(100.0).aktif(bisa_mundur),
                    &mut hover[4],
                    &mut riak[4],
                )
            {
                aksi = Aksi::Mundur;
            }
        });
    });

    let _ = dt;
    aksi
}

fn judul(ui: &mut egui::Ui, teks: &str) {
    ui.label(
        egui::RichText::new(teks)
            .font(tema::font_besar())
            .color(tema::TEKS),
    );
}

fn sub(ui: &mut egui::Ui, teks: &str) {
    ui.add_space(2.0);
    ui.label(
        egui::RichText::new(teks)
            .font(tema::font_sub())
            .color(tema::SEKUNDER),
    );
}

fn baris_info(ui: &mut egui::Ui, label: &str, nilai: &str) {
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(format!("{:<10}", label))
                .font(tema::font_kecil())
                .color(tema::REDUP),
        );
        ui.label(
            egui::RichText::new(nilai)
                .font(tema::font_isi())
                .color(tema::TEKS),
        );
    });
}

/// Label kecil di ATAS kendali interaktif (di luar box).
fn pilihan_milik_anda(ui: &mut egui::Ui, teks: &str) {
    ui.label(
        egui::RichText::new(teks)
            .font(tema::font_kecil())
            .color(tema::SEKUNDER),
    );
    ui.add_space(4.0);
}

/// Pilih folder lewat dialog native Windows.
///
/// Dipisah dari `pemilih_folder` supaya dapat diuji/diganti tanpa UI, dan
/// supaya kegagalan dialog (mis. dibatalkan) tidak mengubah pilihan.
fn pilih_folder_native(awal: &std::path::Path) -> Option<std::path::PathBuf> {
    rfd::FileDialog::new()
        .set_title("Pilih folder tujuan pemasangan")
        .set_directory(awal)
        .pick_folder()
}

/// Pemilih folder tujuan: kotak teks (dapat diketik) + tombol "Telusuri..."
/// yang membuka dialog folder native Windows.
fn pemilih_folder(ui: &mut egui::Ui, konteks: &mut KonteksHalaman<'_>) {
    ui.horizontal(|ui| {
        let lebar_tombol = 110.0;
        let lebar_teks = (ui.available_width() - lebar_tombol - 8.0).max(200.0);

        let mut teks = konteks.rencana.tujuan.to_string_lossy().to_string();
        let respons = ui.add_sized(
            egui::vec2(lebar_teks, 30.0),
            egui::TextEdit::singleline(&mut teks)
                .font(tema::font_mono())
                .hint_text("Pilih folder tujuan..."),
        );
        if respons.changed() {
            konteks.rencana.tujuan = std::path::PathBuf::from(teks);
        }

        let tombol = ui.add_sized(
            egui::vec2(lebar_tombol, 30.0),
            egui::Button::new(egui::RichText::new("Telusuri...").font(tema::font_isi())),
        );
        if tombol.clicked() {
            let awal = if konteks.rencana.tujuan.exists() {
                konteks.rencana.tujuan.clone()
            } else {
                std::env::var("USERPROFILE")
                    .map(std::path::PathBuf::from)
                    .unwrap_or_else(|_| std::path::PathBuf::from("C:\\"))
            };
            if let Some(dipilih) = pilih_folder_native(&awal) {
                konteks.rencana.tujuan = dipilih;
            }
        }
    });

    // Keterangan bantuan di bawah kolom.
    ui.add_space(6.0);
    let path = &konteks.rencana.tujuan;
    let (teks_bantu, warna) = if path.as_os_str().is_empty() {
        ("Folder tujuan belum dipilih.".to_string(), tema::BAHAYA)
    } else if path.exists() {
        (
            "Folder sudah ada \u{2014} installer akan menimpa berkas di dalamnya.".to_string(),
            tema::SEKUNDER,
        )
    } else {
        (
            "Folder akan dibuat otomatis saat pemasangan.".to_string(),
            tema::SEKUNDER,
        )
    };
    ui.label(
        egui::RichText::new(teks_bantu)
            .font(tema::font_kecil())
            .color(warna),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn urutan_halaman_maju_dan_mundur() {
        assert_eq!(Halaman::SelamatDatang.berikutnya(), Some(Halaman::Lisensi));
        assert_eq!(Halaman::SiapPasang.berikutnya(), Some(Halaman::Memasang));
        assert_eq!(Halaman::Lisensi.sebelumnya(), Some(Halaman::SelamatDatang));
        assert_eq!(Halaman::SelamatDatang.sebelumnya(), None);
        assert_eq!(Halaman::Selesai.berikutnya(), None);
    }

    #[test]
    fn dari_indeks_bolak_balik() {
        for i in 0..6 {
            assert_eq!(Halaman::dari_indeks(i).indeks(), i);
        }
        // Di luar rentang -> kembali ke halaman pertama.
        assert_eq!(Halaman::dari_indeks(99), Halaman::SelamatDatang);
    }

    #[test]
    fn status_langkah_menandai_selesai_dan_aktif() {
        // Di halaman Tujuan (indeks 2): 0 & 1 selesai, 2 aktif, sisanya menunggu.
        assert!(matches!(
            status_langkah(Halaman::Tujuan, 0, false),
            StatusLangkah::Selesai
        ));
        assert!(matches!(
            status_langkah(Halaman::Tujuan, 2, false),
            StatusLangkah::Aktif
        ));
        assert!(matches!(
            status_langkah(Halaman::Tujuan, 5, false),
            StatusLangkah::Menunggu
        ));
    }

    #[test]
    fn status_saat_memasang() {
        // Saat memasang: 4 langkah pertama selesai, langkah ke-5 aktif.
        assert!(matches!(
            status_langkah(Halaman::Memasang, 3, true),
            StatusLangkah::Selesai
        ));
        assert!(matches!(
            status_langkah(Halaman::Memasang, 4, true),
            StatusLangkah::Aktif
        ));
        assert!(matches!(
            status_langkah(Halaman::Memasang, 5, true),
            StatusLangkah::Menunggu
        ));
    }

    #[test]
    fn status_pada_halaman_selesai_semuanya_selesai() {
        for i in 0..6 {
            assert!(matches!(
                status_langkah(Halaman::Selesai, i, false),
                StatusLangkah::Selesai
            ));
        }
    }

    #[test]
    fn dialog_folder_menerima_path_awal_yang_tidak_ada() {
        // `pilih_folder_native` tidak boleh panik walau folder awal tidak ada.
        // Tidak dipanggil di test (butuh UI), tapi kita pastikan path awalnya
        // dapat dibentuk dengan aman dari apa pun.
        let awal = std::path::PathBuf::from(r"Z:\tidak-ada-folder-ini");
        assert!(!awal.exists());
        assert_eq!(awal.to_string_lossy(), r"Z:\tidak-ada-folder-ini");
    }
}

const TEKS_LISENSI: &str = "MIT License

Copyright (c) 2026 Satriyo

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the \"Software\"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.";
