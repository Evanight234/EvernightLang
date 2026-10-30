use std::fmt;

/// Jenis-jenis plot visualisasi data dan diagnostik yang didukung.
#[derive(Debug, Clone, PartialEq)]
pub enum JenisPlot {
    Garis,
    Sebar,
    Batang,
    Histogram,
    MatriksKonfusi,
}

/// Struktur data visualisasi grafik (Plot) mandiri.
/// Menyimpan konfigurasi dan data untuk diekspor ke SVG atau dirender ke terminal ASCII.
#[derive(Debug, Clone, PartialEq)]
pub struct MatraPlot {
    pub jenis: JenisPlot,
    pub x_data: Vec<f64>,
    pub y_data: Vec<f64>,
    pub kategori: Vec<String>,
    pub matriks: Vec<Vec<f64>>,
    pub label_kelas: Vec<String>,
    pub bin_count: usize,
    pub judul: String,
    pub label_x: String,
    pub label_y: String,
    pub warna: String,
    pub warna_latar: String,
    pub grid: bool,
    pub lebar: u32,
    pub tinggi: u32,
}

impl MatraPlot {
    /// Membuat grafik garis (Line Chart) dari data sumbu X dan Y.
    pub fn garis(x: Vec<f64>, y: Vec<f64>) -> Result<Self, String> {
        if x.len() != y.len() {
            return Err(format!(
                "Panjang data X ({}) tidak sesuai dengan data Y ({})",
                x.len(),
                y.len()
            ));
        }
        if x.is_empty() {
            return Err("Data grafik garis tidak boleh kosong".to_string());
        }

        Ok(Self {
            jenis: JenisPlot::Garis,
            x_data: x,
            y_data: y,
            kategori: Vec::new(),
            matriks: Vec::new(),
            label_kelas: Vec::new(),
            bin_count: 0,
            judul: String::new(),
            label_x: "X".to_string(),
            label_y: "Y".to_string(),
            warna: "#3B82F6".to_string(),       // Biru cerah
            warna_latar: "#1E1E2E".to_string(), // Dark slate modern
            grid: true,
            lebar: 720,
            tinggi: 420,
        })
    }

    /// Membuat grafik sebar (Scatter Plot) dari data sumbu X dan Y.
    pub fn sebar(x: Vec<f64>, y: Vec<f64>) -> Result<Self, String> {
        if x.len() != y.len() {
            return Err(format!(
                "Panjang data X ({}) tidak sesuai dengan data Y ({})",
                x.len(),
                y.len()
            ));
        }
        if x.is_empty() {
            return Err("Data grafik sebar tidak boleh kosong".to_string());
        }

        Ok(Self {
            jenis: JenisPlot::Sebar,
            x_data: x,
            y_data: y,
            kategori: Vec::new(),
            matriks: Vec::new(),
            label_kelas: Vec::new(),
            bin_count: 0,
            judul: String::new(),
            label_x: "X".to_string(),
            label_y: "Y".to_string(),
            warna: "#10B981".to_string(), // Emerald green
            warna_latar: "#1E1E2E".to_string(),
            grid: true,
            lebar: 720,
            tinggi: 420,
        })
    }

    /// Membuat diagram batang (Bar Chart) dari label kategori dan nilainya.
    pub fn batang(kategori: Vec<String>, nilai: Vec<f64>) -> Result<Self, String> {
        if kategori.len() != nilai.len() {
            return Err(format!(
                "Jumlah kategori ({}) tidak sama dengan jumlah nilai ({})",
                kategori.len(),
                nilai.len()
            ));
        }
        if kategori.is_empty() {
            return Err("Data diagram batang tidak boleh kosong".to_string());
        }

        Ok(Self {
            jenis: JenisPlot::Batang,
            x_data: Vec::new(),
            y_data: nilai,
            kategori,
            matriks: Vec::new(),
            label_kelas: Vec::new(),
            bin_count: 0,
            judul: String::new(),
            label_x: "Kategori".to_string(),
            label_y: "Nilai".to_string(),
            warna: "#8B5CF6".to_string(), // Ungu violet
            warna_latar: "#1E1E2E".to_string(),
            grid: true,
            lebar: 720,
            tinggi: 420,
        })
    }

