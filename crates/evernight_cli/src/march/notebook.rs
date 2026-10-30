//! Model data, parser JSON, dan serializer berkas `.emlbn` (Evernight Machine Learning Notebook).
//!
//! Format berkas `.emlbn` berbasis JSON terbuka yang menyimpan konten sel kode,
//! teks penjelasan (markdown), urutan eksekusi, serta keluaran visual interaktif.

use crate::json::{self, Json};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub struct MarchOutput {
    pub tipe: String, // "teks", "tabel_html", "grafik_svg", "error"
    pub konten: String,
    pub baris_tabel: Option<Vec<Vec<String>>>,
    pub kolom_tabel: Option<Vec<String>>,
    pub svg: Option<String>,
}

impl MarchOutput {
    pub fn ke_json(&self) -> Json {
        let mut pasangan = Vec::new();
        pasangan.push(("tipe".to_string(), Json::Teks(self.tipe.clone())));
        pasangan.push(("konten".to_string(), Json::Teks(self.konten.clone())));

        if let (Some(ref kolom), Some(ref baris)) = (&self.kolom_tabel, &self.baris_tabel) {
            let mut data_tbl = Vec::new();
            let k_larik = kolom.iter().map(|k| Json::Teks(k.clone())).collect();
            data_tbl.push(("kolom".to_string(), Json::Larik(k_larik)));

            let b_larik = baris
                .iter()
                .map(|r| Json::Larik(r.iter().map(|v| Json::Teks(v.clone())).collect()))
                .collect();
            data_tbl.push(("baris".to_string(), Json::Larik(b_larik)));

            pasangan.push(("data_tabel".to_string(), Json::Objek(data_tbl)));
        }

        if let Some(ref svg_code) = self.svg {
            pasangan.push(("svg".to_string(), Json::Teks(svg_code.clone())));
        }

        Json::Objek(pasangan)
    }

