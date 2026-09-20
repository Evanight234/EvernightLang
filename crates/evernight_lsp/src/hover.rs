use serde_json::{json, Value};

const HOVER_INFO: &[(&str, &str)] = &[
    ("fungsi", "Deklarasi fungsi: `fungsi nama(param) { ... }`"),
    ("variabel", "Deklarasi variabel: `variabel x = nilai`"),
    ("tetap", "Konstanta: `tetap PI = 3.14`"),
    ("jika", "Percabangan: `jika kondisi { ... } lainnya { ... }`"),
    ("lainnya", "Blok else: `jika ... { } lainnya { }`"),
    ("selama", "Loop: `selama kondisi { ... }`"),
    ("untuk", "Loop range: `untuk i dari 1 sampai 10 { ... }`"),
    ("dari", "Batas bawah range: `dari 1 sampai 10`"),
    ("sampai", "Batas atas range: `dari 1 sampai 10`"),
    ("kembali", "Return value: `kembali nilai`"),
    ("cetak", "Output ke konsol: `cetak(\"teks\")`"),
    ("baca", "Input dari konsol: `baca()`"),
    ("benar", "Literal boolean: `benar` / `salah`"),
    ("salah", "Literal boolean: `benar` / `salah`"),
    ("kosong", "Nilai kosong (null)"),
    ("impor", "Impor modul: `impor \"nama_modul\"`"),
    ("sebagai", "Alias impor: `impor \"modul\" sebagai m`"),
    ("coba", "Error handling: `coba { ... } tangkap(e) { ... }`"),
    ("tangkap", "Penangkap error: `tangkap(pesan) { ... }`"),
    ("lempar", "Lempar error: `lempar(\"pesan error\")`"),
    ("dan", "Operator logika AND: `a dan b`"),
    ("atau", "Operator logika OR: `a atau b`"),
    ("bukan", "Operator logika NOT: `bukan kondisi`"),
    ("panjang", "Properti: `\"teks\".panjang` → jumlah karakter"),
    ("tambah", "Tambah elemen: `tambah(daftar, nilai)`"),
    ("besar", "Uppercase: `besar(\"halo\")` → \"HALO\""),
    ("kecil", "Lowercase: `kecil(\"HALO\")` → \"halo\""),
    ("potong", "Substring: `potong(teks, start, end)`"),
    ("pecah", "Split: `pecah(teks, delimiter)` → daftar"),
    ("gabung", "Join: `gabung(daftar, delimiter)` → teks"),
    ("ganti", "Replace: `ganti(teks, lama, baru)`"),
    ("mengandung", "Contains: `mengandung(teks, substring)` → bolean"),
    ("urutkan", "Sort: `urutkan(daftar)` → daftar terurut"),
    ("jumlah", "Sum: `jumlah(daftar)` → angka"),
    ("rata2", "Average: `rata2(daftar)` → angka"),
    ("jenis", "Tipe value: `jenis(x)` → teks"),
    ("salin", "Deep copy: `salin(x)` → salinan"),
    ("ke_teks", "Konversi ke teks: `ke_teks(42)` → \"42\""),
    ("ke_angka", "Konversi ke angka: `ke_angka(\"42\")` → 42"),
    ("waktu_sekarang", "Timestamp millis: `waktu_sekarang()` → angka"),
    ("tunda", "Sleep: `tunda(millis)`"),
    ("baca_file", "Baca file: `baca_file(\"path\")` → teks"),
    ("tulis_file", "Tulis file: `tulis_file(\"path\", \"isi\")`"),
    ("ada_file", "Cek file: `ada_file(\"path\")` → bolean"),
    ("env", "Env var: `env(\"NAMA\", \"default\")` → teks"),
    ("atur_env", "Set env: `atur_env(\"NAMA\", \"nilai\")`"),
];

pub fn hover(text: &str, line: usize, col: usize) -> Option<Value> {
    let current_line = text.lines().nth(line)?;
    let safe_col = col.min(current_line.len());

    let before = &current_line[..safe_col];
    let after = &current_line[safe_col..];

    let word_start = before.rfind(|c: char| !c.is_alphanumeric() && c != '_').map_or(0, |i| i + 1);
    let word_end = after.find(|c: char| !c.is_alphanumeric() && c != '_').unwrap_or(after.len());
    let word = &current_line[word_start..safe_col + word_end];

    if word.is_empty() {
        return None;
    }

    for &(keyword, desc) in HOVER_INFO {
        if keyword == word {
            return Some(json!({
                "contents": {
                    "kind": "markdown",
                    "value": format!("**{}**\n\n{}", keyword, desc)
                }
            }));
        }
    }

    None
}
