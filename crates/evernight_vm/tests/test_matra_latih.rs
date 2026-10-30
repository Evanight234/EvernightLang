mod common;
use common::*;
use evernight_core::{Lexer, Parser};
use evernight_vm::{Compiler, Value, Vm};

fn eksekusi_output(sumber: &str) -> (Result<Value, String>, Vec<String>) {
    let tokens = match Lexer::new(sumber).scan_tokens() {
        Ok(t) => t,
        Err(e) => return (Err(e.to_string()), Vec::new()),
    };
    let ast = match Parser::new(tokens).parse() {
        Ok(a) => a,
        Err(e) => return (Err(e.to_string()), Vec::new()),
    };
    let mut compiler = Compiler::new();
    if let Err(e) = compiler.compile(&ast) {
        return (Err(e.to_string()), Vec::new());
    }
    let mut vm = Vm::new();
    vm.set_capture_output(true);
    let res = vm.run(compiler.chunk).map_err(|e| e.to_string());
    let out = vm.take_captured_output();
    (res, out)
}

#[test]
fn test_pisah_data_dan_stratifikasi() {
    let kode = r##"
        impor "matra" sebagai mt

        # 10 sampel dengan 2 fitur
        variabel X = mt.larik([
            [1.0, 2.0], [2.0, 3.0], [3.0, 4.0], [4.0, 5.0], [5.0, 6.0],
            [6.0, 7.0], [7.0, 8.0], [8.0, 9.0], [9.0, 10.0], [10.0, 11.0]
        ])
        variabel y = [0, 0, 0, 0, 0, 1, 1, 1, 1, 1]

        variabel hasil = mt.pisah_data(X, y, 0.2, 42, benar)
        variabel X_latih = hasil[0]
        variabel X_uji = hasil[1]
        variabel y_latih = hasil[2]
        variabel y_uji = hasil[3]

        kembali [X_latih.bentuk, X_uji.bentuk, y_latih.panjang, y_uji.panjang]
    "##;

    let (res, _) = eksekusi_output(kode);
    assert!(res.is_ok(), "Gagal pisah_data: {:?}", res.err());
    if let Value::Daftar(list_rc) = res.unwrap() {
        let list = list_rc.borrow();
        // X_latih bentuk [8, 2]
        if let Value::Daftar(b) = &list[0] {
            assert_eq!(b.borrow()[0], Value::Angka(8.0));
            assert_eq!(b.borrow()[1], Value::Angka(2.0));
        } else {
            panic!("X_latih.bentuk bukan daftar");
        }
        // X_uji bentuk [2, 2]
        if let Value::Daftar(b) = &list[1] {
            assert_eq!(b.borrow()[0], Value::Angka(2.0));
            assert_eq!(b.borrow()[1], Value::Angka(2.0));
        }
        // y_latih panjang 8
        assert_eq!(list[2], Value::Angka(8.0));
        // y_uji panjang 2
        assert_eq!(list[3], Value::Angka(2.0));
    } else {
        panic!("Hasil bukan daftar");
    }
}

#[test]
fn test_standarisasi_dan_invers() {
    let kode = r##"
        impor "matra" sebagai mt

        variabel X = mt.larik([
            [10.0, 100.0],
            [20.0, 200.0],
            [30.0, 300.0],
            [40.0, 400.0]
        ])

        variabel scaler = mt.pra.Standarisasi()
        scaler.sesuaikan(X)
        variabel X_scaled = scaler.transformasi(X)

        variabel X_pulih = scaler.invers_transformasi(X_scaled)
        kembali [X_scaled.bentuk, X_pulih[0][0], X_pulih[3][1]]
    "##;

    let (res, _) = eksekusi_output(kode);
    assert!(res.is_ok(), "Gagal standarisasi: {:?}", res.err());
    if let Value::Daftar(list_rc) = res.unwrap() {
        let list = list_rc.borrow();
        // Bentuk [4, 2]
        if let Value::Daftar(b) = &list[0] {
            assert_eq!(b.borrow()[0], Value::Angka(4.0));
            assert_eq!(b.borrow()[1], Value::Angka(2.0));
        }
        // Pulih mendekati 10.0 dan 400.0
        if let Value::Angka(n) = list[1] {
            assert!((n - 10.0).abs() < 1e-4);
        }
        if let Value::Angka(n) = list[2] {
            assert!((n - 400.0).abs() < 1e-4);
        }
    }
}

#[test]
fn test_penskala_min_maks() {
    let kode = r##"
        impor "matra" sebagai mt

        variabel X = mt.larik([
            [10.0, 50.0],
            [20.0, 100.0],
            [30.0, 150.0]
        ])

        variabel scaler = mt.pra.PenskalaMinMaks(0, 1)
        variabel X_norm = scaler.sesuaikan_transformasi(X)

        # Baris pertama harus [0.0, 0.0] dan baris terakhir [1.0, 1.0]
        kembali [X_norm[0][0], X_norm[0][1], X_norm[2][0], X_norm[2][1]]
    "##;

    let (res, _) = eksekusi_output(kode);
    assert!(res.is_ok(), "Gagal min_maks: {:?}", res.err());
    if let Value::Daftar(list_rc) = res.unwrap() {
        let list = list_rc.borrow();
        assert_eq!(list[0], Value::Angka(0.0));
        assert_eq!(list[1], Value::Angka(0.0));
        assert_eq!(list[2], Value::Angka(1.0));
        assert_eq!(list[3], Value::Angka(1.0));
    }
}

