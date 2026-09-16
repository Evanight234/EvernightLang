mod common;
use common::*;

#[test]
fn string_fungsi_dasar() {
    assert!(teks_equal("HALO", "kembali besar(\"halo\")"));
    assert!(teks_equal("dunia", "kembali kecil(\"DUNIA\")"));
    assert!(teks_equal("teks", "kembali bersih(\"   teks   \")"));
}

#[test]
fn string_potong_pecah_gabung() {
    assert!(teks_equal("abc", "kembali potong(\"abcdef\", 0, 3)"));
    assert!(cek_daftar_teks(
        &["a", "b", "c"],
        "kembali pecah(\"a,b,c\", \",\")"
    ));
    assert!(teks_equal("1-2-3", "kembali gabung([1, 2, 3], \"-\")"));
}

#[test]
fn string_ganti_dan_pemeriksaan() {
    assert!(teks_equal(
        "halo semesta",
        "kembali ganti(\"halo dunia\", \"dunia\", \"semesta\")"
    ));
    assert!(bolean_equal(
        true,
        "kembali mengandung(\"evernight\", \"night\")"
    ));
    assert!(bolean_equal(
        false,
        "kembali mengandung(\"evernight\", \"siang\")"
    ));
    assert!(bolean_equal(
        true,
        "kembali mulai_dengan(\"indonesia\", \"indo\")"
    ));
    assert!(bolean_equal(
        true,
        "kembali akhir_dengan(\"indonesia\", \"sia\")"
    ));
}

#[test]
fn string_ulang_format() {
    assert!(teks_equal("abcabc", "kembali ulang_teks(\"abc\", 2)"));
    assert!(teks_equal(
        "Nama saya Budi, umur 20",
        "kembali format(\"Nama saya {0}, umur {1}\", \"Budi\", 20)"
    ));
}

#[test]
fn string_impor_tanpa_warning() {
    assert!(teks_equal("EVE", "impor string\nkembali besar(\"eve\")"));
}

#[test]
fn string_kesalahan_tipe() {
    assert!(anggap_false_if_err("kembali besar(123)"));
    assert!(anggap_false_if_err("kembali potong(\"abc\", \"awal\", 2)"));
    assert!(anggap_false_if_err("kembali pecah(123, \",\")"));
    assert!(anggap_false_if_err(
        "kembali gabung(\"bukan_daftar\", \",\")"
    ));
}
