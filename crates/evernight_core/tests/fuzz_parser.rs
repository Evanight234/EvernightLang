use evernight_core::{Lexer, Parser};
use std::panic;

struct SimpleLcg {
    state: u64,
}

impl SimpleLcg {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_u32(&mut self) -> u32 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        (self.state >> 32) as u32
    }

    fn next_range(&mut self, min: usize, max: usize) -> usize {
        if min >= max {
            return min;
        }
        min + (self.next_u32() as usize % (max - min + 1))
    }
}

const FUZZ_TOKENS: &[&str] = &[
    "fungsi",
    "variabel",
    "tetap",
    "kembali",
    "jika",
    "lainnya_jika",
    "lainnya",
    "selama",
    "untuk",
    "dari",
    "sampai",
    "dalam",
    "berhenti",
    "lanjut",
    "coba",
    "tangkap",
    "akhirnya",
    "lempar",
    "pastikan",
    "impor",
    "sebagai",
    "hapus",
    "cocok",
    "kasus",
    "bawaan",
    "cetak",
    "baca",
    "baca_angka",
    "bersihkan",
    "benar",
    "salah",
    "kosong",
    "_",
    "123",
    "3.14",
    "0x1F",
    "\"halo\"",
    "'a'",
    "+",
    "-",
    "*",
    "/",
    "%",
    "**",
    "==",
    "!=",
    "<",
    "<=",
    ">",
    ">=",
    "&&",
    "||",
    "!",
    "=",
    "+=",
    "-=",
    "*=",
    "/=",
    "(",
    ")",
    "{",
    "}",
    "[",
    "]",
    ",",
    ":",
    ";",
    ".",
    "\n",
    " ",
    "\t",
    "# komentar\n",
    "#[[ blok ]]",
    "\\",
    "@",
    "$",
    "~",
    "`",
];

#[test]
fn test_fuzz_parser_anti_panic() {
    let mut rng = SimpleLcg::new(0xDEADBEEFCAFEBABE);
    let total_iterations = 10_000;
    let mut parse_success = 0;
    let mut parse_errors = 0;

    for i in 0..total_iterations {
        // Buat string input acak dengan menggabungkan 1-20 token acak
        let num_parts = rng.next_range(1, 20);
        let mut source = String::new();
        for _ in 0..num_parts {
            let idx = rng.next_range(0, FUZZ_TOKENS.len() - 1);
            source.push_str(FUZZ_TOKENS[idx]);
            if rng.next_range(0, 1) == 1 {
                source.push(' ');
            }
        }

        let src_clone = source.clone();
        let result = panic::catch_unwind(move || {
            let mut lexer = Lexer::new(&src_clone);
            if let Ok(tokens) = lexer.scan_tokens() {
                let mut parser = Parser::new(tokens);
                let _ = parser.parse();
            }
        });

        assert!(
            result.is_ok(),
            "Parser panic pada iterasi {} dengan input:\n{:?}",
            i,
            source
        );

        if result.is_ok() {
            parse_success += 1;
        } else {
            parse_errors += 1;
        }
    }

    assert_eq!(parse_errors, 0);
    assert_eq!(parse_success, total_iterations);
}

#[test]
fn test_fuzz_raw_bytes_anti_panic() {
    let mut rng = SimpleLcg::new(0xCAFEBABE12345678);
    let total_iterations = 5_000;

    for i in 0..total_iterations {
        let len = rng.next_range(1, 100);
        let mut bytes = Vec::with_capacity(len);
        for _ in 0..len {
            // Hasilkan byte ASCII dan non-ASCII utf-8 aman
            let b = rng.next_range(0, 127) as u8;
            bytes.push(b);
        }

        let source = String::from_utf8(bytes).unwrap();
        let src_clone = source.clone();
        let result = panic::catch_unwind(move || {
            let mut lexer = Lexer::new(&src_clone);
            if let Ok(tokens) = lexer.scan_tokens() {
                let mut parser = Parser::new(tokens);
                let _ = parser.parse();
            }
        });

        assert!(
            result.is_ok(),
            "Lexer/Parser panic pada raw bytes iterasi {} dengan input:\n{:?}",
            i,
            source
        );
    }
}
