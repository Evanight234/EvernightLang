mod common;
use common::*;
use evernight_vm::Value;

#[test]
fn matematika_dasar() {
    assert!(angka_equal(4.0, "kembali akar(16)"));
    assert!(angka_equal(8.0, "kembali pangkat(2, 3)"));
}

#[test]
fn matematika_pembulatan() {
    assert!(angka_equal(3.0, "kembali bulat_bawah(3.7)"));
    assert!(angka_equal(4.0, "kembali bulat_atas(3.2)"));
    assert!(angka_equal(4.0, "kembali pembulatan(3.6)"));
    assert!(angka_equal(3.0, "kembali bundar(3.4)"));
}

#[test]
fn matematika_mutlak_min_max() {
    assert!(angka_equal(5.0, "kembali mutlak(-5)"));
    assert!(angka_equal(5.0, "kembali abs(-5)"));
    assert!(angka_equal(2.0, "kembali min(5, 2)"));
    assert!(angka_equal(5.0, "kembali max(5, 2)"));
}

#[test]
fn matematika_log_dan_trigonometri() {
    assert!(angka_equal(0.0, "kembali sin(0)"));
    assert!(angka_equal(1.0, "kembali cos(0)"));
    assert!(angka_equal(0.0, "kembali tan(0)"));
    assert!(angka_equal(0.0, "kembali log(1)"));
}

#[test]
fn matematika_faktorial() {
    assert!(angka_equal(120.0, "kembali faktorial(5)"));
    assert!(angka_equal(1.0, "kembali faktorial(0)"));
}

#[test]
fn matematika_acak_antara() {
    let res = eksekusi("kembali acak_antara(1, 10)");
    match res {
        Ok(Value::Angka(n)) => assert!((1.0..=10.0).contains(&n), "Nilai: {}", n),
        other => panic!("Bukan angka: {:?}", other),
    }
}

#[test]
fn matematika_error_tipe_dan_domain() {
    assert!(anggap_false_if_err("kembali akar(-4)"));
    assert!(anggap_false_if_err("kembali log(0)"));
    assert!(anggap_false_if_err("kembali log(-1)"));
    assert!(anggap_false_if_err("kembali faktorial(-1)"));
    assert!(anggap_false_if_err("kembali faktorial(3.5)"));
    assert!(anggap_false_if_err("kembali faktorial(171)"));
    assert!(anggap_false_if_err("kembali akar(\"teks\")"));
}

#[test]
fn matematika_impor_tanpa_warning() {
    assert!(angka_equal(4.0, "impor matematika\nkembali akar(16)"));
}

#[test]
fn konstanta_pi_dan_e() {
    assert!(angka_equal(std::f64::consts::PI, "kembali pi"));
    assert!(angka_equal(std::f64::consts::E, "kembali e"));
}

#[test]
fn faktorial_rekursif() {
    let prog = "fungsi faktorial(n) {\n  jika n == 0 { kembali 1 }\n  kembali n * faktorial(n - 1)\n}\nkembali faktorial(5)";
    assert!(angka_equal(120.0, prog));
}
