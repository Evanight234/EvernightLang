//! Parser JSON minimal (zero-dep) untuk manifest proyek `eve.json` (Fase 6D-6).
//!
//! Mendukung subset JSON yang dipakai manifest: objek, larik, teks, angka,
//! boolean, dan null. Cukup untuk membaca/menulis berkas konfigurasi proyek
//! tanpa menambah dependensi eksternal.

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Json {
    Null,
    Bolean(bool),
    Angka(f64),
    Teks(String),
    Larik(Vec<Json>),
    Objek(Vec<(String, Json)>),
}

impl Json {
    /// Ambil nilai berdasarkan kunci (hanya untuk objek).
    pub fn ambil(&self, kunci: &str) -> Option<&Json> {
        match self {
            Json::Objek(pasangan) => pasangan.iter().find(|(k, _)| k == kunci).map(|(_, v)| v),
            _ => None,
        }
    }

    pub fn sebagai_teks(&self) -> Option<&str> {
        match self {
            Json::Teks(s) => Some(s),
            _ => None,
        }
    }

    pub fn sebagai_larik(&self) -> Option<&[Json]> {
        match self {
            Json::Larik(v) => Some(v),
            _ => None,
        }
    }

    /// Serialisasi rapi (2 spasi) untuk berkas manifest.
    pub fn tulis_rapi(&self) -> String {
        let mut keluar = String::new();
        self.tulis(&mut keluar, 0);
        keluar.push('\n');
        keluar
    }

    fn tulis(&self, keluar: &mut String, indent: usize) {
        let spasi = "  ".repeat(indent);
        let spasi_dalam = "  ".repeat(indent + 1);
        match self {
            Json::Null => keluar.push_str("null"),
            Json::Bolean(b) => keluar.push_str(if *b { "true" } else { "false" }),
            Json::Angka(n) => {
                if n.fract() == 0.0 {
                    keluar.push_str(&format!("{}", *n as i64));
                } else {
                    keluar.push_str(&format!("{}", n));
                }
            }
            Json::Teks(s) => keluar.push_str(&kutip(s)),
            Json::Larik(v) => {
                if v.is_empty() {
                    keluar.push_str("[]");
                    return;
                }
                keluar.push_str("[\n");
                for (i, item) in v.iter().enumerate() {
                    keluar.push_str(&spasi_dalam);
                    item.tulis(keluar, indent + 1);
                    if i + 1 < v.len() {
                        keluar.push(',');
                    }
                    keluar.push('\n');
                }
                keluar.push_str(&spasi);
                keluar.push(']');
            }
            Json::Objek(pasangan) => {
                if pasangan.is_empty() {
                    keluar.push_str("{}");
                    return;
                }
                keluar.push_str("{\n");
                for (i, (k, v)) in pasangan.iter().enumerate() {
                    keluar.push_str(&spasi_dalam);
                    keluar.push_str(&kutip(k));
                    keluar.push_str(": ");
                    v.tulis(keluar, indent + 1);
                    if i + 1 < pasangan.len() {
                        keluar.push(',');
                    }
                    keluar.push('\n');
                }
                keluar.push_str(&spasi);
                keluar.push('}');
            }
        }
    }
}

impl fmt::Display for Json {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.tulis_rapi().trim_end())
    }
}

fn kutip(s: &str) -> String {
    let mut keluar = String::with_capacity(s.len() + 2);
    keluar.push('"');
    for c in s.chars() {
        match c {
            '"' => keluar.push_str("\\\""),
            '\\' => keluar.push_str("\\\\"),
            '\n' => keluar.push_str("\\n"),
            '\r' => keluar.push_str("\\r"),
            '\t' => keluar.push_str("\\t"),
            c => keluar.push(c),
        }
    }
    keluar.push('"');
    keluar
}

/// Parse teks JSON menjadi `Json`.
pub fn parse(s: &str) -> Result<Json, String> {
    let mut p = ParserJson {
        byte: s.as_bytes(),
        pos: 0,
    };
    p.lewati_spasi();
    let nilai = p.nilai()?;
    p.lewati_spasi();
    if p.pos != p.byte.len() {
        return Err(format!("Data berlebih setelah JSON pada posisi {}", p.pos));
    }
    Ok(nilai)
}

struct ParserJson<'a> {
    byte: &'a [u8],
    pos: usize,
}

impl<'a> ParserJson<'a> {
    fn lewati_spasi(&mut self) {
        while self.pos < self.byte.len() && self.byte[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }
    }

    fn lihat(&self) -> Option<u8> {
        self.byte.get(self.pos).copied()
    }

    fn nilai(&mut self) -> Result<Json, String> {
        match self.lihat() {
            Some(b'{') => self.objek(),
            Some(b'[') => self.larik(),
            Some(b'"') => Ok(Json::Teks(self.teks()?)),
            Some(b't') => self.literal("true", Json::Bolean(true)),
            Some(b'f') => self.literal("false", Json::Bolean(false)),
            Some(b'n') => self.literal("null", Json::Null),
            Some(c) if c == b'-' || c.is_ascii_digit() => self.angka(),
            Some(c) => Err(format!(
                "Karakter tak terduga '{}' pada posisi {}",
                c as char, self.pos
            )),
            None => Err("JSON kosong atau terpotong".to_string()),
        }
    }

    fn literal(&mut self, teks: &str, nilai: Json) -> Result<Json, String> {
        if self.byte[self.pos..].starts_with(teks.as_bytes()) {
            self.pos += teks.len();
            Ok(nilai)
        } else {
            Err(format!("Literal tidak valid pada posisi {}", self.pos))
        }
    }

