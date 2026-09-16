use crate::errors::EvernightError;
use crate::token::{Token, TokenType};

pub struct Lexer {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
    column: usize,
    start_column: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        // Buang BOM UTF-8 di awal berkas. Banyak editor Windows
        // (Notepad, PowerShell `Set-Content -Encoding UTF8`) menambahkannya,
        // dan tanpa ini berkas yang sah akan gagal di-tokenisasi.
        let source = source.strip_prefix('\u{FEFF}').unwrap_or(source);
        Self {
            source: source.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
            column: 1,
            start_column: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, EvernightError> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            self.start = self.current;
            self.start_column = self.column;
            if let Some(token) = self.scan_token()? {
                tokens.push(token);
            }
        }

        tokens.push(Token::new(TokenType::Eof, "", self.line, self.column));
        Ok(tokens)
    }

    fn scan_token(&mut self) -> Result<Option<Token>, EvernightError> {
        let c = self.advance();

        match c {
            // Whitespace
            ' ' | '\r' | '\t' => Ok(None),
            '\n' => {
                self.line += 1;
                self.column = 1;
                Ok(None)
            }

            // Komentar
            '#' => {
                while self.peek() != '\n' && !self.is_at_end() {
                    self.advance();
                }
                Ok(None)
            }

            // Tanda Baca
            '(' => Ok(Some(self.make_token(TokenType::KurungBuka))),
            ')' => Ok(Some(self.make_token(TokenType::KurungTutup))),
            '{' => Ok(Some(self.make_token(TokenType::KurawalBuka))),
            '}' => Ok(Some(self.make_token(TokenType::KurawalTutup))),
            '[' => Ok(Some(self.make_token(TokenType::SikuBuka))),
            ']' => Ok(Some(self.make_token(TokenType::SikuTutup))),
            ',' => Ok(Some(self.make_token(TokenType::Koma))),
            ':' => Ok(Some(self.make_token(TokenType::TitikDua))),
            '.' => Ok(Some(self.make_token(TokenType::Titik))),
            ';' => Ok(Some(self.make_token(TokenType::TitikKoma))),

            // Operator Aritmatika & Penugasan
            '+' => {
                if self.match_char('=') {
                    Ok(Some(self.make_token(TokenType::TambahSama)))
                } else {
                    Ok(Some(self.make_token(TokenType::Tambah)))
                }
            }
            '-' => {
                if self.match_char('=') {
                    Ok(Some(self.make_token(TokenType::KurangSama)))
                } else {
                    Ok(Some(self.make_token(TokenType::Kurang)))
                }
            }
            '*' => {
                if self.match_char('*') {
                    Ok(Some(self.make_token(TokenType::BintangBintang)))
                } else if self.match_char('=') {
                    Ok(Some(self.make_token(TokenType::BintangSama)))
                } else {
                    Ok(Some(self.make_token(TokenType::Bintang)))
                }
            }
            '/' => {
                if self.match_char('=') {
                    Ok(Some(self.make_token(TokenType::GarisMiringSama)))
                } else {
                    Ok(Some(self.make_token(TokenType::GarisMiring)))
                }
            }
            '%' => Ok(Some(self.make_token(TokenType::Persen))),

            // Operator Perbandingan & Logika
            '=' => {
                if self.match_char('=') {
                    Ok(Some(self.make_token(TokenType::SamaSama)))
                } else {
                    Ok(Some(self.make_token(TokenType::Sama)))
                }
            }
            '!' => {
                if self.match_char('=') {
                    Ok(Some(self.make_token(TokenType::SeruSama)))
                } else {
                    Ok(Some(self.make_token(TokenType::Seru)))
                }
            }
            '<' => {
                if self.match_char('=') {
                    Ok(Some(self.make_token(TokenType::KurangSamaSimbol)))
                } else {
                    Ok(Some(self.make_token(TokenType::KurangDariSimbol)))
                }
            }
            '>' => {
                if self.match_char('=') {
                    Ok(Some(self.make_token(TokenType::LebihSamaSimbol)))
                } else {
                    Ok(Some(self.make_token(TokenType::LebihDariSimbol)))
                }
            }
            '&' => {
                if self.match_char('&') {
                    Ok(Some(self.make_token(TokenType::DanSimbol)))
                } else {
                    Err(EvernightError::bahaya(
                        "SYNTAX",
                        self.line,
                        self.start_column,
                        "Karakter tak terduga '&', apakah maksud Anda '&&' atau 'dan'?",
                    ))
                }
            }
            '|' => {
                if self.match_char('|') {
                    Ok(Some(self.make_token(TokenType::AtauSimbol)))
                } else {
                    Err(EvernightError::bahaya(
                        "SYNTAX",
                        self.line,
                        self.start_column,
                        "Karakter tak terduga '|', apakah maksud Anda '||' atau 'atau'?",
                    ))
                }
            }

            // String Literal
            '"' | '\'' => self.string(c),

            // Number Literal
            c if c.is_ascii_digit() => self.number(),

            // Identifier / Keyword / Wildcard
            c if c.is_alphabetic() || c == '_' => self.identifier(),

            _ => Err(EvernightError::bahaya(
                "SYNTAX",
                self.line,
                self.start_column,
                format!("Karakter tidak dikenal: '{}'", c),
            )),
        }
    }

    fn string(&mut self, quote: char) -> Result<Option<Token>, EvernightError> {
        let mut value = String::new();

        while self.peek() != quote && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 1;
            }

            if self.peek() == '\\' {
                self.advance(); // lewati '\'
                if self.is_at_end() {
                    break;
                }
                match self.peek() {
                    'n' => value.push('\n'),
                    't' => value.push('\t'),
                    'r' => value.push('\r'),
                    '\\' => value.push('\\'),
                    '"' => value.push('"'),
                    '\'' => value.push('\''),
                    other => {
                        value.push('\\');
                        value.push(other);
                    }
                }
                if !self.is_at_end() {
                    self.advance();
                }
            } else {
                value.push(self.advance());
            }
        }

        if self.is_at_end() {
            return Err(EvernightError::bahaya(
                "SYNTAX",
                self.line,
                self.start_column,
                "Penutup tanda petik tidak ditemukan!",
            ));
        }

        // Konsumsi tanda petik penutup
        self.advance();

        let lexeme: String = self.source[self.start..self.current].iter().collect();
        Ok(Some(Token::new(
            TokenType::Teks(value),
            lexeme,
            self.line,
            self.start_column,
        )))
    }

    fn number(&mut self) -> Result<Option<Token>, EvernightError> {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // Cek bagian desimal (.123)
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance(); // konsumsi '.'
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let lexeme: String = self.source[self.start..self.current].iter().collect();
        let value: f64 = lexeme.parse().map_err(|_| {
            EvernightError::bahaya(
                "NaN",
                self.line,
                self.start_column,
                format!("Angka tidak valid: '{}'", lexeme),
            )
        })?;

        Ok(Some(Token::new(
            TokenType::Angka(value),
            lexeme,
            self.line,
            self.start_column,
        )))
    }

    fn identifier(&mut self) -> Result<Option<Token>, EvernightError> {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let lexeme: String = self.source[self.start..self.current].iter().collect();

        let token_type = match lexeme.as_str() {
            // Wildcard tunggal
            "_" => TokenType::Wildcard,

            // Literal
            "benar" => TokenType::Benar,
            "salah" => TokenType::Salah,
            "kosong" => TokenType::Kosong,

            // Deklarasi
            "variabel" => TokenType::Variabel,
            "tetap" => TokenType::Tetap,
            "fungsi" => TokenType::Fungsi,
            "kembali" => TokenType::Kembali,

            // Percabangan
            "jika" => TokenType::Jika,
            "lainnya_jika" => TokenType::LainnyaJika,
            "lainnya" => TokenType::Lainnya,
            "cocok" => TokenType::Cocok,
            "kasus" => TokenType::Kasus,
            "bawaan" => TokenType::Bawaan,

            // Perulangan
            "selama" => TokenType::Selama,
            "untuk" => TokenType::Untuk,
            "dari" => TokenType::Dari,
            "sampai" => TokenType::Sampai,
            "dalam" => TokenType::Dalam,
            "berhenti" => TokenType::Berhenti,
            "lanjut" => TokenType::Lanjut,
            "hapus" => TokenType::Hapus,

            // Operator Kata
            "dan" => TokenType::Dan,
            "atau" => TokenType::Atau,
            "bukan" => TokenType::Bukan,
            "sama_dengan" => TokenType::SamaDengan,
            "lebih_dari" => TokenType::LebihDari,
            "kurang_dari" => TokenType::KurangDari,

            // Penanganan Kesalahan
            "coba" => TokenType::Coba,
            "tangkap" => TokenType::Tangkap,
            "akhirnya" => TokenType::Akhirnya,
            "lempar" => TokenType::Lempar,
            "pastikan" => TokenType::Pastikan,

            // Modul & I/O
            "impor" => TokenType::Impor,
            "sebagai" => TokenType::Sebagai,
            "cetak" => TokenType::Cetak,
            "baca" => TokenType::Baca,

            // Identifier Pengguna
            _ => TokenType::Identifier(lexeme.clone()),
        };

        Ok(Some(Token::new(
            token_type,
            lexeme,
            self.line,
            self.start_column,
        )))
    }

    fn advance(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        let c = self.source[self.current];
        self.current += 1;
        self.column += 1;
        c
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.current] != expected {
            return false;
        }
        self.current += 1;
        self.column += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        let lexeme: String = self.source[self.start..self.current].iter().collect();
        Token::new(token_type, lexeme, self.line, self.start_column)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keywords_and_identifiers() {
        let source = "variabel hasil = jika x > 10 { kembali benar }";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.scan_tokens().unwrap();

        assert_eq!(tokens[0].token_type, TokenType::Variabel);
        assert_eq!(tokens[1].token_type, TokenType::Identifier("hasil".into()));
        assert_eq!(tokens[2].token_type, TokenType::Sama);
        assert_eq!(tokens[3].token_type, TokenType::Jika);
        assert_eq!(tokens[4].token_type, TokenType::Identifier("x".into()));
        assert_eq!(tokens[5].token_type, TokenType::LebihDariSimbol);
        assert_eq!(tokens[6].token_type, TokenType::Angka(10.0));
        assert_eq!(tokens[7].token_type, TokenType::KurawalBuka);
        assert_eq!(tokens[8].token_type, TokenType::Kembali);
        assert_eq!(tokens[9].token_type, TokenType::Benar);
        assert_eq!(tokens[10].token_type, TokenType::KurawalTutup);
        assert_eq!(tokens[11].token_type, TokenType::Eof);
    }

    #[test]
    fn test_string_escapes() {
        let source = r#" "Halo \"Dunia\"\n" "#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.scan_tokens().unwrap();

        assert_eq!(
            tokens[0].token_type,
            TokenType::Teks("Halo \"Dunia\"\n".into())
        );
    }

    #[test]
    fn test_operators() {
        let source = "+ += - -= * *= ** / /= % == != < <= > >= && || !";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.scan_tokens().unwrap();

        let expected = vec![
            TokenType::Tambah,
            TokenType::TambahSama,
            TokenType::Kurang,
            TokenType::KurangSama,
            TokenType::Bintang,
            TokenType::BintangSama,
            TokenType::BintangBintang,
            TokenType::GarisMiring,
            TokenType::GarisMiringSama,
            TokenType::Persen,
            TokenType::SamaSama,
            TokenType::SeruSama,
            TokenType::KurangDariSimbol,
            TokenType::KurangSamaSimbol,
            TokenType::LebihDariSimbol,
            TokenType::LebihSamaSimbol,
            TokenType::DanSimbol,
            TokenType::AtauSimbol,
            TokenType::Seru,
            TokenType::Eof,
        ];

        let actual: Vec<TokenType> = tokens.into_iter().map(|t| t.token_type).collect();
        assert_eq!(actual, expected);
    }
}
