//! State aplikasi installer: routing halaman, panel maskot, dan status pemasangan.

use std::sync::mpsc::{channel, Receiver};
use std::time::Instant;

use crate::anim::{AnimasiHover, AnimasiHalaman, AnimasiProgress, AnimasiRiak};
use crate::pasang::{self, Pesan, Rencana};
use crate::tema;
use crate::ui::halaman::{self, Aksi, Halaman, KonteksHalaman};
use crate::ui::komponen::penanda_langkah;

/// Mode aplikasi: pemasangan atau pencopotan.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Pasang,
    Copot,
    Diam,
}

pub struct Apl {
    pub mulai: Instant,
    pub halaman: Halaman,
    pub mode: Mode,
    pub rencana: Rencana,
    pub setuju_lisensi: bool,
    pub progres: AnimasiProgress,
    pub transisi: AnimasiHalaman,
    pub hover: Vec<AnimasiHover>,
    pub riak: Vec<AnimasiRiak>,
    pub status_teks: String,
    pub log: Vec<String>,
    pub pekerja: Option<Receiver<Pesan>>,
    pub selesai_ok: bool,
    pub maskot: Option<egui::TextureHandle>,
    pub galat_awal: Option<String>,
}

impl Apl {
    pub fn baru(mode: Mode) -> Self {
        let mut rencana = Rencana::default();
        rencana.tujuan = Rencana::tujuan_default(false);

        // Mode pencopotan langsung menuju halaman konfirmasi, karena tidak
        // ada pilihan tujuan/tugas yang perlu ditanyakan.
        let halaman_awal = if mode == Mode::Copot {
            Halaman::SiapPasang
        } else {
            Halaman::SelamatDatang
        };

        Self {
            mulai: Instant::now(),
            halaman: halaman_awal,
            mode,
            rencana,
            setuju_lisensi: mode == Mode::Copot,
            progres: AnimasiProgress::default(),
            transisi: AnimasiHalaman::default(),
            hover: (0..6).map(|_| AnimasiHover::default()).collect(),
            riak: (0..6).map(|_| AnimasiRiak::default()).collect(),
            status_teks: "Menyiapkan...".to_string(),
            log: Vec::new(),
            pekerja: None,
            selesai_ok: false,
            maskot: None,
            galat_awal: None,
        }
    }

    /// Bilah judul kustom: ikon + judul + tombol minimize/close, sekaligus
    /// area untuk menyeret jendela. Dipakai karena dekorasi native dimatikan
    /// agar wizard tampil menyatu dengan tema.
    fn bilah_judul(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let tinggi = tema::TINGGI_JUDUL;
        let lebar = ui.available_width();
        let (rect, respons) = ui.allocate_exact_size(
            egui::vec2(lebar, tinggi),
            egui::Sense::click_and_drag(),
        );

        ui.painter()
            .rect_filled(rect, egui::CornerRadius::ZERO, tema::WELL);

        if respons.dragged() {
            ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
        }
        if respons.double_clicked() {
            let maks = ctx.input(|i| i.viewport().maximized.unwrap_or(false));
            ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(!maks));
        }

        // Ikon aplikasi.
        let ikon_rect = egui::Rect::from_min_size(
            egui::pos2(rect.left() + 14.0, rect.center().y - 8.0),
            egui::vec2(16.0, 16.0),
        );
        ui.painter()
            .rect_filled(ikon_rect, egui::CornerRadius::same(3), tema::AKSEN);
        ui.painter().text(
            ikon_rect.center(),
            egui::Align2::CENTER_CENTER,
            "E",
            tema::font_kecil(),
            tema::ATAS_AKSEN,
        );

        // Judul.
        ui.painter().text(
            egui::pos2(ikon_rect.right() + 10.0, rect.center().y),
            egui::Align2::LEFT_CENTER,
            "EvernightLanguage Setup",
            tema::font_isi(),
            tema::TEKS,
        );

        // Tombol kontrol jendela.
        let ukuran = egui::vec2(38.0, tinggi);
        let tombol_close = egui::Rect::from_min_size(
            egui::pos2(rect.right() - ukuran.x, rect.top()),
            ukuran,
        );
        let tombol_min = egui::Rect::from_min_size(
            egui::pos2(tombol_close.left() - ukuran.x, rect.top()),
            ukuran,
        );