    /// Membuat histogram frekuensi dari sekumpulan data numerik.
    pub fn histogram(data: Vec<f64>, mut bin: usize) -> Result<Self, String> {
        if data.is_empty() {
            return Err("Data histogram tidak boleh kosong".to_string());
        }
        if bin == 0 {
            bin = 10;
        }

        Ok(Self {
            jenis: JenisPlot::Histogram,
            x_data: data,
            y_data: Vec::new(),
            kategori: Vec::new(),
            matriks: Vec::new(),
            label_kelas: Vec::new(),
            bin_count: bin,
            judul: String::new(),
            label_x: "Rentang Nilai".to_string(),
            label_y: "Frekuensi".to_string(),
            warna: "#F59E0B".to_string(), // Amber gold
            warna_latar: "#1E1E2E".to_string(),
            grid: true,
            lebar: 720,
            tinggi: 420,
        })
    }

    /// Membuat matriks konfusi / peta panas (Confusion Matrix) untuk diagnostik model ML.
    pub fn matriks_konfusi(matriks: Vec<Vec<f64>>, label: Vec<String>) -> Result<Self, String> {
        if matriks.is_empty() {
            return Err("Matriks konfusi tidak boleh kosong".to_string());
        }
        let n = matriks.len();
        for baris in &matriks {
            if baris.len() != n {
                return Err(format!(
                    "Matriks konfusi harus berbentuk bujur sangkar ({}x{}), ditemukan baris dengan panjang {}",
                    n, n, baris.len()
                ));
            }
        }
        if !label.is_empty() && label.len() != n {
            return Err(format!(
                "Jumlah label ({}) tidak sesuai dengan dimensi matriks ({}x{})",
                label.len(),
                n,
                n
            ));
        }

        let label_final = if label.is_empty() {
            (0..n).map(|i| format!("Kelas {}", i)).collect()
        } else {
            label
        };

        Ok(Self {
            jenis: JenisPlot::MatriksKonfusi,
            x_data: Vec::new(),
            y_data: Vec::new(),
            kategori: Vec::new(),
            matriks,
            label_kelas: label_final,
            bin_count: 0,
            judul: "Matriks Konfusi".to_string(),
            label_x: "Prediksi".to_string(),
            label_y: "Sebenarnya".to_string(),
            warna: "#6366F1".to_string(), // Indigo
            warna_latar: "#1E1E2E".to_string(),
            grid: false,
            lebar: 520,
            tinggi: 520,
        })
    }

    pub fn set_judul(&mut self, j: String) {
        self.judul = j;
    }

    pub fn set_label_x(&mut self, lx: String) {
        self.label_x = lx;
    }

    pub fn set_label_y(&mut self, ly: String) {
        self.label_y = ly;
    }

    pub fn set_warna(&mut self, w: String) {
        self.warna = w;
    }

    pub fn set_warna_latar(&mut self, wl: String) {
        self.warna_latar = wl;
    }

    pub fn set_grid(&mut self, g: bool) {
        self.grid = g;
    }

    pub fn set_ukuran(&mut self, l: u32, t: u32) {
        self.lebar = l.max(300);
        self.tinggi = t.max(200);
    }

    /// Menghasilkan representasi string berkas vektor SVG murni (Scalable Vector Graphics).
    pub fn ke_svg(&self) -> String {
        match self.jenis {
            JenisPlot::Garis => self.render_svg_garis(),
            JenisPlot::Sebar => self.render_svg_sebar(),
            JenisPlot::Batang => self.render_svg_batang(),
            JenisPlot::Histogram => self.render_svg_histogram(),
            JenisPlot::MatriksKonfusi => self.render_svg_matriks_konfusi(),
        }
    }

    /// Merender visualisasi ke tampilan teks terminal ASCII / Unicode instan.
    pub fn tampil_terminal(&self) -> String {
        match self.jenis {
            JenisPlot::Garis => self.render_terminal_garis(),
            JenisPlot::Sebar => self.render_terminal_sebar(),
            JenisPlot::Batang => self.render_terminal_batang(),
            JenisPlot::Histogram => self.render_terminal_histogram(),
            JenisPlot::MatriksKonfusi => self.render_terminal_matriks_konfusi(),
        }
    }

