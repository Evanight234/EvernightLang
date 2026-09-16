mod common;
use common::*;
use evernight_vm::Value;

#[test]
fn impor_daftar_kamus_tanpa_warning() {
    assert!(bolean_equal(
        true,
        "impor daftar\nkembali adalah_daftar([])"
    ));
    assert!(bolean_equal(true, "impor kamus\nkembali adalah_kamus({})"));
}

#[test]
fn impor_lintas_berkas_alias() {
    let dir = buat_fixture_impor();
    let hasil = eksekusi_dir(
        dir,
        "impor \"helper_a\" sebagai ha\nkembali ha.tambah(2, 3)",
    );
    assert!(matches!(hasil, Ok(Value::Angka(5.0))), "{:?}", hasil);
}

#[test]
fn impor_lintas_berkas_tanpa_alias() {
    let dir = buat_fixture_impor();
    let hasil = eksekusi_dir(dir, "impor \"helper_a\"\nkembali helper_a.kali(4, 5)");
    assert!(matches!(hasil, Ok(Value::Angka(20.0))), "{:?}", hasil);
}

#[test]
fn impor_lintas_berkas_subdirektori() {
    let dir = buat_fixture_impor();
    let hasil = eksekusi_dir(
        dir,
        "impor \"sub/helper_b\"\nkembali helper_b.kurang(10, 4)",
    );
    assert!(matches!(hasil, Ok(Value::Angka(6.0))), "{:?}", hasil);
}

#[test]
fn impor_modul_diimpor_ganda() {
    let dir = buat_fixture_impor();
    let hasil = eksekusi_dir(
        dir,
        "impor \"helper_a\" sebagai h1\nimpor \"helper_a\" sebagai h2\nkembali h1.kali(2, 3) + h2.kali(4, 5)",
    );
    assert!(matches!(hasil, Ok(Value::Angka(26.0))), "{:?}", hasil);
}

#[test]
fn impor_fungsi_memakai_fungsi_modul_lain() {
    let dir = buat_fixture_impor();
    let hasil = eksekusi_dir(dir, "impor \"helper_c\"\nkembali helper_c.lipat(3)");
    assert!(matches!(hasil, Ok(Value::Angka(6.0))), "{:?}", hasil);
}

#[test]
fn impor_sirkular_dideteksi() {
    let dir = buat_fixture_impor();
    let hasil = eksekusi_dir(dir, "impor \"a\"\nkembali 1");
    assert!(hasil.is_err(), "{:?}", hasil);
    assert!(hasil.unwrap_err().contains("Siklus"));
}

#[test]
fn impor_file_tidak_ada() {
    let dir = buat_fixture_impor();
    let hasil = eksekusi_dir(dir, "impor \"tidak_ada\"\nkembali 1");
    assert!(hasil.is_err(), "{:?}", hasil);
    assert!(hasil.unwrap_err().contains("FILE"));
}
