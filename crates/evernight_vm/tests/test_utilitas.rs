mod common;
use common::*;
use evernight_vm::Value;

#[test]
fn utilitas_adalah() {
    assert!(bolean_equal(true, "kembali adalah_angka(42)"));
    assert!(bolean_equal(false, "kembali adalah_angka(\"teks\")"));
    assert!(bolean_equal(true, "kembali adalah_teks(\"halo\")"));
    assert!(bolean_equal(true, "kembali adalah_daftar([1, 2])"));
    assert!(bolean_equal(true, "kembali adalah_kamus({\"a\": 1})"));
}

#[test]
fn utilitas_boolean() {
    assert!(bolean_equal(true, "kembali ke_boolean(1)"));
    assert!(bolean_equal(false, "kembali ke_boolean(0)"));
    assert!(bolean_equal(true, "kembali e_boolean(\"teks\")"));
    assert!(bolean_equal(false, "kembali bolean(\"\")"));
}

#[test]
fn utilitas_ke_larik() {
    assert!(cek_daftar_teks(
        &["h", "a", "l", "o"],
        "kembali ke_larik(\"halo\")"
    ));
    assert!(cek_daftar_teks(&["1", "2"], "kembali daftar([1, 2])"));
    assert!(cek_daftar_teks(
        &["a", "b"],
        "kembali ke_larik({\"b\": 2, \"a\": 1})"
    ));
}

#[test]
fn utilitas_salin() {
    let skalar = eksekusi("variabel a = 5\nvariabel b = salin(a)\nkembali b");
    assert!(matches!(skalar, Ok(Value::Angka(5.0))), "{:?}", skalar);

    let daftar =
        eksekusi("variabel a = [1, [2, 3]]\nvariabel s = salin(a)\ntambah(a, 9)\nkembali s");
    match daftar {
        Ok(Value::Daftar(d)) => {
            let v = d.borrow();
            assert_eq!(v.len(), 2, "{:?}", v);
        }
        other => panic!("{:?}", other),
    }

    let fungsional = eksekusi("fungsi f(x) { kembali x }\nvariabel s = salin(f)\nkembali s(7)");
    assert!(
        matches!(fungsional, Ok(Value::Angka(7.0))),
        "{:?}",
        fungsional
    );

    let siklik =
        eksekusi("variabel a = [1]\ntambah(a, a)\nvariabel s = salin(a)\nkembali s.panjang");
    assert!(matches!(siklik, Ok(Value::Angka(2.0))), "{:?}", siklik);

    assert!(anggap_false_if_err("kembali salin()"));
}

#[test]
fn utilitas_konversi() {
    assert!(angka_equal(12.5, "kembali angka(\"12.5\")"));
    assert!(angka_equal(1.0, "kembali angka(benar)"));
    assert!(angka_equal(0.0, "kembali angka(salah)"));
    assert!(angka_equal(7.0, "kembali angka(7)"));
    assert!(anggap_false_if_err("kembali angka(\"bukan_angka\")"));
    assert!(teks_equal("budi", "kembali teks(\"budi\")"));
    assert!(teks_equal("42", "kembali teks(42)"));
    assert!(teks_equal("benar", "kembali teks(benar)"));
    assert!(teks_equal("kosong", "kembali teks(kosong)"));
    assert!(teks_equal(
        "budi",
        "kembali dapatkan(kamus([[\"nama\", \"budi\"]]), \"nama\")"
    ));
    assert!(teks_equal(
        "2",
        "kembali teks(dapatkan(kamus({\"a\": 1, \"b\": 2}), \"b\"))"
    ));
    assert!(anggap_false_if_err("kembali angka([1])"));
    assert!(anggap_false_if_err("kembali kamus([[1]])"));
    assert!(anggap_false_if_err("kembali kamus([\"a\"])"));
}

#[test]
fn impor_utilitas_noop() {
    assert!(bolean_equal(
        true,
        "impor utilitas\nkembali adalah_daftar([])"
    ));
}