        let pos = ctx.input(|i| i.pointer.hover_pos());
        let di_close = pos.map(|p| tombol_close.contains(p)).unwrap_or(false);
        let di_min = pos.map(|p| tombol_min.contains(p)).unwrap_or(false);

        if di_close {
            ui.painter()
                .rect_filled(tombol_close, egui::CornerRadius::ZERO, tema::BAHAYA);
        }
        if di_min {
            ui.painter()
                .rect_filled(tombol_min, egui::CornerRadius::ZERO, tema::KARTU_HOVER);
        }

        ui.painter().rect_filled(
            egui::Rect::from_center_size(tombol_min.center(), egui::vec2(11.0, 1.0)),
            egui::CornerRadius::ZERO,
            tema::SEKUNDER,
        );

        let c = tombol_close.center();
        let g = 4.5;
        let warna_x = if di_close { tema::TEKS } else { tema::SEKUNDER };
        ui.painter().line_segment(
            [egui::pos2(c.x - g, c.y - g), egui::pos2(c.x + g, c.y + g)],
            egui::Stroke::new(1.4, warna_x),
        );
        ui.painter().line_segment(
            [egui::pos2(c.x + g, c.y - g), egui::pos2(c.x - g, c.y + g)],
            egui::Stroke::new(1.4, warna_x),
        );

        ui.painter().line_segment(
            [
                egui::pos2(rect.left(), rect.bottom()),
                egui::pos2(rect.right(), rect.bottom()),
            ],
            tema::garis_halus(),
        );

        if respons.clicked() {
            if let Some(p) = pos {
                if tombol_close.contains(p) {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                } else if tombol_min.contains(p) {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
                }
            }
        }
    }

    /// Muat gambar maskot sebagai tekstur egui.
    fn muat_maskot(&mut self, ctx: &egui::Context) {
        if self.maskot.is_some() {
            return;
        }
        const DATA: &[u8] = include_bytes!("../aset/maskot.png");
        match image::load_from_memory(DATA) {
            Ok(gambar) => {
                let rgba = gambar.to_rgba8();
                let ukuran = [rgba.width() as usize, rgba.height() as usize];
                let warna = egui::ColorImage::from_rgba_unmultiplied(ukuran, rgba.as_raw());
                self.maskot = Some(ctx.load_texture("maskot", warna, Default::default()));
            }
            Err(e) => {
                self.galat_awal = Some(format!("Gagal memuat maskot: {}", e));
            }
        }
    }

    /// Pindah halaman sambil memicu transisi.
    fn pindah(&mut self, halaman: Halaman) {
        self.halaman = halaman;
        self.transisi.mulai_ulang();
    }

    /// Mulai pemasangan di thread terpisah.
    fn mulai_pasang(&mut self) {
        let (tx, rx) = channel();
        self.pekerja = Some(rx);
        self.log.clear();
        self.status_teks = "Memulai...".to_string();
        self.progres.set_target(0.0);

        let rencana = self.rencana.clone();
        let payload = crate::muat_payload();

        std::thread::spawn(move || {
            let mut lapor = |p: Pesan| {
                let _ = tx.send(p);
            };
            let hasil = pasang::jalankan(&rencana, &payload, &mut lapor);
            let _ = tx.send(Pesan::Selesai(hasil));
        });
    }

    /// Mulai pencopotan di thread terpisah.
    fn mulai_copot(&mut self) {
        let (tx, rx) = channel();
        self.pekerja = Some(rx);
        self.log.clear();
        self.status_teks = "Mencopot...".to_string();

        std::thread::spawn(move || {
            let mut lapor = |p: Pesan| {
                let _ = tx.send(p);
            };
            let hasil = pasang::copot(&mut lapor);
            let _ = tx.send(Pesan::Selesai(hasil));
        });
    }

    fn serap_pesan(&mut self) {
        let mut selesai: Option<Result<(), String>> = None;
        if let Some(rx) = &self.pekerja {
            while let Ok(p) = rx.try_recv() {
                match p {
                    Pesan::Langkah { persen, teks } => {
                        self.progres.set_target(persen);
                        self.status_teks = teks.clone();
                        self.log.push(format!("> {}", teks));
                    }
                    Pesan::Selesai(hasil) => {
                        selesai = Some(hasil);
                    }
                }
            }
        }
        if let Some(hasil) = selesai {
            self.pekerja = None;
            match hasil {
                Ok(()) => {
                    self.progres.set_target(1.0);
                    self.selesai_ok = true;
                    self.status_teks = "Selesai.".to_string();
                    self.log.push("> Selesai".to_string());
                }
                Err(e) => {
                    self.selesai_ok = false;
                    self.status_teks = format!("Gagal: {}", e);
                    self.log.push(format!("! {}", e));
                }
            }
            if self.mode == Mode::Pasang {
                self.pindah(Halaman::Selesai);
            } else {
                std::process::exit(0);
            }
        }
    }

    fn tangani_aksi(&mut self, aksi: Aksi, ctx: &egui::Context) {
        match aksi {
            Aksi::Lanjut => {
                if let Some(h) = self.halaman.berikutnya() {
                    self.pindah(h);
                }
            }
            Aksi::Mundur => {
                if let Some(h) = self.halaman.sebelumnya() {
                    self.pindah(h);
                }
            }
            Aksi::Keluar => {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
            Aksi::MulaiPasang => {
                if self.mode == Mode::Copot {
                    self.pindah(Halaman::Memasang);
                    self.status_teks = "Mencopot EvernightLanguage...".to_string();
                    self.mulai_copot();
                    return;
                }
                if self.rencana.per_mesin && !pasang::adalah_admin() {
                    // Perlu elevasi: minta UAC lalu keluar.
                    let arg: Vec<String> = std::env::args().skip(1).collect();
                    if let Err(e) = pasang::jalankan_ulang_elevasi(&arg) {
                        self.log.push(format!("! Gagal elevasi: {}", e));
                        return;
                    }
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    return;
                }
                self.pindah(Halaman::Memasang);
                self.mulai_pasang();
            }
            Aksi::JalankanRepl => {
                let _ = std::process::Command::new("cmd")
                    .args(["/C", "start", "cmd", "/K", "evernight"])
                    .spawn();
            }
            Aksi::BukaPanduan => {
                let panduan = self.rencana.tujuan.join("docs").join("PANDUAN.txt");
                let _ = std::process::Command::new("cmd")
                    .args(["/C", "start", "", &panduan.to_string_lossy()])
                    .spawn();
            }
            Aksi::BukaFolder => {
                let _ = std::process::Command::new("explorer")
                    .arg(&self.rencana.tujuan)
                    .spawn();
            }
            Aksi::Tidak => {
                // Mode diam: langsung jalankan tanpa UI.
                if self.mode == Mode::Diam && self.pekerja.is_none() {
                    self.mulai_pasang();
                }
            }
        }
    }
}

