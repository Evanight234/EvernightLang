mod common;
use common::*;

#[test]
fn kamus_operasi_dasar() {
    let p = "variabel k = {\"nama\": \"Budi\", \"umur\": 20}\nkembali k[\"nama\"]";
    assert!(teks_equal("Budi", p));

    let p_prop = "variabel k = {\"nama\": \"Budi\", \"umur\": 20}\nkembali k.umur";
    assert!(angka_equal(20.0, p_prop));

    let p_len = "variabel k = {\"a\": 1, \"b\": 2}\nkembali k.panjang";
    assert!(angka_equal(2.0, p_len));
}

#[test]
fn kamus_mutasi_dan_pemeriksaan() {
    let p = "variabel k = {\"a\": 1}\nk[\"b\"] = 2\nk.c = 3\nkembali k.b + k.c";
    assert!(angka_equal(5.0, p));

    assert!(bolean_equal(
        true,
        "variabel k = {\"a\": 1}\nkembali ada_kunci(k, \"a\")"
    ));
    assert!(bolean_equal(
        false,
        "variabel k = {\"a\": 1}\nkembali ada_kunci(k, \"z\")"
    ));
    assert!(angka_equal(
        10.0,
        "variabel k = {\"a\": 10}\nkembali dapatkan(k, \"a\", 0)"
    ));
    assert!(angka_equal(
        99.0,
        "variabel k = {\"a\": 10}\nkembali dapatkan(k, \"z\", 99)"
    ));
}

#[test]
fn kamus_hapus_kunci_fungsi_dan_nilai() {
    let p = "variabel k = {\"a\": 1, \"b\": 2}\nhapus_kunci(k, \"a\")\nkembali k.panjang";
    assert!(angka_equal(1.0, p));

    assert!(cek_daftar_teks(
        &["a", "b"],
        "kembali kunci({\"b\": 2, \"a\": 1})"
    ));
    assert!(cek_daftar_teks(
        &["1", "2"],
        "kembali nilai({\"a\": 1, \"b\": 2})"
    ));
}

#[test]
fn hapus_statement() {
    let p_daftar = "variabel a = [10, 20, 30]\nhapus a[1]\nkembali a.panjang";
    assert!(angka_equal(2.0, p_daftar));

    let p_kamus = "variabel k = {\"nama\": \"Budi\", \"umur\": 20}\nhapus k[\"nama\"]\nkembali ada_kunci(k, \"nama\")";
    assert!(bolean_equal(false, p_kamus));

    let p_kamus_dot = "variabel k = {\"nama\": \"Budi\", \"umur\": 20}\nhapus k.umur\nkembali ada_kunci(k, \"umur\")";
    assert!(bolean_equal(false, p_kamus_dot));
}

#[test]
fn hapus_statement_kesalahan() {
    assert!(anggap_false_if_err("variabel a = [1]\nhapus a[5]"));
    assert!(anggap_false_if_err("variabel a = 10\nhapus a"));
}

#[test]
fn hapus_kunci_statement() {
    let prog = "variabel k = {\"x\": 1}\nhapus k.x\nkembali ada_kunci(k, \"x\")";
    assert!(bolean_equal(false, prog));
}