    fn objek(&mut self) -> Result<Json, String> {
        self.pos += 1; // buang '{'
        let mut pasangan = Vec::new();
        self.lewati_spasi();
        if self.lihat() == Some(b'}') {
            self.pos += 1;
            return Ok(Json::Objek(pasangan));
        }
        loop {
            self.lewati_spasi();
            if self.lihat() != Some(b'"') {
                return Err(format!("Mengharapkan kunci teks pada posisi {}", self.pos));
            }
            let kunci = self.teks()?;
            self.lewati_spasi();
            if self.lihat() != Some(b':') {
                return Err(format!("Mengharapkan ':' pada posisi {}", self.pos));
            }
            self.pos += 1;
            self.lewati_spasi();
            let nilai = self.nilai()?;
            pasangan.push((kunci, nilai));
            self.lewati_spasi();
            match self.lihat() {
                Some(b',') => {
                    self.pos += 1;
                }
                Some(b'}') => {
                    self.pos += 1;
                    return Ok(Json::Objek(pasangan));
                }
                _ => {
                    return Err(format!(
                        "Mengharapkan ',' atau '}}' pada posisi {}",
                        self.pos
                    ))
                }
            }
        }
    }

    fn larik(&mut self) -> Result<Json, String> {
        self.pos += 1; // buang '['
        let mut item = Vec::new();
        self.lewati_spasi();
        if self.lihat() == Some(b']') {
            self.pos += 1;
            return Ok(Json::Larik(item));
        }
        loop {
            self.lewati_spasi();
            item.push(self.nilai()?);
            self.lewati_spasi();
            match self.lihat() {
                Some(b',') => {
                    self.pos += 1;
                }
                Some(b']') => {
                    self.pos += 1;
                    return Ok(Json::Larik(item));
                }
                _ => {
                    return Err(format!(
                        "Mengharapkan ',' atau ']' pada posisi {}",
                        self.pos
                    ))
                }
            }
        }
    }

    fn teks(&mut self) -> Result<String, String> {
        self.pos += 1; // buang '"' pembuka
        let mut keluar = String::new();
        while let Some(c) = self.lihat() {
            self.pos += 1;
            match c {
                b'"' => return Ok(keluar),
                b'\\' => {
                    let esc = self.lihat().ok_or("Escape terpotong")?;
                    self.pos += 1;
                    match esc {
                        b'"' => keluar.push('"'),
                        b'\\' => keluar.push('\\'),
                        b'/' => keluar.push('/'),
                        b'n' => keluar.push('\n'),
                        b't' => keluar.push('\t'),
                        b'r' => keluar.push('\r'),
                        b'b' => keluar.push('\u{0008}'),
                        b'f' => keluar.push('\u{000C}'),
                        b'u' => {
                            let hex = self
                                .byte
                                .get(self.pos..self.pos + 4)
                                .ok_or("Unicode escape terpotong")?;
                            let kode = u32::from_str_radix(
                                std::str::from_utf8(hex).map_err(|_| "Unicode tidak valid")?,
                                16,
                            )
                            .map_err(|_| "Unicode escape tidak valid")?;
                            self.pos += 4;
                            keluar.push(char::from_u32(kode).unwrap_or('\u{FFFD}'));
                        }
                        c => return Err(format!("Escape tak dikenal '\\{}'", c as char)),
                    }
                }
                c => keluar.push(c as char),
            }
        }
        Err("Teks tidak ditutup".to_string())
    }

    fn angka(&mut self) -> Result<Json, String> {
        let mulai = self.pos;
        if self.lihat() == Some(b'-') {
            self.pos += 1;
        }
        while let Some(c) = self.lihat() {
            if c.is_ascii_digit() || c == b'.' || c == b'e' || c == b'E' || c == b'+' || c == b'-' {
                self.pos += 1;
            } else {
                break;
            }
        }
        let potong =
            std::str::from_utf8(&self.byte[mulai..self.pos]).map_err(|_| "Angka tidak valid")?;
        potong
            .parse::<f64>()
            .map(Json::Angka)
            .map_err(|_| format!("Angka tidak valid: '{}'", potong))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_objek_datar() {
        let j = parse(r#"{"nama": "proyek", "versi": "0.1.0"}"#).unwrap();
        assert_eq!(j.ambil("nama").unwrap().sebagai_teks().unwrap(), "proyek");
    }

    #[test]
    fn parse_larik() {
        let j = parse(r#"{"berkas": ["a.eve", "b.eve"]}"#).unwrap();
        let l = j.ambil("berkas").unwrap().sebagai_larik().unwrap();
        assert_eq!(l.len(), 2);
    }

    #[test]
    fn parse_angka_boolean_null() {
        let j = parse(r#"{"n": 42, "b": true, "z": null}"#).unwrap();
        assert_eq!(j.ambil("n").unwrap(), &Json::Angka(42.0));
        assert_eq!(j.ambil("b").unwrap(), &Json::Bolean(true));
        assert_eq!(j.ambil("z").unwrap(), &Json::Null);
    }

    #[test]
    fn tolak_json_rusak() {
        assert!(parse("{nama: 1}").is_err());
        assert!(parse(r#"{"a": }"#).is_err());
        assert!(parse(r#"{"a": 1"#).is_err());
    }

    #[test]
    fn bolak_balik_tulis_parse() {
        let asal = r#"{"nama": "x", "berkas": ["a", "b"]}"#;
        let j = parse(asal).unwrap();
        let teks = j.tulis_rapi();
        let lagi = parse(&teks).unwrap();
        assert_eq!(j, lagi);
    }

    #[test]
    fn escape_teks() {
        let j = parse(r#"{"p": "baris\nbaru \"kutip\""}"#).unwrap();
        assert_eq!(
            j.ambil("p").unwrap().sebagai_teks().unwrap(),
            "baris\nbaru \"kutip\""
        );
    }
}