impl eframe::App for Apl {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.serap_pesan();
        if self.progres.bergerak() || self.pekerja.is_some() {
            ctx.request_repaint_after(std::time::Duration::from_millis(16));
        }
        if self.halaman == Halaman::Memasang && self.pekerja.is_none() && !self.selesai_ok {
            ctx.request_repaint_after(std::time::Duration::from_millis(160));
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        self.muat_maskot(&ctx);

        let dt = ctx.input(|i| i.stable_dt).min(0.1);
        self.progres.perbarui(dt);

        let waktu = self.mulai.elapsed().as_secs_f64();

        egui::Frame::new().fill(tema::LATAR).show(ui, |ui| {
            ui.set_min_size(ui.available_size());
            self.bilah_judul(ui, &ctx);
            let tinggi_total = ui.available_height();
            let lebar_total = ui.available_width();
            let lebar_panel = lebar_total * tema::RASIO_PANEL_MASKOT;

            ui.horizontal_top(|ui| {
                // ---- Panel maskot (28% lebar, penuh atas-bawah) ----
                let (rect_panel, _) = ui.allocate_exact_size(
                    egui::vec2(lebar_panel, tinggi_total),
                    egui::Sense::hover(),
                );
                let pelukis = ui.painter();
                // Panel maskot memakai WELL (lebih gelap dari latar) agar
                // karakter yang terang menonjol, sesuai referensi desain.
                pelukis.rect_filled(rect_panel, egui::CornerRadius::ZERO, tema::WELL);

                if let Some(tekstur) = &self.maskot {
                    let ukuran = tekstur.size_vec2();
                    // Isi tinggi penuh, crop tengah horizontal.
                    let skala = rect_panel.height() / ukuran.y;
                    let lebar_gambar = ukuran.x * skala;
                    let x0 = rect_panel.left() - (lebar_gambar - rect_panel.width()) / 2.0;
                    let tujuan = egui::Rect::from_min_size(
                        egui::pos2(x0, rect_panel.top()),
                        egui::vec2(lebar_gambar, rect_panel.height()),
                    );
                    let uv0 = egui::pos2(
                        ((rect_panel.left() - x0) / lebar_gambar).clamp(0.0, 1.0),
                        0.0,
                    );
                    let uv1 = egui::pos2(
                        ((rect_panel.right() - x0) / lebar_gambar).clamp(0.0, 1.0),
                        1.0,
                    );
                    ui.painter().with_clip_rect(rect_panel).add(
                        egui::Shape::image(
                            tekstur.id(),
                            rect_panel,
                            egui::Rect::from_min_max(uv0, uv1),
                            egui::Color32::WHITE,
                        ),
                    );
                    let _ = tujuan;
                }

                // Badge versi di dasar panel.
                ui.painter().text(
                    egui::pos2(rect_panel.center().x, rect_panel.bottom() - 18.0),
                    egui::Align2::CENTER_CENTER,
                    format!("v{}", pasang::VERSI),
                    tema::font_kecil(),
                    tema::SEKUNDER,
                );

                ui.add_space(4.0);

                // ---- Panel kanan ----
                ui.vertical(|ui| {
                    ui.set_width(ui.available_width());
                    ui.add_space(12.0);

                    // Tahapan
                    ui.horizontal_wrapped(|ui| {
                        for (i, label) in halaman::LANGKAH.iter().enumerate() {
                            let status = halaman::status_langkah(
                                self.halaman,
                                i,
                                self.pekerja.is_some(),
                            );
                            penanda_langkah(ui, i + 1, label, status, waktu);
                            ui.add_space(6.0);
                        }
                    });

                    ui.add_space(6.0);
                    ui.separator();
                    ui.add_space(10.0);

                    // Isi halaman (dengan transisi fade + geser)
                    let (alpha, geser) = self.transisi.nilai();
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.add_space(geser);
                        let halaman_ini = self.halaman;
                        let mut konteks = KonteksHalaman {
                            rencana: &mut self.rencana,
                            setuju_lisensi: &mut self.setuju_lisensi,
                            progres: &self.progres,
                            status_teks: &self.status_teks,
                            log: &self.log,
                            selesai_ok: self.selesai_ok,
                            mencopot: self.mode == Mode::Copot,
                        };
                        let aksi = halaman::gambar(
                            ui,
                            halaman_ini,
                            &mut konteks,
                            &mut self.hover,
                            &mut self.riak,
                            dt,
                        );
                        if !matches!(aksi, Aksi::Tidak) {
                            self.tangani_aksi(aksi, &ctx);
                        }
                    });

                    // Redupkan sedikit saat transisi masuk (efek fade).
                    if alpha < 1.0 {
                        let r = ui.max_rect();
                        ui.painter().rect_filled(
                            r,
                            egui::CornerRadius::ZERO,
                            egui::Color32::from_rgba_unmultiplied(
                                tema::LATAR.r(),
                                tema::LATAR.g(),
                                tema::LATAR.b(),
                                ((1.0 - alpha) * 140.0) as u8,
                            ),
                        );
                    }
                });
            });
        });

        // Minta repaint kontinu selama ada animasi aktif.
        if self.transisi.progres() < 1.0
            || self.hover.iter().any(|h| h.aktif())
            || self.riak.iter().any(|r| r.ada())
        {
            ctx.request_repaint_after(std::time::Duration::from_millis(16));
        }
    }
}

/// Panel status untuk mode diam (tanpa UI).
pub fn jalankan_diam(mode: Mode) -> i32 {
    let mut rencana = Rencana::default();
    rencana.tujuan = Rencana::tujuan_default(false);

    let hasil = if mode == Mode::Copot {
        let mut lapor = |_p: Pesan| {};
        pasang::copot(&mut lapor)
    } else {
        let payload = crate::muat_payload();
        let mut lapor = |_p: Pesan| {};
        pasang::jalankan(&rencana, &payload, &mut lapor)
    };

    match hasil {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("BAHAYA [INSTALLER]: {}", e);
            1
        }
    }
}
