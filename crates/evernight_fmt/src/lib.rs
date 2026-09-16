//! Formatter EvernightLanguage (Fase 6D-3).
//!
//! Pendekatan: **berbasis baris sumber + lexer per baris**, bukan AST.
//! Alasan penting: lexer membuang komentar (`lexer.rs`), sehingga formatter
//! berbasis AST akan menghapus semua komentar. Dengan memproses baris demi
//! baris, komentar dipertahankan.
//!
//! Sifat penting: formatter **tidak pernah menggabung atau memecah baris**.
//! Ia hanya menormalkan indentasi + spasi dalam baris, sehingga tidak dapat
//! mengubah makna program.
//!
//! Aturan:
//! - Indentasi 4 spasi, tidak ada tab.
//! - Spasi tunggal di sekitar operator & setelah koma.
//! - Tanpa spasi sebelum `(`, `[`, `)`, `]`, `,`, `.`, `:`.
//! - Maksimum satu baris kosong berurutan.
//! - Tanpa spasi di akhir baris.
//!
//! Formatter bersifat **idempoten**.

use evernight_core::token::TokenType;
use evernight_core::{Lexer, Token};

const INDENT: &str = "    ";

/// Format seluruh sumber.
pub fn format(sumber: &str) -> Result<String, String> {
    // Pertahankan BOM UTF-8 bila ada, agar encoding berkas tidak berubah.
    let (bom, isi) = match sumber.strip_prefix('\u{FEFF}') {
        Some(s) => ("\u{FEFF}", s),
        None => ("", sumber),
    };

    let mut keluaran = String::new();
    let mut indent: usize = 0;
    let mut kosong_beruntun: usize = 0;

    for baris in isi.lines() {
        let isi_baris = baris.trim_end();

        if isi_baris.trim().is_empty() {
            kosong_beruntun += 1;
            if kosong_beruntun == 1 && !keluaran.is_empty() {
                keluaran.push('\n');
            }
            continue;
        }
        kosong_beruntun = 0;

        let (kode, komentar) = pisah_komentar(isi_baris);
        let kode = kode.trim();

        // Baris komentar murni.
        if kode.is_empty() {
            keluaran.push_str(&INDENT.repeat(indent));
            if let Some(k) = komentar {
                keluaran.push_str(k.trim_end());
            }
            keluaran.push('\n');
            continue;
        }

        // Lex baris; bila gagal (mis. lanjutan ekspresi), pakai apa adanya.
        let token = lex_baris(kode);

        // Penutup di awal baris mengurangi indentasi lebih dulu.
        let tutup_awal = token
            .as_ref()
            .map(|t| {
                t.iter()
                    .take_while(|x| matches!(x.tipe, TokenType::KurawalTutup))
                    .count()
            })
            .unwrap_or(0);
        let indent_baris = indent.saturating_sub(tutup_awal);

        keluaran.push_str(&INDENT.repeat(indent_baris));

        match &token {
            Some(t) => keluaran.push_str(&susun_baris(t)),
            None => keluaran.push_str(kode),
        }

        if let Some(k) = komentar {
            let k = k.trim_end();
            if !k.is_empty() {
                keluaran.push_str("  ");
                keluaran.push_str(k);
            }
        }
        keluaran.push('\n');

        // Perbarui kedalaman dari kurung kurawal pada baris ini.
        if let Some(t) = &token {
            for x in t {
                match x.tipe {
                    TokenType::KurawalBuka => indent += 1,
                    TokenType::KurawalTutup => indent = indent.saturating_sub(1),
                    _ => {}
                }
            }
        }
    }

    while keluaran.ends_with("\n\n") {
        keluaran.pop();
    }
    Ok(format!("{}{}", bom, keluaran))
}

/// Periksa apakah sumber sudah terformat.
pub fn sudah_rapi(sumber: &str) -> Result<bool, String> {
    Ok(format(sumber)? == sumber)
}

/// Token ringkas satu baris.
struct T {
    lexeme: String,
    tipe: TokenType,
}