    // =========================================================================
    // IMPLEMENTASI RENDERER SVG
    // =========================================================================

    fn render_svg_garis(&self) -> String {
        let (min_x, max_x, min_y, max_y) = self.hitung_rentang_xy();
        let (x0, y0, x1, y1) = self.hitung_plot_box();

        let mut svg = self.mulai_svg();

        // Judul & Sumbu Label
        self.tambah_judul_dan_label(&mut svg);

        // Grid lines & Ticks
        if self.grid {
            self.tambah_grid_dan_sumbu(&mut svg, min_x, max_x, min_y, max_y, x0, y0, x1, y1);
        }

        // Jalur Garis
        let mut path_d = String::new();
        let mut circles = String::new();
        for i in 0..self.x_data.len() {
            let px = Self::petakan(self.x_data[i], min_x, max_x, x0, x1);
            let py = Self::petakan(self.y_data[i], min_y, max_y, y1, y0); // Sumbu Y dibalik

            if i == 0 {
                path_d.push_str(&format!("M {:.2} {:.2} ", px, py));
            } else {
                path_d.push_str(&format!("L {:.2} {:.2} ", px, py));
            }

            circles.push_str(&format!(
                r##"<circle cx="{:.2}" cy="{:.2}" r="3.5" fill="{}" />"##,
                px, py, self.warna
            ));
        }

        svg.push_str(&format!(
            r##"<path d="{}" fill="none" stroke="{}" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" />"##,
            path_d, self.warna
        ));
        svg.push_str(&circles);

        svg.push_str("</svg>");
        svg
    }

    fn render_svg_sebar(&self) -> String {
        let (min_x, max_x, min_y, max_y) = self.hitung_rentang_xy();
        let (x0, y0, x1, y1) = self.hitung_plot_box();

        let mut svg = self.mulai_svg();
        self.tambah_judul_dan_label(&mut svg);

        if self.grid {
            self.tambah_grid_dan_sumbu(&mut svg, min_x, max_x, min_y, max_y, x0, y0, x1, y1);
        }

        for i in 0..self.x_data.len() {
            let px = Self::petakan(self.x_data[i], min_x, max_x, x0, x1);
            let py = Self::petakan(self.y_data[i], min_y, max_y, y1, y0);

            svg.push_str(&format!(
                r##"<circle cx="{:.2}" cy="{:.2}" r="5" fill="{}" fill-opacity="0.85" stroke="#FFFFFF" stroke-width="1.2" />"##,
                px, py, self.warna
            ));
        }

        svg.push_str("</svg>");
        svg
    }

