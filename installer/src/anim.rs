//! State animasi (Fase 6F). Semua efek micro-interaction dikelola di sini.

use crate::tema;

/// Pelacak hover satu tombol, dengan transisi warna.
#[derive(Default, Clone)]
pub struct AnimasiHover {
    pub progres: f32,
    /// Progres efek tekan (0 = tidak ditekan, 1 = sepenuhnya mengecil).
    pub tekan: f32,
}

impl AnimasiHover {
    /// Perbarui progres hover menuju target (0/1) memakai delta waktu nyata.
    pub fn perbarui(&mut self, aktif: bool, dt: f32) {
        let target = if aktif { 1.0 } else { 0.0 };
        let durasi = tema::durasi(tema::DURASI_HOVER).max(0.001);
        let laju = dt / durasi;
        if self.progres < target {
            self.progres = (self.progres + laju).min(target);
        } else if self.progres > target {
            self.progres = (self.progres - laju).max(target);
        }
    }

    /// Perbarui progres tekan. Dipakai untuk efek tombol "mengecil" saat diklik.
    pub fn perbarui_tekan(&mut self, aktif: bool, dt: f32) {
        let target = if aktif { 1.0 } else { 0.0 };
        let durasi = tema::durasi(tema::DURASI_TEKAN).max(0.001);
        let laju = dt / durasi;
        if self.tekan < target {
            self.tekan = (self.tekan + laju).min(target);
        } else if self.tekan > target {
            self.tekan = (self.tekan - laju).max(target);
        }
    }

    /// Faktor skala tombol saat ditekan (1.0 = normal, 0.97 = ditekan penuh).
    pub fn skala_tekan(&self) -> f32 {
        1.0 - 0.03 * self.tekan
    }

    /// Warna latar tombol saat ini (hasil interpolasi).
    pub fn warna(&self, dasar: egui::Color32, hover: egui::Color32) -> egui::Color32 {
        tema::campur_warna(dasar, hover, tema::easing_kubik_keluar(self.progres))
    }

    pub fn aktif(&self) -> bool {
        self.progres > 0.001 || self.tekan > 0.001
    }
}

/// Efek riak yang memuai dari titik klik.
#[derive(Clone)]
pub struct Riak {
    pub mulai: std::time::Instant,
    pub titik: egui::Pos2,
}

/// Riak klik satu tombol.
#[derive(Default, Clone)]
pub struct AnimasiRiak {
    pub riak: Vec<Riak>,
}

impl AnimasiRiak {
    pub fn picu(&mut self, titik: egui::Pos2) {
        self.riak.push(Riak {
            mulai: std::time::Instant::now(),
            titik,
        });
    }

    /// Buang riak yang sudah selesai; kembalikan yang masih aktif beserta progres.
    pub fn aktif(&mut self) -> Vec<(egui::Pos2, f32)> {
        let durasi = tema::DURASI_RIAK;
        let mut keluar = Vec::new();
        self.riak.retain(|r| {
            let t = r.mulai.elapsed().as_secs_f32() / durasi;
            if t < 1.0 {
                keluar.push((r.titik, t));
                true
            } else {
                false
            }
        });
        keluar
    }

    pub fn ada(&self) -> bool {
        !self.riak.is_empty()
    }
}

/// Transisi antar-halaman (fade + geser).
#[derive(Clone)]
pub struct AnimasiHalaman {
    pub mulai: std::time::Instant,
}

impl Default for AnimasiHalaman {
    fn default() -> Self {
        Self {
            mulai: std::time::Instant::now() - std::time::Duration::from_secs(10),
        }
    }
}

impl AnimasiHalaman {
    pub fn mulai_ulang(&mut self) {
        self.mulai = std::time::Instant::now();
    }

    /// Progres 0..1; 1 berarti selesai.
    pub fn progres(&self) -> f32 {
        let durasi = tema::durasi(tema::DURASI_HALAMAN);
        if durasi <= 0.0 {
            return 1.0;
        }
        (self.mulai.elapsed().as_secs_f32() / durasi).clamp(0.0, 1.0)
    }

