use crate::value::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum OpCode {
    // Konstanta & Literal
    Konstanta = 0x00, // operand: index konstanta (u16)

    // Stack manipulation
    Pop = 0x01,
    Dup = 0x02,

    // Variabel lokal (slot index u16)
    AmbilLokal = 0x10,
    SimpanLokal = 0x11,
    AmbilGlobal = 0x12,
    SimpanGlobal = 0x13,

    // Variabel global (nama string di konstanta)
    AmbilGlobalNama = 0x14,
    SimpanGlobalNama = 0x15,

    // Aritmatika
    Tambah = 0x20,
    Kurang = 0x21,
    Kali = 0x22,
    Bagi = 0x23,
    Modulo = 0x24,
    Pangkat = 0x25,
    Negatif = 0x26,

    // Perbandingan
    SamaDengan = 0x30,
    TidakSama = 0x31,
    KurangDari = 0x32,
    KurangSama = 0x33,
    LebihDari = 0x34,
    LebihSama = 0x35,
    Dalam = 0x36, // 'dalam' operator

    // Logika
    Dan = 0x40,
    Atau = 0x41,
    Bukan = 0x42,

    // Kontrol alur - Lompatan
    Lompat = 0x50,          // operand: offset relatif (i16)
    LompatJikaSalah = 0x51, // operand: offset relatif (i16)
    LompatJikaBenar = 0x52, // operand: offset relatif (i16)
    Loop = 0x53,            // operand: offset loop (i16)

    // Fungsi
    Panggil = 0x60, // operand: jumlah argumen (u8)
    Kembali = 0x61,

    // Daftar / Array
    BuatDaftar = 0x70, // operand: jumlah elemen (u16)
    AmbilIndeks = 0x71,
    SimpanIndeks = 0x72,
    PanjangDaftar = 0x73,
    TambahDaftar = 0x74, // method .tambah()
    HapusIndeks = 0x75,  // hapus a[i]

    // Kamus / Dictionary
    BuatKamus = 0x80, // operand: jumlah pasangan (u16)
    AmbilKunci = 0x81,
    SimpanKunci = 0x82,
    HapusKunci = 0x83, // hapus k["x"] / hapus k.nama

    // Builtin functions
    Cetak = 0x90,     // operand: jumlah argumen (u8)
    Baca = 0x91,      // operand: jumlah argumen (u8 — prompt opsional)
    BacaAngka = 0x92, // operand: jumlah argumen (u8 — prompt opsional)
    Bersihkan = 0x93,
    PanggilBuiltin = 0x94, // operand: id builtin (u8), jumlah argumen (u8)
    ImporModul = 0x95,     // pop module path (Teks), push exports (Kamus)

    // Error handling
    Llempar = 0xA0,     // operand: konstanta pesan (u16)
    Lempar = 0xA1,      // ambil pesan dari stack (pop)
    Pastikan = 0xA2,    // operand: ada/tidak pesan (1/0xFFFF); pop kondisi
    BuatHandler = 0xA3, // operand: offset ke blok tangkap (i16)
    PopHandler = 0xA4,

    // Akhir
    Henti = 0xFF,
}

impl OpCode {
    pub fn from_u8(byte: u8) -> Option<Self> {
        match byte {
            0x00 => Some(OpCode::Konstanta),
            0x01 => Some(OpCode::Pop),
            0x02 => Some(OpCode::Dup),
            0x10 => Some(OpCode::AmbilLokal),
            0x11 => Some(OpCode::SimpanLokal),
            0x12 => Some(OpCode::AmbilGlobal),
            0x13 => Some(OpCode::SimpanGlobal),
            0x14 => Some(OpCode::AmbilGlobalNama),
            0x15 => Some(OpCode::SimpanGlobalNama),
            0x20 => Some(OpCode::Tambah),
            0x21 => Some(OpCode::Kurang),
            0x22 => Some(OpCode::Kali),
            0x23 => Some(OpCode::Bagi),
            0x24 => Some(OpCode::Modulo),
            0x25 => Some(OpCode::Pangkat),
            0x26 => Some(OpCode::Negatif),
            0x30 => Some(OpCode::SamaDengan),
            0x31 => Some(OpCode::TidakSama),
            0x32 => Some(OpCode::KurangDari),
            0x33 => Some(OpCode::KurangSama),
            0x34 => Some(OpCode::LebihDari),
            0x35 => Some(OpCode::LebihSama),
            0x36 => Some(OpCode::Dalam),
            0x40 => Some(OpCode::Dan),
            0x41 => Some(OpCode::Atau),
            0x42 => Some(OpCode::Bukan),
            0x50 => Some(OpCode::Lompat),
            0x51 => Some(OpCode::LompatJikaSalah),
            0x52 => Some(OpCode::LompatJikaBenar),
            0x53 => Some(OpCode::Loop),
            0x60 => Some(OpCode::Panggil),
            0x61 => Some(OpCode::Kembali),
            0x70 => Some(OpCode::BuatDaftar),
            0x71 => Some(OpCode::AmbilIndeks),
            0x72 => Some(OpCode::SimpanIndeks),
            0x73 => Some(OpCode::PanjangDaftar),
            0x74 => Some(OpCode::TambahDaftar),
            0x75 => Some(OpCode::HapusIndeks),
            0x80 => Some(OpCode::BuatKamus),
            0x81 => Some(OpCode::AmbilKunci),
            0x82 => Some(OpCode::SimpanKunci),
            0x83 => Some(OpCode::HapusKunci),
            0x90 => Some(OpCode::Cetak),
            0x91 => Some(OpCode::Baca),
            0x92 => Some(OpCode::BacaAngka),
            0x93 => Some(OpCode::Bersihkan),
            0x94 => Some(OpCode::PanggilBuiltin),
            0x95 => Some(OpCode::ImporModul),
            0xA0 => Some(OpCode::Llempar),
            0xA1 => Some(OpCode::Lempar),
            0xA2 => Some(OpCode::Pastikan),
            0xA3 => Some(OpCode::BuatHandler),
            0xA4 => Some(OpCode::PopHandler),
            0xFF => Some(OpCode::Henti),
            _ => None,
        }
    }

