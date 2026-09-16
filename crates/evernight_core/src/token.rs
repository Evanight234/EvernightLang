use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Literal
    Angka(f64),
    Teks(String),
    Benar,
    Salah,
    Kosong,

    // Identifier & Wildcard
    Identifier(String),
    Wildcard, // _

    // Kata Kunci - Deklarasi & Struktur
    Variabel, // variabel
    Tetap,    // tetap
    Fungsi,   // fungsi
    Kembali,  // kembali

    // Percabangan
    Jika,        // jika
    LainnyaJika, // lainnya_jika
    Lainnya,     // lainnya
    Cocok,       // cocok
    Kasus,       // kasus
    Bawaan,      // bawaan

    // Perulangan
    Selama,   // selama
    Untuk,    // untuk
    Dari,     // dari
    Sampai,   // sampai
    Dalam,    // dalam
    Berhenti, // berhenti
    Lanjut,   // lanjut
    Hapus,    // hapus

    // Operator Logika & Perbandingan Berbasis Kata
    Dan,        // dan
    Atau,       // atau
    Bukan,      // bukan
    SamaDengan, // sama_dengan
    LebihDari,  // lebih_dari
    KurangDari, // kurang_dari

    // Penanganan Kesalahan
    Coba,     // coba
    Tangkap,  // tangkap
    Akhirnya, // akhirnya
    Lempar,   // lempar
    Pastikan, // pastikan

    // Modul & I/O
    Impor,   // impor
    Sebagai, // sebagai
    Cetak,   // cetak
    Baca,    // baca

    // Operator Simbol Aritmatika & Pangkat
    Tambah,         // +
    Kurang,         // -
    Bintang,        // *
    GarisMiring,    // /
    Persen,         // %
    BintangBintang, // **

    // Operator Penugasan
    Sama,            // =
    TambahSama,      // +=
    KurangSama,      // -=
    BintangSama,     // *=
    GarisMiringSama, // /=

    // Operator Simbol Perbandingan & Logika
    SamaSama,         // ==
    SeruSama,         // !=
    KurangDariSimbol, // <
    KurangSamaSimbol, // <=
    LebihDariSimbol,  // >
    LebihSamaSimbol,  // >=
    DanSimbol,        // &&
    AtauSimbol,       // ||
    Seru,             // !

    // Tanda Baca & Pembatas
    KurungBuka,   // (
    KurungTutup,  // )
    KurawalBuka,  // {
    KurawalTutup, // }
    SikuBuka,     // [
    SikuTutup,    // ]
    Koma,         // ,
    TitikDua,     // :
    Titik,        // .
    TitikKoma,    // ;

    // Khusus
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: impl Into<String>,
        line: usize,
        column: usize,
    ) -> Self {
        Self {
            token_type,
            lexeme: lexeme.into(),
            line,
            column,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Token({:?}, \"{}\", baris: {}, kolom: {})",
            self.token_type, self.lexeme, self.line, self.column
        )
    }
}