    fn render_svg_batang(&self) -> String {
        let (x0, y0, x1, y1) = self.hitung_plot_box();
        let n = self.kategori.len();

        let max_val = self.y_data.iter().cloned().fold(0.0_f64, f64::max).max(1.0);
        let min_val = 0.0_f64;

        let mut svg = self.mulai_svg();
        self.tambah_judul_dan_label(&mut svg);

        // Grid horizontal
        if self.grid {
            for step in 0..=5 {
                let frac = step as f64 / 5.0;
                let val = min_val + frac * (max_val - min_val);
                let py = y1 - frac * (y1 - y0);

                svg.push_str(&format!(
                    r##"<line x1="{:.2}" y1="{:.2}" x2="{:.2}" y2="{:.2}" stroke="#334155" stroke-dasharray="3 3" />"##,
                    x0, py, x1, py
                ));
                svg.push_str(&format!(
                    r##"<text x="{:.2}" y="{:.2}" fill="#94A3B8" font-size="11" text-anchor="end" font-family="monospace">{:.1}</text>"##,
                    x0 - 8.0, py + 4.0, val
                ));
            }
        }

        // Gambar batang
        let total_w = x1 - x0;
        let slot_w = total_w / n as f64;
        let bar_w = slot_w * 0.65;
        let gap = slot_w * 0.35 / 2.0;

        for (i, val) in self.y_data.iter().enumerate() {
            let frac = (val / max_val).clamp(0.0, 1.0);
            let bh = frac * (y1 - y0);
            let bx = x0 + i as f64 * slot_w + gap;
            let by = y1 - bh;

            svg.push_str(&format!(
                r##"<rect x="{:.2}" y="{:.2}" width="{:.2}" height="{:.2}" rx="4" fill="{}" />"##,
                bx, by, bar_w, bh, self.warna
            ));

            // Nilai di atas batang
            svg.push_str(&format!(
                r##"<text x="{:.2}" y="{:.2}" fill="#F8FAFC" font-size="11" font-weight="bold" text-anchor="middle" font-family="sans-serif">{:.0}</text>"##,
                bx + bar_w / 2.0, by - 6.0, val
            ));

            // Label kategori di bawah batang
            svg.push_str(&format!(
                r##"<text x="{:.2}" y="{:.2}" fill="#CBD5E1" font-size="12" text-anchor="middle" font-family="sans-serif">{}</text>"##,
                bx + bar_w / 2.0, y1 + 18.0, self.kategori[i]
            ));
        }

        // Garis dasar sumbu X
        svg.push_str(&format!(
            r##"<line x1="{:.2}" y1="{:.2}" x2="{:.2}" y2="{:.2}" stroke="#64748B" stroke-width="1.5" />"##,
            x0, y1, x1, y1
        ));

        svg.push_str("</svg>");
        svg
    }

    fn render_svg_histogram(&self) -> String {
        let (bins_ranges, frekuensi) = self.hitung_histogram();
        let (x0, y0, x1, y1) = self.hitung_plot_box();
        let n_bins = frekuensi.len();

        let max_freq = frekuensi.iter().cloned().fold(0.0_f64, f64::max).max(1.0);

        let mut svg = self.mulai_svg();
        self.tambah_judul_dan_label(&mut svg);

        if self.grid {
            for step in 0..=5 {
                let frac = step as f64 / 5.0;
                let val = frac * max_freq;
                let py = y1 - frac * (y1 - y0);

                svg.push_str(&format!(
                    r##"<line x1="{:.2}" y1="{:.2}" x2="{:.2}" y2="{:.2}" stroke="#334155" stroke-dasharray="3 3" />"##,
                    x0, py, x1, py
                ));
                svg.push_str(&format!(
                    r##"<text x="{:.2}" y="{:.2}" fill="#94A3B8" font-size="11" text-anchor="end" font-family="monospace">{:.0}</text>"##,
                    x0 - 8.0, py + 4.0, val
                ));
            }
        }

        let total_w = x1 - x0;
        let bar_w = total_w / n_bins as f64;

        for (i, &freq) in frekuensi.iter().enumerate() {
            let frac = freq / max_freq;
            let bh = frac * (y1 - y0);
            let bx = x0 + i as f64 * bar_w;
            let by = y1 - bh;

            svg.push_str(&format!(
                r##"<rect x="{:.2}" y="{:.2}" width="{:.2}" height="{:.2}" fill="{}" stroke="#1E1E2E" stroke-width="1.5" />"##,
                bx, by, bar_w, bh, self.warna
            ));

            if freq > 0.0 {
                svg.push_str(&format!(
                    r##"<text x="{:.2}" y="{:.2}" fill="#F8FAFC" font-size="10" text-anchor="middle" font-family="sans-serif">{:.0}</text>"##,
                    bx + bar_w / 2.0, by - 5.0, freq
                ));
            }

            // Label batas bawah
            svg.push_str(&format!(
                r##"<text x="{:.2}" y="{:.2}" fill="#94A3B8" font-size="10" text-anchor="middle" font-family="monospace">{:.1}</text>"##,
                bx, y1 + 15.0, bins_ranges[i].0
            ));
        }

        // Label batas atas terakhir
        if let Some(last) = bins_ranges.last() {
            svg.push_str(&format!(
                r##"<text x="{:.2}" y="{:.2}" fill="#94A3B8" font-size="10" text-anchor="middle" font-family="monospace">{:.1}</text>"##,
                x1, y1 + 15.0, last.1
            ));
        }

        svg.push_str(&format!(
            r##"<line x1="{:.2}" y1="{:.2}" x2="{:.2}" y2="{:.2}" stroke="#64748B" stroke-width="1.5" />"##,
            x0, y1, x1, y1
        ));

        svg.push_str("</svg>");
        svg
    }