    /// Nama opcode yang dapat dibaca manusia (untuk trace debug & profiler).
    pub fn name(self) -> &'static str {
        match self {
            OpCode::Konstanta => "Konstanta",
            OpCode::Pop => "Pop",
            OpCode::Dup => "Dup",
            OpCode::AmbilLokal => "AmbilLokal",
            OpCode::SimpanLokal => "SimpanLokal",
            OpCode::AmbilGlobal => "AmbilGlobal",
            OpCode::SimpanGlobal => "SimpanGlobal",
            OpCode::AmbilGlobalNama => "AmbilGlobalNama",
            OpCode::SimpanGlobalNama => "SimpanGlobalNama",
            OpCode::Tambah => "Tambah",
            OpCode::Kurang => "Kurang",
            OpCode::Kali => "Kali",
            OpCode::Bagi => "Bagi",
            OpCode::Modulo => "Modulo",
            OpCode::Pangkat => "Pangkat",
            OpCode::Negatif => "Negatif",
            OpCode::SamaDengan => "SamaDengan",
            OpCode::TidakSama => "TidakSama",
            OpCode::KurangDari => "KurangDari",
            OpCode::KurangSama => "KurangSama",
            OpCode::LebihDari => "LebihDari",
            OpCode::LebihSama => "LebihSama",
            OpCode::Dalam => "Dalam",
            OpCode::Dan => "Dan",
            OpCode::Atau => "Atau",
            OpCode::Bukan => "Bukan",
            OpCode::Lompat => "Lompat",
            OpCode::LompatJikaSalah => "LompatJikaSalah",
            OpCode::LompatJikaBenar => "LompatJikaBenar",
            OpCode::Loop => "Loop",
            OpCode::Panggil => "Panggil",
            OpCode::Kembali => "Kembali",
            OpCode::BuatDaftar => "BuatDaftar",
            OpCode::AmbilIndeks => "AmbilIndeks",
            OpCode::SimpanIndeks => "SimpanIndeks",
            OpCode::PanjangDaftar => "PanjangDaftar",
            OpCode::TambahDaftar => "TambahDaftar",
            OpCode::HapusIndeks => "HapusIndeks",
            OpCode::BuatKamus => "BuatKamus",
            OpCode::AmbilKunci => "AmbilKunci",
            OpCode::SimpanKunci => "SimpanKunci",
            OpCode::HapusKunci => "HapusKunci",
            OpCode::Cetak => "Cetak",
            OpCode::Baca => "Baca",
            OpCode::BacaAngka => "BacaAngka",
            OpCode::Bersihkan => "Bersihkan",
            OpCode::PanggilBuiltin => "PanggilBuiltin",
            OpCode::ImporModul => "ImporModul",
            OpCode::Llempar => "Llempar",
            OpCode::Lempar => "Lempar",
            OpCode::Pastikan => "Pastikan",
            OpCode::BuatHandler => "BuatHandler",
            OpCode::PopHandler => "PopHandler",
            OpCode::Henti => "Henti",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub lines: Vec<usize>,
    pub constants: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            lines: Vec::new(),
            constants: Vec::new(),
        }
    }

    pub fn write(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn write_u16(&mut self, value: u16, line: usize) {
        self.code.push((value >> 8) as u8);
        self.code.push((value & 0xFF) as u8);
        self.lines.push(line);
        self.lines.push(line);
    }

    pub fn write_i16(&mut self, value: i16, line: usize) {
        let bytes = value.to_be_bytes();
        self.code.push(bytes[0]);
        self.code.push(bytes[1]);
        self.lines.push(line);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: Value) -> u16 {
        self.constants.push(value);
        (self.constants.len() - 1) as u16
    }

    pub fn code_len(&self) -> usize {
        self.code.len()
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Self::new()
    }
}
