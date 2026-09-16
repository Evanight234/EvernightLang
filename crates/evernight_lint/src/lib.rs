//! Linter EvernightLanguage (Fase 6D-4).
//!
//! Analisis statis berbasis AST, terpisah dari compiler. Melengkapi warning
//! compiler (`WKVAR`, `WKREACH`, `WKFUNG`) dengan aturan gaya & kerapian.
//!
//! Kode aturan:
//! | Kode       | Aturan                                             |
//! |------------|----------------------------------------------------|
//! | `WKHURUF`  | Nama variabel/fungsi/parameter bukan `snake_case`  |
//! | `WKIMPOR`  | `impor` tidak pernah dipakai                       |
//! | `WKPANJANG`| Fungsi terlalu panjang (> 50 baris)                |
//! | `WKPARAM`  | Fungsi terlalu banyak parameter (> 4)              |
//! | `WKSARANG` | Blok `jika`/`selama`/`untuk` kosong                |
//! | `WKMATI`   | Kode setelah `kembali`/`lempar` tidak terjangkau   |
//! | `WKMAGIS`  | Angka literal "magic" (heuristik)                  |

use evernight_core::ast::{Block, Expr, LiteralValue, Statement};
use evernight_core::{EvernightError, Lexer, Parser};

/// Batas panjang badan fungsi (baris) sebelum `WKPANJANG`.
pub const MAKS_BARIS_FUNGSI: usize = 50;
/// Batas jumlah parameter sebelum `WKPARAM`.
pub const MAKS_PARAM: usize = 4;

/// Hasil lint: daftar peringatan (terurut menaik berdasarkan baris).
#[derive(Debug, Clone, Default)]
pub struct HasilLint {
    pub peringatan: Vec<EvernightError>,
}

impl HasilLint {
    pub fn bersih(&self) -> bool {
        self.peringatan.is_empty()
    }
}

/// Lint sumber `.eve`. `Err` bila gagal parse.
pub fn lint(sumber: &str) -> Result<HasilLint, EvernightError> {
    let mut lexer = Lexer::new(sumber);
    let tokens = lexer.scan_tokens()?;
    let mut parser = Parser::new(tokens);
    let program = parser.parse()?;

    let mut linter = Linter {
        hasil: HasilLint::default(),
    };
    linter.periksa_daftar(&program);
    linter
        .hasil
        .peringatan
        .sort_by_key(|p| (p.line, p.code.clone()));
    Ok(linter.hasil)
}

struct Linter {
    hasil: HasilLint,
}

impl Linter {
    fn lapor(&mut self, kode: &str, line: usize, pesan: String) {
        self.hasil
            .peringatan
            .push(EvernightError::peringatan(kode, line, 0, pesan));
    }

    // ---------------------------------------------------------------
    // Struktur
    // ---------------------------------------------------------------

    fn periksa_daftar(&mut self, stmt: &[Statement]) {
        // Kumpulkan nama impor untuk cek pemakaian.
        let mut impor: Vec<(String, usize)> = Vec::new();
        for s in stmt {
            if let Statement::ImportStatement { module, line, .. } = s {
                impor.push((module.clone(), *line));
            }
        }

        let mut dipakai: Vec<String> = Vec::new();
        for s in stmt {
            self.periksa_stmt(s, &mut dipakai);
        }

        // `WKIMPOR`: impor yang tidak pernah direferensikan.
        for (modul, line) in impor {
            let akar = modul
                .rsplit(['/', '\\'])
                .next()
                .unwrap_or(&modul)
                .trim_end_matches(".eve")
                .to_string();
            let terpakai = dipakai
                .iter()
                .any(|n| n == &akar || n.starts_with(&format!("{}.", akar)));
            if !terpakai {
                self.lapor(
                    "WKIMPOR",
                    line,
                    format!("Modul '{}' diimpor tetapi tidak dipakai", akar),
                );
            }
        }
    }

