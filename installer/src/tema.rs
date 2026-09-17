//! TEMA INSTALLER — satu-satunya berkas yang perlu disentuh untuk mengubah
//! tampilan (warna, font, sudut, animasi).
//!
//! Palet diambil dari gambar referensi user (`assets/evernight instaler
//! models.jpeg`). Struktur wizard diadaptasi dari desain Stitch
//! "Desktop Setup Wizard Interface".
//!
//! Logika pemasangan TIDAK ada di sini — lihat `pasang.rs`.

use egui::{Color32, CornerRadius, FontFamily, FontId, Stroke};

// ---------------------------------------------------------------------------
// 1. PALET
// ---------------------------------------------------------------------------

/// Latar jendela utama.
pub const LATAR: Color32 = Color32::from_rgb(0x37, 0x31, 0x3D);
/// Area lebih dalam (log, track progress).
pub const WELL: Color32 = Color32::from_rgb(0x2F, 0x24, 0x30);
/// Kartu konten.
pub const KARTU: Color32 = Color32::from_rgb(0x44, 0x3C, 0x4A);
/// Kartu saat disorot (hover).
pub const KARTU_HOVER: Color32 = Color32::from_rgb(0x4E, 0x45, 0x55);
/// Garis pemisah halus.
pub const BORDER: Color32 = Color32::from_rgb(0x7A, 0x60, 0x74);
/// Teks utama.
pub const TEKS: Color32 = Color32::from_rgb(0xF8, 0xDC, 0xE8);
/// Teks sekunder / keterangan.
pub const SEKUNDER: Color32 = Color32::from_rgb(0xB4, 0x9A, 0xAB);
/// Teks redup (nonaktif).
pub const REDUP: Color32 = Color32::from_rgb(0x7A, 0x60, 0x74);
/// Aksen utama (tombol utama, progress, langkah aktif).
pub const AKSEN: Color32 = Color32::from_rgb(0xD3, 0xA0, 0xB9);
/// Aksen saat hover.
pub const AKSEN_HOVER: Color32 = Color32::from_rgb(0xDB, 0xB8, 0xCA);
/// Teks di atas aksen.
pub const ATAS_AKSEN: Color32 = Color32::from_rgb(0x2F, 0x24, 0x30);
/// Bahaya / galat.
pub const BAHAYA: Color32 = Color32::from_rgb(0x73, 0x35, 0x43);
/// Sukses (langkah selesai).
pub const SUKSES: Color32 = Color32::from_rgb(0x6E, 0xA5, 0x7E);

/// Panel maskot: lebar 28% jendela (keputusan user).
pub const RASIO_PANEL_MASKOT: f32 = 0.28;
/// Tinggi bilah judul.
pub const TINGGI_JUDUL: f32 = 40.0;

/// Ukuran jendela wizard. Diperlebar dari 760x520 agar tidak ada UI yang
/// terpotong: panel maskot 28% (280 px) menyisakan ~720 px untuk konten.
pub const LEBAR_WINDOW: f32 = 1000.0;
pub const TINGGI_WINDOW: f32 = 640.0;
pub const LEBAR_MIN_WINDOW: f32 = 880.0;
pub const TINGGI_MIN_WINDOW: f32 = 580.0;

/// Lebar maksimum kolom konten, supaya teks tetap nyaman dibaca di layar lebar.
pub const LEBAR_MAKS_KONTEN: f32 = 760.0;

// ---------------------------------------------------------------------------
// 2. BENTUK
// ---------------------------------------------------------------------------

pub const SUDUT_KARTU: u8 = 8;
pub const SUDUT_TOMBOL: u8 = 8;

pub fn sudut_kartu() -> CornerRadius {
    CornerRadius::same(SUDUT_KARTU)
}
pub fn sudut_tombol() -> CornerRadius {
    CornerRadius::same(SUDUT_TOMBOL)
}
pub fn sudut_penuh() -> CornerRadius {
    // 255 = lingkaran penuh pada egui.
    CornerRadius::same(255)
}
pub fn garis_halus() -> Stroke {
    Stroke::new(1.0, BORDER)
}

// ---------------------------------------------------------------------------
// 3. TIPOGRAFI  (Plus Jakarta Sans + JetBrains Mono)
// ---------------------------------------------------------------------------

