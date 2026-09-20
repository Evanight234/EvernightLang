mod common;
use common::*;
use evernight_vm::Value;

// === 7C: Edge-case & ketahanan ===

#[test]
fn edge_aritmatika_dasar() {
    assert!(angka_equal(0.0, "kembali 0"));
    assert!(angka_equal(-1.0, "kembali -1"));
    assert!(angka_equal(42.0, "kembali 6 * 7"));
    assert!(angka_equal(0.0, "kembali 0 / 1"));
    assert!(angka_equal(1.0, "kembali 10 % 3"));
}

#[test]
fn edge_pembagian_nol() {
    assert!(anggap_false_if_err("kembali 1 / 0"));
    assert!(anggap_false_if_err("kembali 0 / 0"));
}

#[test]
fn edge_string_kosong() {
    assert!(teks_equal("", "kembali \"\""));
    // panjang via properti .panjang
    assert!(angka_equal(0.0, "kembali \"\".panjang"));
}

#[test]
fn edge_string_panjang() {
    // String 1000 karakter → .panjang == 1000
    let s = "x".repeat(1000);
    let prog = format!("kembali \"{}\".panjang", s);
    assert!(angka_equal(1000.0, &prog));
}

#[test]
fn edge_daftar_kosong() {
    assert!(angka_equal(0.0, "kembali [].panjang"));
}

#[test]
fn edge_daftar_besar() {
    // Buat list 1000 elemen via loop + fungsi tambah()
    let prog = r#"
        variabel d = []
        untuk i dari 0 sampai 999 {
            tambah(d, i)
        }
        kembali d.panjang
    "#;
    assert!(angka_equal(1000.0, prog));
}

#[test]
fn edge_kamus_kosong() {
    // Mengakses kunci yang tidak ada → harus error
    assert!(anggap_false_if_err(r#"
        variabel k = {}
        kembali k["tidak_ada"]
    "#));
}

#[test]
fn edge_rekursi_dalam() {
    // Rekursi 50 level
    let prog = r#"
        fungsi f(n) {
            jika n <= 0 { kembali 0 }
            kembali 1 + f(n - 1)
        }
        kembali f(50)
    "#;
    assert!(angka_equal(50.0, prog));
}

#[test]
fn edge_try_catch_basic() {
    let prog = r#"
        coba {
            kembali 1 / 0
        } tangkap(pesan) {
            kembali -1
        }
    "#;
    assert!(angka_equal(-1.0, prog));
}

#[test]
fn edge_try_catch_tanpa_error() {
    let prog = r#"
        coba {
            kembali 42
        } tangkap(pesan) {
            kembali -1
        }
    "#;
    assert!(angka_equal(42.0, prog));
}

#[test]
fn edge_fungsi_tanpa_return() {
    // Fungsi tanpa return explicit → return nil/kosong
    let prog = r#"
        fungsi void_func() {
            0
        }
        kembali void_func() == void_func()
    "#;
    assert!(bolean_equal(true, prog));
}

#[test]
fn edge_sandbox_ditolak() {
    // Semua operasi file di luar sandbox harus ditolak
    assert!(anggap_false_if_err("kembali baca_file(\"C:/Windows/System32/config/sam\")"));
    assert!(anggap_false_if_err("tulis_file(\"C:/Windows/test.txt\", \"x\")"));
}

#[test]
fn edge_loop_nol_iterasi() {
    let prog = r#"
        variabel x = 0
        untuk i dari 1 sampai 0 {
            x += 1
        }
        kembali x
    "#;
    assert!(angka_equal(0.0, prog));
}

#[test]
fn edge_boolean_operasi() {
    // == menghasilkan bolean, bukan angka
    assert!(bolean_equal(true, "kembali benar == benar"));
    assert!(bolean_equal(false, "kembali benar == salah"));
    assert!(bolean_equal(true, "kembali bukan salah"));
    assert!(bolean_equal(true, "kembali benar atau salah"));
    assert!(bolean_equal(false, "kembali benar dan salah"));
}

#[test]
fn edge_nested_expression() {
    // 2 + 2*2 + 2*2*2 = 2 + 4 + 8 = 14
    assert!(angka_equal(14.0, "kembali 2 + 2 * 2 + 2 * 2 * 2"));
    assert!(angka_equal(8.0, "kembali (2 + 2) * 2"));
}

#[test]
fn edge_string_operasi() {
    assert!(teks_equal("HELLO", "kembali besar(\"hello\")"));
    assert!(teks_equal("abc", "kembali potong(\"abcdef\", 0, 3)"));
    assert!(bolean_equal(true, "kembali mengandung(\"hello\", \"ell\")"));
}

#[test]
fn edge_daftar_operasi() {
    // tambah + panjang + indeks
    let prog = r#"
        variabel a = [3, 1, 2]
        tambah(a, 0)
        kembali a.panjang
    "#;
    assert!(angka_equal(4.0, prog));

    // indeks
    assert!(angka_equal(10.0, "kembali [10, 20, 30][0]"));
    assert!(angka_equal(30.0, "kembali [10, 20, 30][2]"));
}

#[test]
fn edge_kamus_operasi() {
    let prog = r#"
        variabel k = {"a": 1, "b": 2}
        kembali k["a"] + k["b"]
    "#;
    assert!(angka_equal(3.0, prog));
}