    fn render_svg_matriks_konfusi(&self) -> String {
        let n = self.matriks.len();
        let mut svg = self.mulai_svg();

        let w = self.lebar as f64;
        let h = self.tinggi as f64;
        let margin_l = 90.0;
        let margin_b = 80.0;
        let margin_t = 60.0;
        let margin_r = 40.0;

        let grid_w = w - margin_l - margin_r;
        let grid_h = h - margin_t - margin_b;
        let cell_size = (grid_w / n as f64).min(grid_h / n as f64);

        let max_val = self
            .matriks
            .iter()
            .flat_map(|r| r.iter())
            .cloned()
            .fold(0.0_f64, f64::max)
            .max(1.0);

        // Judul
        if !self.judul.is_empty() {
            svg.push_str(&format!(
                r##"<text x="{:.2}" y="36" fill="#F8FAFC" font-size="17" font-weight="bold" text-anchor="middle" font-family="sans-serif">{}</text>"##,
                w / 2.0, self.judul
            ));
        }

        // Label sumbu Y (Sebenarnya)
        svg.push_str(&format!(
            r##"<text transform="rotate(-90)" x="{:.2}" y="24" fill="#94A3B8" font-size="12" font-weight="600" text-anchor="middle" font-family="sans-serif">{}</text>"##,
            -(margin_t + (cell_size * n as f64) / 2.0), self.label_y
        ));

        // Label sumbu X (Prediksi)
        svg.push_str(&format!(
            r##"<text x="{:.2}" y="{:.2}" fill="#94A3B8" font-size="12" font-weight="600" text-anchor="middle" font-family="sans-serif">{}</text>"##,
            margin_l + (cell_size * n as f64) / 2.0, margin_t + cell_size * n as f64 + 48.0, self.label_x
        ));

        // Sel matriks
        for r in 0..n {
            // Label kelas baris
            svg.push_str(&format!(
                r##"<text x="{:.2}" y="{:.2}" fill="#CBD5E1" font-size="12" text-anchor="end" font-family="sans-serif">{}</text>"##,
                margin_l - 12.0, margin_t + r as f64 * cell_size + cell_size / 2.0 + 4.0, self.label_kelas[r]
            ));

            for c in 0..n {
                let val = self.matriks[r][c];
                let intensity = (val / max_val).clamp(0.08, 0.95);
                let cx = margin_l + c as f64 * cell_size;
                let cy = margin_t + r as f64 * cell_size;

                svg.push_str(&format!(
                    r##"<rect x="{:.2}" y="{:.2}" width="{:.2}" height="{:.2}" fill="{}" fill-opacity="{:.2}" stroke="#1E1E2E" stroke-width="2" rx="4" />"##,
                    cx, cy, cell_size, cell_size, self.warna, intensity
                ));

                svg.push_str(&format!(
                    r##"<text x="{:.2}" y="{:.2}" fill="#FFFFFF" font-size="14" font-weight="bold" text-anchor="middle" font-family="monospace">{:.0}</text>"##,
                    cx + cell_size / 2.0, cy + cell_size / 2.0 + 5.0, val
                ));
            }
        }

        // Label kelas kolom (bawah)
        for c in 0..n {
            svg.push_str(&format!(
                r##"<text x="{:.2}" y="{:.2}" fill="#CBD5E1" font-size="12" text-anchor="middle" font-family="sans-serif">{}</text>"##,
                margin_l + c as f64 * cell_size + cell_size / 2.0, margin_t + cell_size * n as f64 + 20.0, self.label_kelas[c]
            ));
        }

        svg.push_str("</svg>");
        svg
    }

    // =========================================================================
    // IMPLEMENTASI RENDERER TERMINAL ASCII
    // =========================================================================

    fn render_terminal_garis(&self) -> String {
        self.render_terminal_grid(true)
    }