    fn periksa_stmt(&mut self, stmt: &Statement, dipakai: &mut Vec<String>) {
        match stmt {
            Statement::VarDecl {
                name, init, line, ..
            } => {
                self.cek_nama(name, "variabel", *line);
                if let Some(e) = init {
                    self.periksa_expr(e, dipakai);
                }
            }
            Statement::FuncDecl {
                name,
                params,
                body,
                line,
            } => {
                self.cek_nama(name, "fungsi", *line);
                if params.len() > MAKS_PARAM {
                    self.lapor(
                        "WKPARAM",
                        *line,
                        format!(
                            "Fungsi '{}' memiliki {} parameter (disarankan maks {}); pertimbangkan objek",
                            name,
                            params.len(),
                            MAKS_PARAM
                        ),
                    );
                }
                for p in params {
                    self.cek_nama(&p.name, "parameter", *line);
                }
                let panjang = hitung_baris(body);
                if panjang > MAKS_BARIS_FUNGSI {
                    self.lapor(
                        "WKPANJANG",
                        *line,
                        format!(
                            "Fungsi '{}' panjangnya {} baris (disarankan maks {}); pertimbangkan dipecah",
                            name, panjang, MAKS_BARIS_FUNGSI
                        ),
                    );
                }
                self.periksa_blok(body, dipakai);
            }
            Statement::IfStatement {
                condition,
                then_branch,
                elif_branches,
                else_branch,
                line: _,
            } => {
                self.periksa_expr(condition, dipakai);
                self.cek_blok_kosong(then_branch, "jika");
                self.periksa_blok(then_branch, dipakai);
                for (c, b) in elif_branches {
                    self.periksa_expr(c, dipakai);
                    self.cek_blok_kosong(b, "lainnya_jika");
                    self.periksa_blok(b, dipakai);
                }
                if let Some(b) = else_branch {
                    self.cek_blok_kosong(b, "lainnya");
                    self.periksa_blok(b, dipakai);
                }
            }
            Statement::MatchStatement {
                expr,
                cases,
                wildcard,
                default,
                ..
            } => {
                self.periksa_expr(expr, dipakai);
                for (c, b) in cases {
                    self.periksa_expr(c, dipakai);
                    self.periksa_blok(b, dipakai);
                }
                if let Some(b) = wildcard {
                    self.periksa_blok(b, dipakai);
                }
                if let Some(b) = default {
                    self.periksa_blok(b, dipakai);
                }
            }
            Statement::WhileStatement {
                condition,
                body,
                line: _,
            } => {
                self.periksa_expr(condition, dipakai);
                self.cek_blok_kosong(body, "selama");
                self.periksa_blok(body, dipakai);
            }
            Statement::ForStatement {
                var_name,
                iter,
                body,
                line,
            } => {
                self.cek_nama(var_name, "variabel perulangan", *line);
                match iter {
                    evernight_core::ast::ForIter::Range { start, end } => {
                        self.periksa_expr(start, dipakai);
                        self.periksa_expr(end, dipakai);
                    }
                    evernight_core::ast::ForIter::Collection(e) => {
                        self.periksa_expr(e, dipakai);
                    }
                }
                self.cek_blok_kosong(body, "untuk");
                self.periksa_blok(body, dipakai);
            }
            Statement::ReturnStatement { value: Some(e), .. } => {
                self.periksa_expr(e, dipakai);
            }
            Statement::HapusStatement { target, .. } => self.periksa_expr(target, dipakai),
            Statement::TryCatchStatement {
                try_block,
                catch_var,
                catch_block,
                finally_block,
                line,
            } => {
                self.cek_nama(catch_var, "variabel tangkap", *line);
                self.periksa_blok(try_block, dipakai);
                self.periksa_blok(catch_block, dipakai);
                if let Some(b) = finally_block {
                    self.periksa_blok(b, dipakai);
                }
            }
            Statement::ThrowStatement { expr, .. } => self.periksa_expr(expr, dipakai),
            Statement::AssertStatement {
                condition, message, ..
            } => {
                self.periksa_expr(condition, dipakai);
                if let Some(m) = message {
                    self.periksa_expr(m, dipakai);
                }
            }
            Statement::ImportStatement { .. } => { /* ditangani di periksa_daftar */ }
            Statement::ExprStatement { expr, .. } => self.periksa_expr(expr, dipakai),
            Statement::Block(b) => self.periksa_blok(b, dipakai),
            _ => {}
        }
    }

    fn periksa_blok(&mut self, blok: &Block, dipakai: &mut Vec<String>) {
        // `WKMATI`: kode setelah kembali/lempar dalam blok yang sama.
        let mut terhenti: Option<usize> = None;
        for (i, s) in blok.statements.iter().enumerate() {
            if let Some(l) = terhenti {
                if !matches!(
                    s,
                    Statement::ReturnStatement { .. }
                        | Statement::ThrowStatement { .. }
                        | Statement::BreakStatement { .. }
                        | Statement::ContinueStatement { .. }
                ) {
                    self.lapor(
                        "WKMATI",
                        l,
                        "Baris ini tidak akan pernah dijalankan (setelah 'kembali'/'lempar')"
                            .to_string(),
                    );
                    terhenti = None;
                }
                let _ = i;
                continue;
            }
            if let Statement::ReturnStatement { line, .. }
            | Statement::ThrowStatement { line, .. } = s
            {
                terhenti = Some(*line);
            }
        }

        for s in &blok.statements {
            self.periksa_stmt(s, dipakai);
        }
    }