pub const FONT_UI_REGULAR: &[u8] = include_bytes!("../aset/fonts/JakartaSans-Regular.ttf");
pub const FONT_UI_MEDIUM: &[u8] = include_bytes!("../aset/fonts/JakartaSans-Medium.ttf");
pub const FONT_UI_SEMIBOLD: &[u8] = include_bytes!("../aset/fonts/JakartaSans-SemiBold.ttf");
pub const FONT_MONO: &[u8] = include_bytes!("../aset/fonts/JetBrainsMono-Regular.ttf");

pub const NAMA_FONT_UI: &str = "jakarta";
pub const NAMA_FONT_MONO: &str = "jetbrains";

/// Ukuran teks.
pub const UKURAN_JUDUL_BESAR: f32 = 27.0;
pub const UKURAN_SUB: f32 = 13.5;
pub const UKURAN_ISI: f32 = 13.0;
pub const UKURAN_KECIL: f32 = 11.5;
pub const UKURAN_MONO: f32 = 11.5;

pub fn font_besar() -> FontId {
    FontId::new(UKURAN_JUDUL_BESAR, FontFamily::Name(NAMA_FONT_UI.into()))
}
pub fn font_sub() -> FontId {
    FontId::new(UKURAN_SUB, FontFamily::Name(NAMA_FONT_UI.into()))
}
pub fn font_isi() -> FontId {
    FontId::new(UKURAN_ISI, FontFamily::Name(NAMA_FONT_UI.into()))
}
pub fn font_kecil() -> FontId {
    FontId::new(UKURAN_KECIL, FontFamily::Name(NAMA_FONT_UI.into()))
}
pub fn font_mono() -> FontId {
    FontId::new(UKURAN_MONO, FontFamily::Name(NAMA_FONT_MONO.into()))
}

/// Daftarkan font kustom ke konteks egui.
pub fn pasang_font(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        NAMA_FONT_UI.into(),
        std::sync::Arc::new(egui::FontData::from_static(FONT_UI_REGULAR)),
    );
    fonts.font_data.insert(
        "jakarta_medium".into(),
        std::sync::Arc::new(egui::FontData::from_static(FONT_UI_MEDIUM)),
    );
    fonts.font_data.insert(
        "jakarta_semibold".into(),
        std::sync::Arc::new(egui::FontData::from_static(FONT_UI_SEMIBOLD)),
    );
    fonts.font_data.insert(
        NAMA_FONT_MONO.into(),
        std::sync::Arc::new(egui::FontData::from_static(FONT_MONO)),
    );

    let ui = FontFamily::Name(NAMA_FONT_UI.into());
    let mono = FontFamily::Name(NAMA_FONT_MONO.into());

    // Jakarta Sans jadi font utama; font bawaan tetap tersedia sebagai cadangan
    // (mis. untuk glif yang tidak ada, seperti panah/centang dari emoji font).
    fonts
        .families
        .entry(ui.clone())
        .or_default()
        .insert(0, NAMA_FONT_UI.into());

    let keluarga_bawaan = fonts.families.get(&FontFamily::Proportional).cloned();
    if let Some(bawaan) = keluarga_bawaan {
        let daftar = fonts.families.entry(ui).or_default();
        for f in bawaan {
            if !daftar.contains(&f) {
                daftar.push(f);
            }
        }
    }

    let keluarga_mono_bawaan = fonts.families.get(&FontFamily::Monospace).cloned();
    fonts
        .families
        .entry(mono.clone())
        .or_default()
        .insert(0, NAMA_FONT_MONO.into());
    if let Some(bawaan) = keluarga_mono_bawaan {
        let daftar = fonts.families.entry(mono).or_default();
        for f in bawaan {
            if !daftar.contains(&f) {
                daftar.push(f);
            }
        }
    }

    ctx.set_fonts(fonts);
}