    /// Nilai untuk alpha (fade) dan geser vertikal.
    ///
    /// Memakai easing kubik-masuk-keluar agar transisi halaman terasa halus
    /// di awal maupun akhir, bukan hanya melambat di akhir.
    pub fn nilai(&self) -> (f32, f32) {
        let p = tema::easing_kubik(self.progres());
        let alpha = p;
        let geser = (1.0 - p) * tema::GESER_HALAMAN;
        (alpha, geser)
    }
}

/// Denyut halus untuk penanda langkah aktif.
pub fn denyut(waktu: f64) -> f32 {
    let periode = tema::PERIODE_DENYUT as f64;
    let fasa = (waktu % periode) / periode; // 0..1
                                            // Sinus penuh: 1 -> 0.85 -> 1
    let s = (fasa * std::f64::consts::TAU).sin() as f32;
    0.925 + 0.075 * s
}

/// Posisi progress bar yang bergerak mulus menuju target.
#[derive(Default, Clone)]
pub struct AnimasiProgress {
    pub nilai: f32,
    target: f32,
}

impl AnimasiProgress {
    pub fn set_target(&mut self, target: f32) {
        self.target = target.clamp(0.0, 1.0);
    }

    pub fn target(&self) -> f32 {
        self.target
    }

    pub fn perbarui(&mut self, dt: f32) {
        let laju = (dt / 0.35).clamp(0.0, 1.0);
        if self.nilai < self.target {
            self.nilai =
                (self.nilai + laju * (self.target - self.nilai).max(0.02)).min(self.target);
        } else if self.nilai > self.target {
            self.nilai =
                (self.nilai - laju * (self.nilai - self.target).max(0.02)).max(self.target);
        }
    }

    pub fn bergerak(&self) -> bool {
        (self.nilai - self.target).abs() > 0.001
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn easing_kubik_keluar_terjepit() {
        assert_eq!(tema::easing_kubik_keluar(0.0), 0.0);
        assert_eq!(tema::easing_kubik_keluar(1.0), 1.0);
        assert!(tema::easing_kubik_keluar(0.5) > 0.5);
    }

    #[test]
    fn easing_kubik_simetris() {
        assert!((tema::easing_kubik(0.0) - 0.0).abs() < 1e-6);
        assert!((tema::easing_kubik(1.0) - 1.0).abs() < 1e-6);
        let t = tema::easing_kubik(0.5);
        assert!((t - 0.5).abs() < 1e-6, "tengah harus 0.5, dapat {}", t);
    }

    #[test]
    fn campur_warna_ujung() {
        let a = egui::Color32::from_rgb(0, 0, 0);
        let b = egui::Color32::from_rgb(255, 255, 255);
        assert_eq!(tema::campur_warna(a, b, 0.0), a);
        assert_eq!(tema::campur_warna(a, b, 1.0), b);
        let t = tema::campur_warna(a, b, 0.5);
        assert_eq!(t.r(), 128);
    }

    #[test]
    fn denyut_dalam_rentang() {
        for i in 0..20 {
            let w = i as f64 * 0.1;
            let d = denyut(w);
            assert!((0.80..=1.0).contains(&d), "denyut {} di luar rentang", d);
        }
    }

    #[test]
    fn progress_menuju_target() {
        let mut p = AnimasiProgress::default();
        p.set_target(1.0);
        for _ in 0..100 {
            p.perbarui(0.05);
        }
        assert!((p.nilai - 1.0).abs() < 0.01, "nilai: {}", p.nilai);
    }

    #[test]
    fn riak_dibersihkan_setelah_selesai() {
        let mut r = AnimasiRiak::default();
        r.picu(egui::pos2(1.0, 2.0));
        assert!(r.ada());
        std::thread::sleep(std::time::Duration::from_millis(
            (tema::DURASI_RIAK * 1000.0) as u64 + 30,
        ));
        let aktif = r.aktif();
        assert!(aktif.is_empty());
        assert!(!r.ada());
    }
}