    fn periksa_expr(&mut self, expr: &Expr, dipakai: &mut Vec<String>) {
        match expr {
            Expr::Literal { value, line } => {
                // `WKMAGIS`: angka bulat selain -1..10 dianggap magic.
                if let LiteralValue::Angka(n) = value {
                    let bulat = n.fract() == 0.0;
                    let kecil = (-1.0..=10.0).contains(n);
                    if bulat && !kecil {
                        self.lapor(
                            "WKMAGIS",
                            *line,
                            format!(
                                "Angka {} tampak 'magic'; pertimbangkan menyimpannya di variabel bernama",
                                *n as i64
                            ),
                        );
                    }
                }
            }
            Expr::Identifier { name, line: _ } => dipakai.push(name.clone()),
            Expr::Binary { left, right, .. } => {
                self.periksa_expr(left, dipakai);
                self.periksa_expr(right, dipakai);
            }
            Expr::Unary { expr, .. } => self.periksa_expr(expr, dipakai),
            Expr::Assign { target, value, .. } => {
                self.periksa_expr(target, dipakai);
                self.periksa_expr(value, dipakai);
            }
            Expr::Call { callee, args, line } => {
                if let Expr::Identifier { name, .. } = callee.as_ref() {
                    if is_kata_kunci_kontrol(name) {
                        self.lapor(
                            "WKHURUF",
                            *line,
                            format!(
                                "'{}' adalah kata kunci; tidak dapat dipanggil seperti fungsi",
                                name
                            ),
                        );
                    }
                }
                self.periksa_expr(callee, dipakai);
                for a in args {
                    self.periksa_expr(a, dipakai);
                }
            }
            Expr::Index {
                target,
                index,
                end,
                step,
                ..
            } => {
                self.periksa_expr(target, dipakai);
                self.periksa_expr(index, dipakai);
                if let Some(e) = end {
                    self.periksa_expr(e, dipakai);
                }
                if let Some(s) = step {
                    self.periksa_expr(s, dipakai);
                }
            }
            Expr::Property { target, name, .. } => {
                dipakai.push(name.clone());
                self.periksa_expr(target, dipakai);
            }
            Expr::ArrayLiteral { elements, .. } => {
                for e in elements {
                    self.periksa_expr(e, dipakai);
                }
            }
            Expr::DictLiteral { entries, .. } => {
                for (k, v) in entries {
                    self.periksa_expr(k, dipakai);
                    self.periksa_expr(v, dipakai);
                }
            }
            Expr::FuncExpr { params, body, line } => {
                if params.len() > MAKS_PARAM {
                    self.lapor(
                        "WKPARAM",
                        *line,
                        format!(
                            "Fungsi anonim memiliki {} parameter (disarankan maks {})",
                            params.len(),
                            MAKS_PARAM
                        ),
                    );
                }
                for p in params {
                    self.cek_nama(&p.name, "parameter", *line);
                }
                self.periksa_blok(body, dipakai);
            }
        }
    }

    fn cek_blok_kosong(&mut self, blok: &Block, jenis: &str) {
        if blok.statements.is_empty() {
            self.lapor(
                "WKSARANG",
                blok.line,
                format!("Blok '{}' kosong; hapus atau isi", jenis),
            );
        }
    }

    /// `WKHURUF`: nama harus `snake_case` (huruf kecil, angka, garis bawah).
    fn cek_nama(&mut self, nama: &str, jenis: &str, line: usize) {
        if nama.is_empty() || nama == "_" {
            return;
        }
        let valid = nama
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
            && !nama.starts_with(|c: char| c.is_ascii_digit());
        if !valid {
            let saran = ke_snake_case(nama);
            self.lapor(
                "WKHURUF",
                line,
                format!(
                    "Nama {} '{}' bukan snake_case; disarankan '{}'",
                    jenis, nama, saran
                ),
            );
        }
    }
}

/// Ubah nama gaya apa pun menjadi `snake_case`.
fn ke_snake_case(nama: &str) -> String {
    let mut keluar = String::new();
    for (i, c) in nama.chars().enumerate() {
        if c.is_ascii_uppercase() {
            if i > 0 && !keluar.ends_with('_') {
                keluar.push('_');
            }
            keluar.push(c.to_ascii_lowercase());
        } else if c == '-' {
            keluar.push('_');
        } else {
            keluar.push(c);
        }
    }
    keluar
}