#[test]
fn test_pengekod_label() {
    let kode = r##"
        impor "matra" sebagai mt

        variabel label_teks = ["kucing", "anjing", "kelinci", "kucing", "anjing"]
        variabel encoder = mt.pra.PengekodLabel()
        variabel ids = encoder.sesuaikan_transformasi(label_teks)

        variabel label_balik = encoder.invers_transformasi(ids)
        kembali [ids.panjang, label_balik[0], label_balik[1], label_balik[2]]
    "##;

    let (res, _) = eksekusi_output(kode);
    assert!(res.is_ok(), "Gagal pengekod label: {:?}", res.err());
    if let Value::Daftar(list_rc) = res.unwrap() {
        let list = list_rc.borrow();
        assert_eq!(list[0], Value::Angka(5.0));
        assert_eq!(list[1], Value::Teks("kucing".to_string()));
        assert_eq!(list[2], Value::Teks("anjing".to_string()));
        assert_eq!(list[3], Value::Teks("kelinci".to_string()));
    }
}

#[test]
fn test_smote_penyeimbang_data() {
    let kode = r##"
        impor "matra" sebagai mt

        # Kelas 0 ada 6 sampel, Kelas 1 hanya 2 sampel
        variabel X = mt.larik([
            [1.0, 2.0], [1.1, 2.1], [0.9, 1.9], [1.2, 2.2], [1.0, 2.0], [1.1, 2.0],
            [8.0, 9.0], [8.1, 9.1]
        ])
        variabel y = [0, 0, 0, 0, 0, 0, 1, 1]

        variabel hasil = mt.seimbang.SMOTE(X, y, 1, 1.0, 42)
        variabel X_seimbang = hasil[0]
        variabel y_seimbang = hasil[1]

        kembali [X_seimbang.bentuk, y_seimbang.panjang]
    "##;

    let (res, _) = eksekusi_output(kode);
    assert!(res.is_ok(), "Gagal SMOTE: {:?}", res.err());
    if let Value::Daftar(list_rc) = res.unwrap() {
        let list = list_rc.borrow();
        // Total sampel sekarang harus 12 (6 kelas 0 + 6 kelas 1)
        if let Value::Daftar(b) = &list[0] {
            assert_eq!(b.borrow()[0], Value::Angka(12.0));
            assert_eq!(b.borrow()[1], Value::Angka(2.0));
        }
        assert_eq!(list[1], Value::Angka(12.0));
    }
}

#[test]
fn test_pipa_kerja_anti_leakage() {
    let kode = r##"
        impor "matra" sebagai mt

        variabel X_latih = mt.larik([
            [10.0, 20.0],
            [20.0, 30.0],
            [30.0, 40.0]
        ])
        variabel X_uji = mt.larik([
            [15.0, 25.0]
        ])

        variabel scaler = mt.pra.Standarisasi()
        variabel pipa = mt.alur.PipaKerja([
            ["penskala", scaler]
        ])

        pipa.latih(X_latih)
        variabel X_uji_trans = pipa.transformasi(X_uji)
        kembali X_uji_trans.bentuk
    "##;

    let (res, _) = eksekusi_output(kode);
    assert!(res.is_ok(), "Gagal PipaKerja: {:?}", res.err());
    if let Value::Daftar(b_rc) = res.unwrap() {
        let b = b_rc.borrow();
        assert_eq!(b[0], Value::Angka(1.0));
        assert_eq!(b[1], Value::Angka(2.0));
    }
}

#[test]
fn test_deteksi_kebocoran_dan_drift() {
    let kode = r##"
        impor "matra" sebagai mt

        variabel X_latih = mt.larik([
            [1.0, 2.0],
            [3.0, 4.0],
            [5.0, 6.0]
        ])
        variabel X_uji_bocor = mt.larik([
            [3.0, 4.0], # Bocor! Persis baris ke-2 latih
            [7.0, 8.0]
        ])

        variabel hasil_bocor = mt.uji.cek_kebocoran(X_latih, X_uji_bocor)

        # Pergeseran distribusi (drift)
        variabel X_geser = mt.larik([
            [100.0, 200.0],
            [150.0, 250.0]
        ])
        variabel hasil_drift = mt.uji.cek_pergeseran(X_latih, X_geser, 0.25)

        kembali [hasil_bocor.ada_kebocoran, hasil_bocor.jumlah_kebocoran, hasil_drift.ada_pergeseran]
    "##;

    let (res, _) = eksekusi_output(kode);
    assert!(res.is_ok(), "Gagal cek kebocoran dan drift: {:?}", res.err());
    if let Value::Daftar(list_rc) = res.unwrap() {
        let list = list_rc.borrow();
        assert_eq!(list[0], Value::Bolean(true)); // ada kebocoran
        assert_eq!(list[1], Value::Angka(1.0));   // 1 sampel bocor
        assert_eq!(list[2], Value::Bolean(true)); // ada pergeseran distribusi
    }
}

#[test]
fn test_pencarian_kisi_parameter() {
    let kode = r##"
        impor "matra" sebagai mt

        variabel kisi = mt.tala.PencarianKisi({
            "kecepatan_belajar": [0.01, 0.1],
            "kedalaman": [3, 5]
        })

        kembali [kisi.jumlah, kisi.kombinasi.panjang]
    "##;

    let (res, _) = eksekusi_output(kode);
    assert!(res.is_ok(), "Gagal PencarianKisi: {:?}", res.err());
    if let Value::Daftar(list_rc) = res.unwrap() {
        let list = list_rc.borrow();
        // 2 x 2 = 4 kombinasi
        assert_eq!(list[0], Value::Angka(4.0));
        assert_eq!(list[1], Value::Angka(4.0));
    }
}