    fn render_terminal_sebar(&self) -> String {
        self.render_terminal_grid(false)
    }

    fn render_terminal_grid(&self, connect_line: bool) -> String {
        let (min_x, max_x, min_y, max_y) = self.hitung_rentang_xy();
        let cols = 55;
        let rows = 14;

        let mut grid = vec![vec![' '; cols]; rows];

        for i in 0..self.x_data.len() {
            let c = Self::petakan_usize(self.x_data[i], min_x, max_x, cols.saturating_sub(1));
            let r_inv = Self::petakan_usize(self.y_data[i], min_y, max_y, rows.saturating_sub(1));
            let r = rows.saturating_sub(1) - r_inv;

            if connect_line && i > 0 {
                let prev_c =
                    Self::petakan_usize(self.x_data[i - 1], min_x, max_x, cols.saturating_sub(1));
                let prev_r_inv =
                    Self::petakan_usize(self.y_data[i - 1], min_y, max_y, rows.saturating_sub(1));
                let prev_r = rows.saturating_sub(1) - prev_r_inv;

                // Garis interpolasi sederhana antar titik
                let steps = (c as isize - prev_c as isize)
                    .abs()
                    .max((r as isize - prev_r as isize).abs()) as usize;
                for s in 0..=steps {
                    let t = if steps == 0 {
                        0.0
                    } else {
                        s as f64 / steps as f64
                    };
                    let ic = ((1.0 - t) * prev_c as f64 + t * c as f64).round() as usize;
                    let ir = ((1.0 - t) * prev_r as f64 + t * r as f64).round() as usize;
                    if ir < rows && ic < cols {
                        grid[ir][ic] = '•';
                    }
                }
            }
            grid[r][c] = '*';
        }

        let mut out = String::new();
        if !self.judul.is_empty() {
            out.push_str(&format!("  [Grafik: {}]\n", self.judul));
        }

        for r in 0..rows {
            let frac = 1.0 - (r as f64 / (rows - 1) as f64);
            let y_val = min_y + frac * (max_y - min_y);
            out.push_str(&format!("{:8.2} │", y_val));
            for c in 0..cols {
                out.push(grid[r][c]);
            }
            out.push('\n');
        }

        out.push_str("         └");
        out.push_str(&"─".repeat(cols));
        out.push('\n');
        out.push_str(&format!(
            "{:8.2} {:width$}{:.2}\n",
            min_x,
            "",
            max_x,
            width = cols.saturating_sub(12)
        ));
        if !self.label_x.is_empty() || !self.label_y.is_empty() {
            out.push_str(&format!(
                "  Sumbu: X = {}, Y = {}\n",
                self.label_x, self.label_y
            ));
        }

        out
    }

    fn render_terminal_batang(&self) -> String {
        let max_val = self.y_data.iter().cloned().fold(0.0_f64, f64::max).max(1.0);
        let max_len_label = self
            .kategori
            .iter()
            .map(|k| k.len())
            .max()
            .unwrap_or(8)
            .max(6);
        let max_bar_cols = 35;

        let mut out = String::new();
        if !self.judul.is_empty() {
            out.push_str(&format!("  [Diagram Batang: {}]\n", self.judul));
        }

        for (i, cat) in self.kategori.iter().enumerate() {
            let val = self.y_data[i];
            let frac = (val / max_val).clamp(0.0, 1.0);
            let bar_len = (frac * max_bar_cols as f64).round() as usize;

            let bar_str = "█".repeat(bar_len);
            out.push_str(&format!(
                "  {:<width$} │ {} {:.2}\n",
                cat,
                bar_str,
                val,
                width = max_len_label
            ));
        }

        out
    }

