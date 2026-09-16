mod common;
use common::*;

#[test]
fn daftar_operasi_dasar() {
    assert!(angka_equal(
        3.0,
        "variabel a = [10, 20, 30]\nkembali a.panjang"
    ));
    assert!(angka_equal(20.0, "variabel a = [10, 20, 30]\nkembali a[1]"));
    assert!(angka_equal(
        30.0,
        "variabel a = [10, 20, 30]\nkembali a[-1]"
    ));
    assert!(angka_equal(
        4.0,
        "variabel a = [1, 2]\ntambah(a, 3)\nsisip(a, 0, 0)\nkembali a.panjang"
    ));
    assert!(angka_equal(
        2.0,
        "variabel a = [10, 20, 30]\nhapus(a, 20)\nkembali a.panjang"
    ));
}

#[test]
fn daftar_gabung_dan_urut() {
    assert!(cek_daftar_teks(
        &["1", "2", "3", "4"],
        "variabel a = [1, 2]\nvariabel b = [3, 4]\nkembali gabung_larik(a, b)"
    ));
    assert!(cek_daftar_teks(
        &["1", "2", "3"],
        "variabel a = [3, 1, 2]\nkembali urutkan(a)"
    ));
    assert!(cek_daftar_teks(
        &["3", "2", "1"],
        "variabel a = [1, 2, 3]\nkembali balik(a)"
    ));
    assert!(cek_daftar_teks(
        &["1", "2", "3"],
        "variabel a = [1, 2, 2, 3, 1]\nkembali unik(a)"
    ));
}

#[test]
fn daftar_jumlah_rata_cari_ada() {
    assert!(angka_equal(10.0, "kembali jumlah([1, 2, 3, 4])"));
    assert!(angka_equal(2.5, "kembali rata_rata([1, 2, 3, 4])"));
    assert!(angka_equal(2.0, "kembali cari([10, 20, 30], 30)"));
    assert!(angka_equal(-1.0, "kembali cari([10, 20, 30], 99)"));
    assert!(bolean_equal(true, "kembali ada([10, 20, 30], 20)"));
    assert!(bolean_equal(false, "kembali ada([10, 20, 30], 99)"));
    assert!(cek_daftar_teks(
        &["20", "30"],
        "kembali iris([10, 20, 30, 40], 1, 3)"
    ));
}

#[test]
fn higher_order_peta_saring() {
    let p = "fungsi kuadrat(x) { kembali x * x }\nkembali peta([1, 2, 3], kuadrat)";
    assert!(cek_daftar_teks(&["1", "4", "9"], p));

    let s = "fungsi genap(x) { kembali x % 2 == 0 }\nkembali saring([1, 2, 3, 4, 5, 6], genap)";
    assert!(cek_daftar_teks(&["2", "4", "6"], s));
}

#[test]
fn higher_order_lipat_dan_error() {
    let l =
        "fungsi jumlahkan(acc, x) { kembali acc + x }\nkembali lipat([1, 2, 3, 4], 0, jumlahkan)";
    assert!(angka_equal(10.0, l));

    let err_arity = "fungsi salah(x) { kembali x }\nkembali peta([1, 2], salah)";
    assert!(anggap_false_if_err(err_arity));
}

#[test]
fn higher_order_setiap() {
    let prog = "variabel total = 0\nfungsi tambah_total(x) { total += x }\nsetiap([10, 20, 30], tambah_total)\nkembali total";
    assert!(angka_equal(60.0, prog));
}

#[test]
fn daftar_kesalahan() {
    assert!(anggap_false_if_err("variabel a = [1]\nkembali a[5]"));
    assert!(anggap_false_if_err("variabel a = [1]\nkembali a[-5]"));
    assert!(anggap_false_if_err("kembali jumlah([1, \"a\"])"));
    assert!(anggap_false_if_err("kembali rata_rata([])"));
    assert!(anggap_false_if_err("variabel a = [1, \"a\"]\nurutkan(a)"));
}
