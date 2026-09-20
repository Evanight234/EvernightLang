use serde_json::{json, Value};

const KEYWORDS: &[&str] = &[
    "fungsi", "variabel", "tetap", "jika", "lainnya", "selama", "untuk",
    "dari", "sampai", "kembali", "cetak", "baca", "baca_angka", "bersihkan",
    "benar", "salah", "kosong", "impor", "sebagai", "coba", "tangkap", "lempar",
    "dan", "atau", "bukan", "cocok", "kasus", "bawaan", "berhenti", "lanjut",
    "hapus", "akhirnya", "pastikan",
];

const BUILTINS: &[&str] = &[
    "besar", "kecil", "bersih", "potong", "pecah", "gabung", "ganti",
    "mengandung", "mulai_dengan", "akhiri_dengan",
    "tambah", "sisip", "urutkan", "balik", "rata2", "jumlah",
    "cari", "ada", "range",
    "ke_teks", "ke_angka", "ke_bolean", "adalah",
    "jenis", "salin", "panjang",
    "waktu_sekarang", "selisih_waktu", "format_tanggal", "tunda",
    "argumen", "env", "atur_env", "baca_file", "tulis_file", "ada_file",
    "bersihkan",
];

const STDLIB: &[(&str, &[&str])] = &[
    ("math", &["abs", "min", "max", "bulat", "ceil", "floor", "pangkat", "akar", "log", "sin", "cos", "tan", "pi", "e", "acak"]),
    ("string", &["besar", "kecil", "bersih", "potong", "pecah", "gabung", "ganti", "mengandung", "ulang", "format"]),
    ("daftar", &["tambah", "sisip", "hapus", "urutkan", "balik", "cari", "ada", "jumlah", "rata2"]),
    ("kamus", &["kunci", "nilai", "ada_kunci", "hapus"]),
    ("konsol", &["cetak", "bersihkan"]),
    ("utilitas", &["jenis", "salin", "ke_teks", "ke_angka", "adalah"]),
    ("sistem", &["argumen", "env", "atur_env", "baca_file", "tulis_file", "ada_file", "waktu_sekarang"]),
];

pub fn complete(text: &str, line: usize, col: usize) -> Vec<Value> {
    let current_line = text.lines().nth(line).unwrap_or("");
    let safe_col = col.min(current_line.len());
    let prefix = &current_line[..safe_col];

    if let Some(modul) = detect_import_module(text, line) {
        return complete_stdlib_module(&modul);
    }

    if prefix.ends_with('.') {
        return complete_property();
    }

    let mut items: Vec<Value> = Vec::new();

    for kw in KEYWORDS {
        items.push(json!({
            "label": kw,
            "kind": 14,
            "detail": "keyword"
        }));
    }

    for bi in BUILTINS {
        items.push(json!({
            "label": bi,
            "kind": 3,
            "detail": "built-in"
        }));
    }

    if !prefix.is_empty() {
        let last_word = prefix.split_whitespace().last().unwrap_or("");
        if !last_word.is_empty() {
            items.retain(|item| {
                item.get("label")
                    .and_then(|l| l.as_str())
                    .map(|l| l.starts_with(last_word))
                    .unwrap_or(false)
            });
        }
    }

    items
}

fn detect_import_module(text: &str, line: usize) -> Option<String> {
    let prev_line = if line > 0 {
        text.lines().nth(line - 1)?
    } else {
        text.lines().nth(line)?
    };
    let trimmed = prev_line.trim();
    if trimmed.starts_with("impor") {
        if let Some(start) = trimmed.find('"') {
            if let Some(end) = trimmed[start + 1..].find('"') {
                return Some(trimmed[start + 1..start + 1 + end].to_string());
            }
        }
    }
    None
}

fn complete_stdlib_module(modul: &str) -> Vec<Value> {
    for &(name, funcs) in STDLIB {
        if name == modul {
            return funcs
                .iter()
                .map(|f| {
                    json!({
                        "label": format!("{}.{}", modul, f),
                        "kind": 6,
                        "detail": format!("stdlib/{}", modul)
                    })
                })
                .collect();
        }
    }
    Vec::new()
}

fn complete_property() -> Vec<Value> {
    vec![json!({
        "label": "panjang",
        "kind": 10,
        "detail": "property"
    })]
}