    fn render_terminal_histogram(&self) -> String {
        let (ranges, frekuensi) = self.hitung_histogram();
        let max_freq = frekuensi.iter().cloned().fold(0.0_f64, f64::max).max(1.0);
        let max_bar_cols = 30;

        let mut out = String::new();
        if !self.judul.is_empty() {
            out.push_str(&format!("  [Histogram: {}]\n", self.judul));
        }

        for (i, (b_min, b_max)) in ranges.iter().enumerate() {
            let freq = frekuensi[i];
            let frac = freq / max_freq;
            let bar_len = (frac * max_bar_cols as f64).round() as usize;
            let bar_str = "█".repeat(bar_len);

            out.push_str(&format!(
                "  [{:6.2} - {:6.2}] │ {} ({:.0})\n",
                b_min, b_max, bar_str, freq
            ));
        }

        out
    }

    fn render_terminal_matriks_konfusi(&self) -> String {
        let n = self.matriks.len();
        let mut out = String::new();

        if !self.judul.is_empty() {
            out.push_str(&format!("  [Diagnostik: {}]\n", self.judul));
        }

        let max_label_len = self
            .label_kelas
            .iter()
            .map(|l| l.len())
            .max()
            .unwrap_or(8)
            .max(8);
        let cell_w = 10;

        out.push_str(&format!(
            "  {:<width$} │ Prediksi:\n",
            "",
            width = max_label_len + 2
        ));
        out.push_str(&format!(
            "  {:<width$} │",
            "Sebenarnya",
            width = max_label_len
        ));

        for label in &self.label_kelas {
            out.push_str(&format!(" {:^width$}", label, width = cell_w));
        }
        out.push('\n');

        out.push_str(&format!(
            "  {}┼{}\n",
            "─".repeat(max_label_len + 2),
            "─".repeat((cell_w + 1) * n + 1)
        ));

        for r in 0..n {
            out.push_str(&format!(
                "  {:<width$} │",
                self.label_kelas[r],
                width = max_label_len
            ));
            for c in 0..n {
                out.push_str(&format!(
                    " {:^width$.0}",
                    self.matriks[r][c],
                    width = cell_w
                ));
            }
            out.push('\n');
        }

        out
    }

    // =========================================================================
    // UTILITAS PERHITUNGAN & GEOMETRI
    // =========================================================================

