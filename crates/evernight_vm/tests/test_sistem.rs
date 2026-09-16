mod common;
use common::*;
use evernight_vm::Value;

#[test]
fn sistem_waktu_dan_selisih() {
    let p_waktu = "kembali waktu_sekarang() > 0";
    assert!(bolean_equal(true, p_waktu));

    let p_tgl = "kembali format_tanggal(waktu_sekarang(), \"YYYY\")";
    let res = eksekusi(p_tgl);
    assert!(
        matches!(res, Ok(Value::Teks(ref s)) if s.len() == 4),
        "{:?}",
        res
    );

    let p_selisih = "kembali selisih_waktu(100, 40)";
    assert!(angka_equal(60.0, p_selisih));
}

#[test]
fn sistem_berkas() {
    let file = std::env::temp_dir().join(format!("eve_test_fs_{}.txt", std::process::id()));
    let path_str = file.to_str().unwrap().replace('\\', "/");

    let p_tulis = format!(
        "tulis_file(\"{}\", \"Halo Evernight!\")\nkembali ada_file(\"{}\")",
        path_str, path_str
    );
    assert!(bolean_equal(true, &p_tulis));

    let p_baca = format!("kembali baca_file(\"{}\")", path_str);
    assert!(teks_equal("Halo Evernight!", &p_baca));

    let _ = std::fs::remove_file(file);
}

#[test]
fn sistem_env() {
    let p_set = "atur_env(\"EVE_TEST_ENV\", \"123\")\nkembali env(\"EVE_TEST_ENV\", \"default\")";
    assert!(teks_equal("123", p_set));

    let p_default = "kembali env(\"ENV_TIDAK_ADA_PASTI\", \"bawaan\")";
    assert!(teks_equal("bawaan", p_default));

    assert!(anggap_false_if_err("kembali env(\"ENV_TIDAK_ADA_PASTI\")"));
}

#[test]
fn sistem_tunda() {
    let res = eksekusi("tunda(0.01)\nkembali 1");
    assert!(matches!(res, Ok(Value::Angka(1.0))), "{:?}", res);
    assert!(anggap_false_if_err("tunda(-1)"));
}