/// Lex satu baris; `None` bila baris bukan potongan kode yang valid.
fn lex_baris(kode: &str) -> Option<Vec<T>> {
    let mut lexer = Lexer::new(kode);
    let token = lexer.scan_tokens().ok()?;
    Some(
        token
            .into_iter()
            .filter(|t| !matches!(t.token_type, TokenType::Eof))
            .map(|t: Token| T {
                lexeme: t.lexeme,
                tipe: t.token_type,
            })
            .collect(),
    )
}

/// Pisahkan kode dari komentar `#` di luar string.
fn pisah_komentar(baris: &str) -> (String, Option<String>) {
    let mut kutip: Option<char> = None;
    let mut lewat = false;
    for (i, c) in baris.char_indices() {
        if lewat {
            lewat = false;
            continue;
        }
        match kutip {
            Some(q) => {
                if c == '\\' {
                    lewat = true;
                } else if c == q {
                    kutip = None;
                }
            }
            None => {
                if c == '"' || c == '\'' {
                    kutip = Some(c);
                } else if c == '#' {
                    return (baris[..i].to_string(), Some(baris[i..].to_string()));
                }
            }
        }
    }
    (baris.to_string(), None)
}

/// Gabungkan token satu baris memakai aturan spasi.
fn susun_baris(token: &[T]) -> String {
    let mut keluar = String::new();
    for (i, t) in token.iter().enumerate() {
        if i > 0 {
            let sebelum = &token[i - 1];
            let sebelum2 = if i >= 2 { Some(&token[i - 2]) } else { None };
            if perlu_spasi(sebelum, t, sebelum2) {
                keluar.push(' ');
            }
        }
        keluar.push_str(&t.lexeme);
    }
    keluar
}

fn perlu_spasi(a: &T, b: &T, sebelum_a: Option<&T>) -> bool {
    use TokenType::*;

    // Tanpa spasi setelah pembuka.
    if matches!(a.tipe, KurungBuka | SikuBuka) {
        return false;
    }
    // Tanpa spasi sebelum penutup / pemisah erat.
    if matches!(
        b.tipe,
        KurungTutup | SikuTutup | Koma | TitikKoma | TitikDua | Titik
    ) {
        return false;
    }
    // Tanpa spasi setelah titik (akses properti / metode).
    if matches!(a.tipe, Titik) {
        return false;
    }
    // Setelah koma selalu spasi.
    if matches!(a.tipe, Koma) {
        return true;
    }
    // Operator unary: tidak ada spasi ke operandnya.
    if matches!(a.tipe, Kurang | Tambah | Seru | Bukan) && unary_context(sebelum_a) {
        return false;
    }
    // Sebelum `(`: spasi hanya untuk kata kunci kontrol.
    if matches!(b.tipe, KurungBuka) {
        return matches!(
            a.tipe,
            Jika | LainnyaJika | Selama | Untuk | Tangkap | Cocok | Pastikan
        );
    }
    // `[` : tanpa spasi bila pengindeksan (setelah identifier/tutup/literal),
    // sebaliknya (mis. `= [`) diberi spasi karena ini literal daftar.
    if matches!(b.tipe, SikuBuka) {
        return !matches!(
            a.tipe,
            Identifier(_) | KurungTutup | SikuTutup | Angka(_) | Teks(_) | Wildcard
        );
    }
    true
}

/// Apakah token sebelum operator menandakan posisi unary.
fn unary_context(t: Option<&T>) -> bool {
    use TokenType::*;
    match t {
        None => true,
        Some(x) => matches!(
            x.tipe,
            KurungBuka
                | SikuBuka
                | KurawalBuka
                | Koma
                | TitikDua
                | Sama
                | TambahSama
                | KurangSama
                | BintangSama
                | GarisMiringSama
                | SamaSama
                | SeruSama
                | KurangDariSimbol
                | KurangSamaSimbol
                | LebihDariSimbol
                | LebihSamaSimbol
                | Tambah
                | Kurang
                | Bintang
                | GarisMiring
                | Persen
                | BintangBintang
                | Dan
                | Atau
                | DanSimbol
                | AtauSimbol
                | Seru
                | Bukan
                | Kembali
                | Lempar
                | Kasus
        ),
    }
}