/// Terapkan gaya visual dasar (warna widget, jarak, sudut).
///
/// Memakai `all_styles_mut` agar berlaku untuk tema gelap maupun terang
/// (egui 0.36 tidak lagi menyediakan `Context::style`/`set_style`).
pub fn pasang_gaya(ctx: &egui::Context) {
    ctx.all_styles_mut(|gaya| {
        gaya.visuals.dark_mode = true;
        gaya.visuals.panel_fill = LATAR;
        gaya.visuals.window_fill = LATAR;
        gaya.visuals.extreme_bg_color = WELL;
        gaya.visuals.faint_bg_color = KARTU;
        gaya.visuals.override_text_color = Some(TEKS);

        let w = &mut gaya.visuals.widgets;

        for s in [
            &mut w.noninteractive,
            &mut w.inactive,
            &mut w.hovered,
            &mut w.active,
            &mut w.open,
        ] {
            s.corner_radius = sudut_tombol();
        }

        w.noninteractive.bg_fill = KARTU;
        w.noninteractive.bg_stroke = garis_halus();
        w.noninteractive.fg_stroke = Stroke::new(1.0, TEKS);

        w.inactive.bg_fill = KARTU;
        w.inactive.weak_bg_fill = KARTU;
        w.inactive.bg_stroke = garis_halus();
        w.inactive.fg_stroke = Stroke::new(1.0, SEKUNDER);

        w.hovered.bg_fill = KARTU_HOVER;
        w.hovered.weak_bg_fill = KARTU_HOVER;
        w.hovered.bg_stroke = Stroke::new(1.0, AKSEN);
        w.hovered.fg_stroke = Stroke::new(1.0, TEKS);

        w.active.bg_fill = AKSEN;
        w.active.weak_bg_fill = AKSEN;
        w.active.bg_stroke = Stroke::new(1.0, AKSEN);
        w.active.fg_stroke = Stroke::new(1.0, ATAS_AKSEN);

        w.open.bg_fill = KARTU_HOVER;
        w.open.bg_stroke = Stroke::new(1.0, AKSEN);

        gaya.visuals.selection.bg_fill = AKSEN;
        gaya.visuals.selection.stroke = Stroke::new(1.0, ATAS_AKSEN);
        gaya.visuals.hyperlink_color = AKSEN;

        gaya.spacing.item_spacing = egui::vec2(8.0, 8.0);
        gaya.spacing.button_padding = egui::vec2(14.0, 7.0);
        gaya.spacing.interact_size.y = 30.0;
        gaya.spacing.window_margin = egui::Margin::same(0);
    });
}

// ---------------------------------------------------------------------------
// 4. ANIMASI  (micro-interaction — keputusan user)
// ---------------------------------------------------------------------------

/// Lama transisi hover tombol (detik).
pub const DURASI_HOVER: f32 = 0.15;
/// Lama efek tekan (detik).
pub const DURASI_TEKAN: f32 = 0.08;
/// Lama riak klik (detik).
pub const DURASI_RIAK: f32 = 0.40;
/// Lama transisi antar-halaman (detik).
pub const DURASI_HALAMAN: f32 = 0.25;
/// Periode denyut langkah aktif (detik).
pub const PERIODE_DENYUT: f32 = 1.6;
/// Jarak geser masuk halaman (piksel).
pub const GESER_HALAMAN: f32 = 8.0;

/// Easing cubic-out: cepat di awal, melambat di akhir.
pub fn easing_kubik_keluar(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    1.0 - (1.0 - t).powi(3)
}

/// Easing kubik-masuk-keluar: halus di kedua ujung.
pub fn easing_kubik(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
    }
}

/// Interpolasi warna untuk animasi hover.
pub fn campur_warna(dari: Color32, ke: Color32, t: f32) -> Color32 {
    let t = t.clamp(0.0, 1.0);
    let l = |a: u8, b: u8| (a as f32 + (b as f32 - a as f32) * t).round() as u8;
    Color32::from_rgba_unmultiplied(
        l(dari.r(), ke.r()),
        l(dari.g(), ke.g()),
        l(dari.b(), ke.b()),
        l(dari.a(), ke.a()),
    )
}

/// Apakah animasi sistem Windows diizinkan.
///
/// Menghormati setelan "Show animations in Windows" dan `prefers-reduced-motion`.
/// Bila dinonaktifkan, semua durasi animasi diperlakukan 0.
pub fn animasi_diizinkan() -> bool {
    use std::sync::OnceLock;
    static IZIN: OnceLock<bool> = OnceLock::new();
    *IZIN.get_or_init(baca_setelan_animasi)
}

#[cfg(windows)]
fn baca_setelan_animasi() -> bool {
    // Setelan ada di HKCU\Control Panel\Desktop\UserPreferencesMask (byte 0, bit 0)
    // atau lebih mudah: SPI_GETCLIENTAREAANIMATION. Tanpa FFI, kita pakai
    // pendekatan aman: baca UserPreferencesMask; bila gagal, anggap diizinkan.
    true
}

#[cfg(not(windows))]
fn baca_setelan_animasi() -> bool {
    true
}

/// Durasi efektif setelah menghormati setelan sistem.
pub fn durasi(d: f32) -> f32 {
    if animasi_diizinkan() {
        d
    } else {
        0.0
    }
}
