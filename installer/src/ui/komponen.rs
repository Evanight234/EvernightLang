//! Komponen UI: tombol beranimasi, progress bar, penanda langkah, kartu.

use crate::anim::{AnimasiHover, AnimasiProgress, AnimasiRiak};
use crate::tema;

/// Tombol dengan animasi hover + tekan + riak.
pub struct Tombol<'a> {
    pub teks: &'a str,
    pub utama: bool,
    pub aktif: bool,
    pub lebar_min: f32,
}

impl<'a> Tombol<'a> {
    pub fn utama(teks: &'a str) -> Self {
        Self {
            teks,
            utama: true,
            aktif: true,
            lebar_min: 96.0,
        }
    }

    pub fn kedua(teks: &'a str) -> Self {
        Self {
            teks,
            utama: false,
            aktif: true,
            lebar_min: 96.0,
        }
    }

    pub fn aktif(mut self, v: bool) -> Self {
        self.aktif = v;
        self
    }

    pub fn lebar(mut self, v: f32) -> Self {
        self.lebar_min = v;
        self
    }
}

/// Gambar satu tombol. Mengembalikan `true` bila diklik.
pub fn tombol(
    ui: &mut egui::Ui,
    t: Tombol<'_>,
    hover: &mut AnimasiHover,
    riak: &mut AnimasiRiak,
) -> bool {
    let tinggi = 32.0;
    let lebar = t.lebar_min.max(ukuran_teks(ui, t.teks) + 34.0);
    let (rect, respons) =
        ui.allocate_exact_size(egui::vec2(lebar, tinggi), egui::Sense::click());

    let bisa = t.aktif;
    let dt = ui.input(|i| i.stable_dt);
    hover.perbarui(bisa && respons.hovered(), dt);
    hover.perbarui_tekan(bisa && respons.is_pointer_button_down_on(), dt);

    if bisa && respons.clicked() {
        if let Some(p) = ui.input(|i| i.pointer.interact_pos()) {
            riak.picu(p);
        } else {
            riak.picu(rect.center());
        }
    }

    let (dasar, hover_warna, garis, warna_teks) = if !bisa {
        (tema::WELL, tema::WELL, tema::REDUP, tema::REDUP)
    } else if t.utama {
        (tema::AKSEN, tema::AKSEN_HOVER, tema::AKSEN, tema::ATAS_AKSEN)
    } else {
        (tema::KARTU, tema::KARTU_HOVER, tema::BORDER, tema::SEKUNDER)
    };

    let latar = if bisa {
        hover.warna(dasar, hover_warna)
    } else {
        dasar
    };

    // Efek tekan: tombol sedikit mengecil (animasi, bukan loncatan).
    let rect_gambar = if bisa {
        egui::Rect::from_center_size(rect.center(), rect.size() * hover.skala_tekan())
    } else {
        rect
    };

    let pelukis = ui.painter();
    pelukis.rect_filled(rect_gambar, tema::sudut_tombol(), latar);
    pelukis.rect_stroke(
        rect_gambar,
        tema::sudut_tombol(),
        egui::Stroke::new(1.0, if bisa { garis } else { tema::REDUP }),
        egui::StrokeKind::Inside,
    );

    // Riak: lingkaran memuai dari titik klik, dipotong ke bentuk tombol.
    let aktif_riak = riak.aktif();
    if !aktif_riak.is_empty() {
        let pelukis_riak = ui.painter().with_clip_rect(rect_gambar.shrink(1.0));
        for (titik, progres) in aktif_riak {
            let radius = (rect_gambar.width().max(rect_gambar.height()) * 1.6)
                * tema::easing_kubik_keluar(progres);
            let alpha = ((1.0 - progres) * 0.18 * 255.0) as u8;
            let warna = egui::Color32::from_rgba_unmultiplied(
                tema::AKSEN_HOVER.r(),
                tema::AKSEN_HOVER.g(),
                tema::AKSEN_HOVER.b(),
                alpha,
            );
            pelukis_riak.circle_filled(titik, radius, warna);
        }
    }

    ui.painter().text(
        rect_gambar.center(),
        egui::Align2::CENTER_CENTER,
        t.teks,
        tema::font_isi(),
        warna_teks,
    );

    if bisa && (respons.hovered() || respons.has_focus()) {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }

    bisa && respons.clicked()
}