/// Format satu berkas, kembalikan pesan galat ramah bila gagal.
pub fn format_atau_galat(sumber: &str, nama: &str) -> Result<String, String> {
    format(sumber).map_err(|e| format!("Gagal memformat '{}': {}", nama, e))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn f(s: &str) -> String {
        format(s).unwrap()
    }

    #[test]
    fn idempoten_kode_sederhana() {
        let s = "variabel x = 10\ncetak(x)\n";
        let a = f(s);
        assert_eq!(a, f(&a));
    }

    #[test]
    fn normalkan_spasi_operator() {
        assert_eq!(f("variabel x=1+2\n"), "variabel x = 1 + 2\n");
    }

    #[test]
    fn pertahankan_komentar() {
        let h = f("# catatan\nvariabel x = 1  # ekor\n");
        assert!(h.contains("# catatan"), "hasil: {}", h);
        assert!(h.contains("# ekor"), "hasil: {}", h);
    }

    #[test]
    fn pagar_dalam_string_bukan_komentar() {
        let h = f("cetak(\"a # b\")\n");
        assert!(h.contains("\"a # b\""), "hasil: {}", h);
    }

    #[test]
    fn indentasi_blok() {
        assert_eq!(
            f("jika benar {\ncetak(1)\n}\n"),
            "jika benar {\n    cetak(1)\n}\n"
        );
    }

    #[test]
    fn blok_bersarang() {
        let h = f("fungsi f() {\njika benar {\nkembali 1\n}\n}\n");
        assert_eq!(
            h,
            "fungsi f() {\n    jika benar {\n        kembali 1\n    }\n}\n"
        );
    }

    #[test]
    fn bersihkan_pemanggilan() {
        assert_eq!(f("cetak ( 1 , 2 )\n"), "cetak(1, 2)\n");
    }

    #[test]
    fn satu_baris_kosong_maksimal() {
        assert_eq!(
            f("variabel a = 1\n\n\n\nvariabel b = 2\n"),
            "variabel a = 1\n\nvariabel b = 2\n"
        );
    }

    #[test]
    fn tanpa_spasi_akhir() {
        assert_eq!(f("variabel x = 1   \n"), "variabel x = 1\n");
    }

    #[test]
    fn deteksi_sudah_rapi() {
        assert!(sudah_rapi("variabel x = 1\ncetak(x)\n").unwrap());
        assert!(!sudah_rapi("variabel x=1\n").unwrap());
    }

    #[test]
    fn blok_cocok_kasus() {
        let src = "cocok n {\nkasus 0 {\nkembali \"nol\"\n}\nkasus _ {\nkembali \"lain\"\n}\n}\n";
        let harap = "cocok n {\n    kasus 0 {\n        kembali \"nol\"\n    }\n    kasus _ {\n        kembali \"lain\"\n    }\n}\n";
        assert_eq!(f(src), harap);
    }

    #[test]
    fn unary_minus_tanpa_spasi() {
        assert_eq!(f("variabel x = -1\n"), "variabel x = -1\n");
        assert_eq!(f("variabel y = a - 1\n"), "variabel y = a - 1\n");
    }

    #[test]
    fn lainnya_dan_tangkap_sebaris_penutup() {
        let src = "jika a {\ncetak(1)\n} lainnya {\ncetak(2)\n}\n";
        let harap = "jika a {\n    cetak(1)\n} lainnya {\n    cetak(2)\n}\n";
        assert_eq!(f(src), harap);
    }

    #[test]
    fn akses_properti_tanpa_spasi() {
        assert_eq!(f("cetak ( a . panjang )\n"), "cetak(a.panjang)\n");
    }

    #[test]
    fn bom_utf8_dipertahankan() {
        let s = "\u{FEFF}variabel x=1\ncetak(x)\n";
        let h = format(s).unwrap();
        assert!(h.starts_with('\u{FEFF}'), "BOM hilang: {:?}", h);
        assert!(h.contains("variabel x = 1"), "isi: {:?}", h);
        assert_eq!(h, format(&h).unwrap(), "harus idempoten dengan BOM");
    }
}
