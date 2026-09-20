mod common;
use common::*;
use evernight_vm::Value;
use std::fs;

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
    let dir = std::env::temp_dir().join(format!("eve_sandbox_{}", std::process::id()));
    let _ = fs::create_dir_all(&dir);
    let file = dir.join("test.txt");
    let path_str = file.to_str().unwrap().replace('\\', "/");

    let p_tulis = format!(
        "tulis_file(\"{}\", \"Halo Evernight!\")\nkembali ada_file(\"{}\")",
        path_str, path_str
    );
    match eksekusi_dir(&dir, &p_tulis) {
        Ok(Value::Bolean(b)) => assert!(b, "tulis+ada_file harus true"),
        other => panic!("tulis+ada_file gagal: {:?}", other),
    }

    let p_baca = format!("kembali baca_file(\"{}\")", path_str);
    assert!(teks_equal_dir(&dir, "Halo Evernight!", &p_baca));

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn sistem_sandbox_tulis_ditolak() {
    let dir = std::env::temp_dir().join(format!("eve_sandbox_deny_{}", std::process::id()));
    let _ = fs::create_dir_all(&dir);
    // Coba tulis ke path di luar direktori program
    let target = std::env::temp_dir().join("eve_outside_test.txt");
    let path_str = target.to_str().unwrap().replace('\\', "/");
    let p = format!("tulis_file(\"{}\", \"boleh\")", path_str);
    let res = eksekusi_dir(&dir, &p);
    assert!(res.is_err(), "tulis di luar sandbox harus ditolak");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn sistem_sandbox_baca_ditolak() {
    let dir = std::env::temp_dir().join(format!("eve_sandbox_baca_{}", std::process::id()));
    let _ = fs::create_dir_all(&dir);
    let target = std::env::temp_dir().join("eve_outside_read.txt");
    let path_str = target.to_str().unwrap().replace('\\', "/");
    let p = format!("kembali baca_file(\"{}\")", path_str);
    let res = eksekusi_dir(&dir, &p);
    assert!(res.is_err(), "baca di luar sandbox harus ditolak");
    let _ = fs::remove_dir_all(&dir);
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