fn ukuran_teks(ui: &egui::Ui, teks: &str) -> f32 {
    ui.painter()
        .layout_no_wrap(teks.to_string(), tema::font_isi(), tema::TEKS)
        .size()
        .x
}

/// Progress bar dengan isian yang bergerak mulus.
pub fn progress(ui: &mut egui::Ui, p: &AnimasiProgress, lebar: f32, tinggi: f32) {
    let (rect, _) = ui.allocate_exact_size(egui::vec2(lebar, tinggi), egui::Sense::hover());
    let pelukis = ui.painter();

    pelukis.rect_filled(rect, tema::sudut_penuh(), tema::WELL);

    let isi_lebar = (rect.width() * p.nilai).max(0.0);
    if isi_lebar > 1.0 {
        let isi = egui::Rect::from_min_size(rect.min, egui::vec2(isi_lebar, rect.height()));
        pelukis.rect_filled(isi, tema::sudut_penuh(), tema::AKSEN);
    }
}

/// Kartu konten.
pub fn kartu<R>(ui: &mut egui::Ui, isi: impl FnOnce(&mut egui::Ui) -> R) -> R {
    egui::Frame::new()
        .fill(tema::KARTU)
        .stroke(tema::garis_halus())
        .corner_radius(tema::sudut_kartu())
        .inner_margin(egui::Margin::same(18))
        .show(ui, isi)
        .inner
}

/// Penanda satu langkah wizard (lingkaran bernomor / centang).
pub fn penanda_langkah(
    ui: &mut egui::Ui,
    nomor: usize,
    label: &str,
    status: StatusLangkah,
    waktu: f64,
) {
    ui.horizontal(|ui| {
        let ukuran = 22.0;
        let (rect, _) = ui.allocate_exact_size(egui::vec2(ukuran, ukuran), egui::Sense::hover());
        let pelukis = ui.painter();

        let (isi, warna_teks) = match status {
            StatusLangkah::Selesai => (tema::SUKSES, tema::ATAS_AKSEN),
            StatusLangkah::Aktif => (tema::AKSEN, tema::ATAS_AKSEN),
            StatusLangkah::Menunggu => (tema::WELL, tema::SEKUNDER),
        };

        // Denyut halus pada langkah aktif.
        let (rect_gambar, isi_final) = if matches!(status, StatusLangkah::Aktif) {
            let d = crate::anim::denyut(waktu);
            let r = egui::Rect::from_center_size(rect.center(), rect.size() * d);
            (
                r,
                egui::Color32::from_rgba_unmultiplied(
                    isi.r(),
                    isi.g(),
                    isi.b(),
                    (d * 255.0) as u8,
                ),
            )
        } else {
            (rect, isi)
        };

        pelukis.circle_filled(rect_gambar.center(), rect_gambar.width() / 2.0, isi_final);
        if matches!(status, StatusLangkah::Menunggu) {
            pelukis.circle_stroke(
                rect_gambar.center(),
                rect_gambar.width() / 2.0,
                egui::Stroke::new(1.0, tema::REDUP),
            );
        }

        let teks_marker = if matches!(status, StatusLangkah::Selesai) {
            "v".to_string()
        } else {
            nomor.to_string()
        };
        pelukis.text(
            rect_gambar.center(),
            egui::Align2::CENTER_CENTER,
            teks_marker,
            tema::font_kecil(),
            warna_teks,
        );

        let warna_label = match status {
            StatusLangkah::Aktif => tema::TEKS,
            StatusLangkah::Selesai => tema::SEKUNDER,
            StatusLangkah::Menunggu => tema::REDUP,
        };
        ui.label(egui::RichText::new(label).font(tema::font_kecil()).color(warna_label));
    });
}

#[derive(Clone, Copy, PartialEq)]
pub enum StatusLangkah {
    Selesai,
    Aktif,
    Menunggu,
}
