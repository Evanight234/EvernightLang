mod common;
use common::*;
use evernight_vm::Value;

#[test]
fn konsol_baca_input() {
    let res = eksekusi_input("variabel a = baca()\nkembali a", &["halo"]);
    assert!(
        matches!(res, Ok(Value::Teks(ref s)) if s == "halo"),
        "{:?}",
        res
    );

    let res2 = eksekusi_input("variabel b = baca_angka()\nkembali b", &["42.5"]);
    assert!(
        matches!(res2, Ok(Value::Angka(n)) if (n - 42.5).abs() < 1e-9),
        "{:?}",
        res2
    );
}

#[test]
fn konsol_cetak_bersihkan_nan() {
    assert!(matches!(eksekusi("cetak(\"halo\")"), Ok(Value::Kosong)));
    assert!(matches!(eksekusi("bersihkan()"), Ok(Value::Kosong)));
    assert!(eksekusi_input("baca_angka()", &["abc"]).is_err());
}