    pub fn dari_json(json: &Json) -> Result<Self, String> {
        let tipe = json.ambil_teks("tipe").unwrap_or("teks").to_string();
        let konten = json.ambil_teks("konten").unwrap_or("").to_string();

        let mut kolom_tabel = None;
        let mut baris_tabel = None;
        if let Some(Json::Objek(dt)) = json.ambil("data_tabel") {
            let mut k_res = Vec::new();
            let mut b_res = Vec::new();
            for (k, v) in dt {
                if k == "kolom" {
                    if let Json::Larik(kl) = v {
                        for item in kl {
                            if let Some(s) = item.sebagai_teks() {
                                k_res.push(s.to_string());
                            }
                        }
                    }
                } else if k == "baris" {
                    if let Json::Larik(bl) = v {
                        for baris_item in bl {
                            if let Json::Larik(row) = baris_item {
                                let mut r_vec = Vec::new();
                                for cell in row {
                                    r_vec.push(cell.sebagai_teks().unwrap_or("").to_string());
                                }
                                b_res.push(r_vec);
                            }
                        }
                    }
                }
            }
            if !k_res.is_empty() {
                kolom_tabel = Some(k_res);
                baris_tabel = Some(b_res);
            }
        }

        let svg = json.ambil_teks("svg").map(|s| s.to_string());

        Ok(MarchOutput {
            tipe,
            konten,
            baris_tabel,
            kolom_tabel,
            svg,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MarchCell {
    pub id: String,
    pub tipe: String, // "kode" atau "markdown"
    pub urutan_eksekusi: Option<usize>,
    pub sumber: String,
    pub keluaran: Option<MarchOutput>,
}

impl MarchCell {
    pub fn ke_json(&self) -> Json {
        let mut pasangan = Vec::new();
        pasangan.push(("id".to_string(), Json::Teks(self.id.clone())));
        pasangan.push(("tipe".to_string(), Json::Teks(self.tipe.clone())));
        match self.urutan_eksekusi {
            Some(n) => pasangan.push(("urutan_eksekusi".to_string(), Json::Angka(n as f64))),
            None => pasangan.push(("urutan_eksekusi".to_string(), Json::Null)),
        }
        pasangan.push(("sumber".to_string(), Json::Teks(self.sumber.clone())));
        match &self.keluaran {
            Some(out) => pasangan.push(("keluaran".to_string(), out.ke_json())),
            None => pasangan.push(("keluaran".to_string(), Json::Null)),
        }
        Json::Objek(pasangan)
    }

    pub fn dari_json(json: &Json) -> Result<Self, String> {
        let id = json
            .ambil_teks("id")
            .ok_or_else(|| "Sel tidak memiliki 'id'".to_string())?
            .to_string();
        let tipe = json.ambil_teks("tipe").unwrap_or("kode").to_string();
        let urutan_eksekusi = json.ambil_angka("urutan_eksekusi").map(|n| n as usize);
        let sumber = json.ambil_teks("sumber").unwrap_or("").to_string();
        let keluaran = match json.ambil("keluaran") {
            Some(Json::Objek(_)) => json
                .ambil("keluaran")
                .and_then(|k| MarchOutput::dari_json(k).ok()),
            _ => None,
        };

        Ok(MarchCell {
            id,
            tipe,
            urutan_eksekusi,
            sumber,
            keluaran,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MarchMetadata {
    pub penulis: String,
    pub folder_kerja: String,
    pub kernel_nama: String,
    pub kernel_versi: String,
}

impl MarchMetadata {
    pub fn ke_json(&self) -> Json {
        let kernel = Json::Objek(vec![
            ("nama".to_string(), Json::Teks(self.kernel_nama.clone())),
            ("versi".to_string(), Json::Teks(self.kernel_versi.clone())),
        ]);
        Json::Objek(vec![
            ("penulis".to_string(), Json::Teks(self.penulis.clone())),
            (
                "folder_kerja".to_string(),
                Json::Teks(self.folder_kerja.clone()),
            ),
            ("kernel".to_string(), kernel),
        ])
    }

    pub fn dari_json(json: &Json) -> Self {
        let penulis = json
            .ambil_teks("penulis")
            .unwrap_or("Pengguna Evernight")
            .to_string();
        let folder_kerja = json.ambil_teks("folder_kerja").unwrap_or(".").to_string();
        let mut kernel_nama = "evernight".to_string();
        let mut kernel_versi = env!("CARGO_PKG_VERSION").to_string();

        if let Some(Json::Objek(k)) = json.ambil("kernel") {
            for (kunci, val) in k {
                if kunci == "nama" {
                    if let Some(s) = val.sebagai_teks() {
                        kernel_nama = s.to_string();
                    }
                } else if kunci == "versi" {
                    if let Some(s) = val.sebagai_teks() {
                        kernel_versi = s.to_string();
                    }
                }
            }
        }

        MarchMetadata {
            penulis,
            folder_kerja,
            kernel_nama,
            kernel_versi,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MarchNotebook {
    pub march_versi: String,
    pub bahasa: String,
    pub nama_notebook: String,
    pub waktu_dibuat: String,
    pub waktu_diubah: String,
    pub metadata: MarchMetadata,
    pub sel: Vec<MarchCell>,
}

impl MarchNotebook {
    pub fn baru(nama: &str, folder: &str) -> Self {
        let now = "2026-10-28T12:00:00Z".to_string();
        let cells = vec![
            MarchCell {
                id: "sel_01".to_string(),
                tipe: "markdown".to_string(),
                urutan_eksekusi: None,
                sumber: format!(
                    "# {}\nEksplorasi data dan eksperimen Machine Learning dengan **EvernightLanguage** & pustaka **Matra**.",
                    nama
                ),
                keluaran: None,
            },
            MarchCell {
                id: "sel_02".to_string(),
                tipe: "kode".to_string(),
                urutan_eksekusi: None,
                sumber: "impor \"matra\" sebagai mt\n\n# Membuat data matriks contoh\nvariabel X = mt.larik([[1, 2], [3, 4], [5, 6]])\ncetak(\"Bentuk larik:\", X.bentuk)\nX".to_string(),
                keluaran: None,
            },
        ];

        MarchNotebook {
            march_versi: "1.0".to_string(),
            bahasa: "EvernightLanguage".to_string(),
            nama_notebook: nama.to_string(),
            waktu_dibuat: now.clone(),
            waktu_diubah: now,
            metadata: MarchMetadata {
                penulis: "Pengguna Evernight".to_string(),
                folder_kerja: folder.to_string(),
                kernel_nama: "evernight".to_string(),
                kernel_versi: env!("CARGO_PKG_VERSION").to_string(),
            },
            sel: cells,
        }
    }

    pub fn ke_json(&self) -> Json {
        let sel_larik = self.sel.iter().map(|c| c.ke_json()).collect();
        Json::Objek(vec![
            (
                "march_versi".to_string(),
                Json::Teks(self.march_versi.clone()),
            ),
            ("bahasa".to_string(), Json::Teks(self.bahasa.clone())),
            (
                "nama_notebook".to_string(),
                Json::Teks(self.nama_notebook.clone()),
            ),
            (
                "waktu_dibuat".to_string(),
                Json::Teks(self.waktu_dibuat.clone()),
            ),
            (
                "waktu_diubah".to_string(),
                Json::Teks(self.waktu_diubah.clone()),
            ),
            ("metadata".to_string(), self.metadata.ke_json()),
            ("sel".to_string(), Json::Larik(sel_larik)),
        ])
    }

    pub fn dari_json(json: &Json) -> Result<Self, String> {
        let march_versi = json.ambil_teks("march_versi").unwrap_or("1.0").to_string();
        let bahasa = json
            .ambil_teks("bahasa")
            .unwrap_or("EvernightLanguage")
            .to_string();
        let nama_notebook = json
            .ambil_teks("nama_notebook")
            .unwrap_or("Notebook March")
            .to_string();
        let waktu_dibuat = json.ambil_teks("waktu_dibuat").unwrap_or("").to_string();
        let waktu_diubah = json.ambil_teks("waktu_diubah").unwrap_or("").to_string();

        let metadata = match json.ambil("metadata") {
            Some(m) => MarchMetadata::dari_json(m),
            None => MarchMetadata {
                penulis: "Pengguna".to_string(),
                folder_kerja: ".".to_string(),
                kernel_nama: "evernight".to_string(),
                kernel_versi: env!("CARGO_PKG_VERSION").to_string(),
            },
        };

        let mut sel = Vec::new();
        if let Some(Json::Larik(sel_list)) = json.ambil("sel") {
            for c in sel_list {
                if let Ok(item) = MarchCell::dari_json(c) {
                    sel.push(item);
                }
            }
        }

        Ok(MarchNotebook {
            march_versi,
            bahasa,
            nama_notebook,
            waktu_dibuat,
            waktu_diubah,
            metadata,
            sel,
        })
    }

    pub fn muat_dari_berkas(path: &Path) -> Result<Self, String> {
        let raw = fs::read_to_string(path)
            .map_err(|e| format!("Gagal membaca berkas notebook '{:?}': {}", path, e))?;
        let json_val =
            json::parse(&raw).map_err(|e| format!("Gagal mengurai format JSON '.emlbn': {}", e))?;
        Self::dari_json(&json_val)
    }

    pub fn simpan_ke_berkas(&self, path: &Path) -> Result<(), String> {
        let json_val = self.ke_json();
        let teks = json_val.tulis_rapi();
        fs::write(path, teks)
            .map_err(|e| format!("Gagal menyimpan berkas notebook '{:?}': {}", path, e))?;
        Ok(())
    }

    pub fn ekspor_ke_eve(&self) -> String {
        let mut out = String::new();
        out.push_str("# ======================================================================\n");
        out.push_str(&format!("# March Notebook: {}\n", self.nama_notebook));
        out.push_str("# Dihasilkan secara otomatis oleh Evernight March Subsystem\n");
        out.push_str(
            "# ======================================================================\n\n",
        );

        for (i, cell) in self.sel.iter().enumerate() {
            if cell.tipe == "kode" {
                out.push_str(&format!("# --- [Sel {} | {}] ---\n", i + 1, cell.id));
                out.push_str(cell.sumber.trim());
                out.push_str("\n\n");
            } else if cell.tipe == "markdown" {
                out.push_str(&format!("# [Catatan {}]:\n", cell.id));
                for baris in cell.sumber.lines() {
                    out.push_str(&format!("#   {}\n", baris));
                }
                out.push('\n');
            }
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notebook_baru_dan_roundtrip_json() {
        let nb = MarchNotebook::baru("Uji Coba ML", "/tmp/proyek");
        assert_eq!(nb.nama_notebook, "Uji Coba ML");
        assert_eq!(nb.sel.len(), 2);

        let json_val = nb.ke_json();
        let teks_json = json_val.tulis_rapi();
        let parsed = json::parse(&teks_json).expect("Gagal mengurai JSON");
        let nb_reconstructed =
            MarchNotebook::dari_json(&parsed).expect("Gagal merekonstruksi notebook");

        assert_eq!(nb_reconstructed.nama_notebook, "Uji Coba ML");
        assert_eq!(nb_reconstructed.sel.len(), 2);
        assert_eq!(nb_reconstructed.sel[0].tipe, "markdown");
        assert_eq!(nb_reconstructed.sel[1].tipe, "kode");
    }

    #[test]
    fn test_notebook_ekspor_ke_eve() {
        let mut nb = MarchNotebook::baru("Analisis Regresi", ".");
        nb.sel.push(MarchCell {
            id: "sel_03".to_string(),
            tipe: "kode".to_string(),
            urutan_eksekusi: Some(1),
            sumber: "variabel x = 100\ncetak(x)".to_string(),
            keluaran: None,
        });

        let skrip = nb.ekspor_ke_eve();
        assert!(skrip.contains("March Notebook: Analisis Regresi"));
        assert!(skrip.contains("variabel x = 100"));
        assert!(skrip.contains("cetak(x)"));
    }

    #[test]
    fn test_output_rich_serialization() {
        let out_tabel = MarchOutput {
            tipe: "tabel_html".to_string(),
            konten: "DataFrame 2x2".to_string(),
            baris_tabel: Some(vec![
                vec!["1.0".to_string(), "A".to_string()],
                vec!["2.0".to_string(), "B".to_string()],
            ]),
            kolom_tabel: Some(vec!["skor".to_string(), "kelas".to_string()]),
            svg: None,
        };

        let json_val = out_tabel.ke_json();
        let reconstructed = MarchOutput::dari_json(&json_val).expect("Gagal rekonstruksi output");
        assert_eq!(reconstructed.tipe, "tabel_html");
        assert_eq!(reconstructed.kolom_tabel.unwrap(), vec!["skor", "kelas"]);
        assert_eq!(reconstructed.baris_tabel.unwrap().len(), 2);

        let out_svg = MarchOutput {
            tipe: "grafik_svg".to_string(),
            konten: "".to_string(),
            baris_tabel: None,
            kolom_tabel: None,
            svg: Some("<svg viewBox='0 0 100 100'></svg>".to_string()),
        };
        let json_svg = out_svg.ke_json();
        let reconstructed_svg = MarchOutput::dari_json(&json_svg).expect("Gagal rekonstruksi svg");
        assert_eq!(reconstructed_svg.tipe, "grafik_svg");
        assert!(reconstructed_svg.svg.unwrap().contains("<svg"));
    }
}