/// Perkiraan jumlah baris badan fungsi (dari baris awal sampai akhir - awal).
fn hitung_baris(blok: &Block) -> usize {
    let akhir = baris_terakhir_blok(blok);
    akhir.saturating_sub(blok.line) + 1
}

fn baris_terakhir_blok(blok: &Block) -> usize {
    let mut maks = blok.line;
    for s in &blok.statements {
        maks = maks.max(baris_terakhir_stmt(s));
    }
    maks
}

fn baris_terakhir_stmt(stmt: &Statement) -> usize {
    match stmt {
        Statement::VarDecl { line, .. }
        | Statement::FuncDecl { line, .. }
        | Statement::IfStatement { line, .. }
        | Statement::MatchStatement { line, .. }
        | Statement::WhileStatement { line, .. }
        | Statement::ForStatement { line, .. }
        | Statement::ReturnStatement { line, .. }
        | Statement::BreakStatement { line, .. }
        | Statement::ContinueStatement { line, .. }
        | Statement::HapusStatement { line, .. }
        | Statement::TryCatchStatement { line, .. }
        | Statement::ThrowStatement { line, .. }
        | Statement::AssertStatement { line, .. }
        | Statement::ImportStatement { line, .. }
        | Statement::ExprStatement { line, .. } => *line,
        Statement::Block(b) => baris_terakhir_blok(b),
    }
}

fn is_kata_kunci_kontrol(nama: &str) -> bool {
    matches!(
        nama,
        "jika"
            | "lainnya"
            | "lainnya_jika"
            | "selama"
            | "untuk"
            | "kembali"
            | "berhenti"
            | "lanjut"
            | "benar"
            | "salah"
            | "kosong"
            | "dan"
            | "atau"
            | "bukan"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kode(h: &HasilLint, k: &str) -> bool {
        h.peringatan.iter().any(|p| p.code == k)
    }

    #[test]
    fn deteksi_nama_bukan_snake_case() {
        let h = lint("variabel namaSaya = 1\ncetak(namaSaya)\n").unwrap();
        assert!(kode(&h, "WKHURUF"), "{:?}", h.peringatan);
    }

    #[test]
    fn nama_snake_case_lolos() {
        let h = lint("variabel nama_saya = 1\ncetak(nama_saya)\n").unwrap();
        assert!(!kode(&h, "WKHURUF"));
    }

    #[test]
    fn deteksi_impor_tak_dipakai() {
        let h = lint("impor string\ncetak(1)\n").unwrap();
        assert!(kode(&h, "WKIMPOR"), "{:?}", h.peringatan);
    }

    #[test]
    fn impor_dipakai_lolos() {
        let h = lint("impor string\ncetak(string.besar(\"a\"))\n").unwrap();
        assert!(!kode(&h, "WKIMPOR"), "{:?}", h.peringatan);
    }

    #[test]
    fn deteksi_terlalu_banyak_parameter() {
        let h = lint("fungsi f(a, b, c, d, e) {\n    kembali a\n}\ncetak(f(1,2,3,4,5))\n").unwrap();
        assert!(kode(&h, "WKPARAM"), "{:?}", h.peringatan);
    }

    #[test]
    fn deteksi_blok_kosong() {
        let h = lint("jika benar {\n}\ncetak(1)\n").unwrap();
        assert!(kode(&h, "WKSARANG"), "{:?}", h.peringatan);
    }

    #[test]
    fn deteksi_kode_mati() {
        let h = lint("fungsi f() {\n    kembali 1\n    cetak(2)\n}\ncetak(f())\n").unwrap();
        assert!(kode(&h, "WKMATI"), "{:?}", h.peringatan);
    }

    #[test]
    fn deteksi_angka_magis() {
        let h = lint("cetak(42)\n").unwrap();
        assert!(kode(&h, "WKMAGIS"), "{:?}", h.peringatan);
    }

    #[test]
    fn angka_kecil_bukan_magis() {
        let h = lint("cetak(3)\n").unwrap();
        assert!(!kode(&h, "WKMAGIS"));
    }

    #[test]
    fn kode_bersih_tanpa_peringatan() {
        let src = "fungsi tambah(a, b) {\n    kembali a + b\n}\ncetak(tambah(1, 2))\n";
        let h = lint(src).unwrap();
        assert!(h.bersih(), "{:?}", h.peringatan);
    }

    #[test]
    fn galat_parse_dikembalikan() {
        let r = lint("variabel 123 = 1\n");
        assert!(r.is_err());
    }
}