    fn hitung_rentang_xy(&self) -> (f64, f64, f64, f64) {
        let mut min_x = self.x_data.iter().cloned().fold(f64::INFINITY, f64::min);
        let mut max_x = self
            .x_data
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);
        let mut min_y = self.y_data.iter().cloned().fold(f64::INFINITY, f64::min);
        let mut max_y = self
            .y_data
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);

        if min_x == max_x {
            min_x -= 1.0;
            max_x += 1.0;
        }
        if min_y == max_y {
            min_y -= 1.0;
            max_y += 1.0;
        }

        (min_x, max_x, min_y, max_y)
    }

    fn hitung_plot_box(&self) -> (f64, f64, f64, f64) {
        let x0 = 70.0;
        let y0 = 60.0;
        let x1 = self.lebar as f64 - 40.0;
        let y1 = self.tinggi as f64 - 55.0;
        (x0, y0, x1, y1)
    }

    fn mulai_svg(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {} {}" width="{}" height="{}" style="background-color: {}; border-radius: 8px;">"#,
            self.lebar, self.tinggi, self.lebar, self.tinggi, self.warna_latar
        ));
        s
    }

    fn tambah_judul_dan_label(&self, svg: &mut String) {
        let w = self.lebar as f64;
        let h = self.tinggi as f64;

        if !self.judul.is_empty() {
            svg.push_str(&format!(
                r##"<text x="{:.2}" y="35" fill="#F8FAFC" font-size="18" font-weight="bold" text-anchor="middle" font-family="Inter, system-ui, sans-serif">{}</text>"##,
                w / 2.0, self.judul
            ));
        }

        if !self.label_x.is_empty() {
            svg.push_str(&format!(
                r##"<text x="{:.2}" y="{:.2}" fill="#94A3B8" font-size="13" text-anchor="middle" font-family="Inter, system-ui, sans-serif">{}</text>"##,
                w / 2.0, h - 14.0, self.label_x
            ));
        }

        if !self.label_y.is_empty() {
            svg.push_str(&format!(
                r##"<text transform="rotate(-90)" x="{:.2}" y="24" fill="#94A3B8" font-size="13" text-anchor="middle" font-family="Inter, system-ui, sans-serif">{}</text>"##,
                -(h / 2.0), self.label_y
            ));
        }
    }

    fn tambah_grid_dan_sumbu(
        &self,
        svg: &mut String,
        min_x: f64,
        max_x: f64,
        min_y: f64,
        max_y: f64,
        x0: f64,
        y0: f64,
        x1: f64,
        y1: f64,
    ) {
        // Grid Horizontal (Sumbu Y)
        for i in 0..=5 {
            let frac = i as f64 / 5.0;
            let val = min_y + frac * (max_y - min_y);
            let py = y1 - frac * (y1 - y0);

            svg.push_str(&format!(
                r##"<line x1="{:.2}" y1="{:.2}" x2="{:.2}" y2="{:.2}" stroke="#334155" stroke-dasharray="3 3" />"##,
                x0, py, x1, py
            ));
            svg.push_str(&format!(
                r##"<text x="{:.2}" y="{:.2}" fill="#94A3B8" font-size="11" text-anchor="end" font-family="monospace">{:.1}</text>"##,
                x0 - 8.0, py + 4.0, val
            ));
        }

        // Grid Vertikal (Sumbu X)
        for i in 0..=5 {
            let frac = i as f64 / 5.0;
            let val = min_x + frac * (max_x - min_x);
            let px = x0 + frac * (x1 - x0);

            svg.push_str(&format!(
                r##"<line x1="{:.2}" y1="{:.2}" x2="{:.2}" y2="{:.2}" stroke="#334155" stroke-dasharray="3 3" />"##,
                px, y0, px, y1
            ));
            svg.push_str(&format!(
                r##"<text x="{:.2}" y="{:.2}" fill="#94A3B8" font-size="11" text-anchor="middle" font-family="monospace">{:.1}</text>"##,
                px, y1 + 18.0, val
            ));
        }

        // Garis batas sumbu X dan Y
        svg.push_str(&format!(
            r##"<line x1="{:.2}" y1="{:.2}" x2="{:.2}" y2="{:.2}" stroke="#64748B" stroke-width="1.5" />"##,
            x0, y1, x1, y1
        ));
        svg.push_str(&format!(
            r##"<line x1="{:.2}" y1="{:.2}" x2="{:.2}" y2="{:.2}" stroke="#64748B" stroke-width="1.5" />"##,
            x0, y0, x0, y1
        ));
    }

    fn hitung_histogram(&self) -> (Vec<(f64, f64)>, Vec<f64>) {
        let n_bins = self.bin_count.max(1);
        let mut min_val = self.x_data.iter().cloned().fold(f64::INFINITY, f64::min);
        let mut max_val = self
            .x_data
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);

        if min_val == max_val {
            min_val -= 1.0;
            max_val += 1.0;
        }

        let bin_w = (max_val - min_val) / n_bins as f64;
        let mut ranges = Vec::with_capacity(n_bins);
        let mut frekuensi = vec![0.0_f64; n_bins];

        for i in 0..n_bins {
            let b0 = min_val + i as f64 * bin_w;
            let b1 = if i == n_bins - 1 { max_val } else { b0 + bin_w };
            ranges.push((b0, b1));
        }

        for &val in &self.x_data {
            let idx = ((val - min_val) / bin_w).floor() as usize;
            let safe_idx = idx.min(n_bins - 1);
            frekuensi[safe_idx] += 1.0;
        }

        (ranges, frekuensi)
    }

    fn petakan(val: f64, src_min: f64, src_max: f64, dst_min: f64, dst_max: f64) -> f64 {
        if src_min == src_max {
            return (dst_min + dst_max) / 2.0;
        }
        let frac = (val - src_min) / (src_max - src_min);
        dst_min + frac * (dst_max - dst_min)
    }

    fn petakan_usize(val: f64, src_min: f64, src_max: f64, max_dst: usize) -> usize {
        if src_min == src_max {
            return 0;
        }
        let frac = ((val - src_min) / (src_max - src_min)).clamp(0.0, 1.0);
        (frac * max_dst as f64).round() as usize
    }
}

impl fmt::Display for MatraPlot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.tampil_terminal())
    }
}
