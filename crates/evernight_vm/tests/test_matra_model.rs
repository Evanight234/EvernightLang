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
fn test_regresi_linear_dan_metrik() {
    let kode = r##"
        impor "matra" sebagai mt

        # Data: y = 2*x1 + 3*x2 + 5
        variabel X = mt.larik([
            [1.0, 2.0],
            [2.0, 1.0],
            [3.0, 4.0],
            [4.0, 3.0],
            [5.0, 5.0],
            [6.0, 4.0]
        ])
        variabel y = [13.0, 12.0, 23.0, 22.0, 30.0, 29.0]

        variabel model = mt.model.RegresiLinear(0.02, 3000)
        model.latih(X, y)

        variabel y_pred = model.prediksi(X)
        variabel r2 = mt.metrik.r2_skor(y, y_pred)
        variabel mse = mt.metrik.mse(y, y_pred)
        variabel mae = mt.metrik.mae(y, y_pred)

        kembali [r2, mse, mae]
    "##;

    let (res, _) = eksekusi_output(kode);
    assert!(res.is_ok(), "Gagal regresi linear: {:?}", res.err());
    if let Value::Daftar(list_rc) = res.unwrap() {
        let list = list_rc.borrow();
        if let Value::Angka(r2) = list[0] {
            assert!(r2 > 0.95, "R2 harus tinggi (>0.95), didapat: {}", r2);
        }
        if let Value::Angka(mse) = list[1] {
            assert!(mse < 2.0, "MSE harus rendah (<2.0), didapat: {}", mse);
        }
    }
}

#[test]
fn test_regresi_logistik_biner_dan_probabilitas() {
    let kode = r##"
        impor "matra" sebagai mt

        # 2 kelas terpisah jelas
        variabel X = mt.larik([
            [1.0, 1.0], [1.5, 1.2], [1.2, 1.5],
            [8.0, 8.0], [8.5, 8.2], [8.2, 8.5]
        ])
        variabel y = [0.0, 0.0, 0.0, 1.0, 1.0, 1.0]

        variabel model = mt.model.RegresiLogistik(0.1, 1000)
        model.latih(X, y)

        variabel y_pred = model.prediksi(X)
        variabel proba = model.prediksi_probabilitas(X)
        variabel acc = mt.metrik.akurasi(y, y_pred)

        kembali [acc, proba.bentuk]
    "##;

    let (res, _) = eksekusi_output(kode);
    assert!(res.is_ok(), "Gagal regresi logistik: {:?}", res.err());
    if let Value::Daftar(list_rc) = res.unwrap() {
        let list = list_rc.borrow();
        assert_eq!(list[0], Value::Angka(1.0)); // Akurasi 100%
        if let Value::Daftar(b) = &list[1] {
            assert_eq!(b.borrow()[0], Value::Angka(6.0));
            assert_eq!(b.borrow()[1], Value::Angka(2.0));
        }
    }
}

#[test]
fn test_knn_klasifikasi_dan_regresi() {
    let kode = r##"
        impor "matra" sebagai mt

        variabel X = mt.larik([
            [1.0, 1.0], [2.0, 1.0],
            [10.0, 10.0], [11.0, 10.0]
        ])
        variabel y_kelas = [0.0, 0.0, 1.0, 1.0]
        variabel y_nilai = [10.0, 20.0, 100.0, 110.0]

        # KNN Klasifikasi
        variabel knn_c = mt.model.KNNKlasifikasi(1)
        knn_c.latih(X, y_kelas)
        variabel pred_c = knn_c.prediksi(mt.larik([[1.5, 1.0], [10.5, 10.0]]))

        # KNN Regresi
        variabel knn_r = mt.model.KNNRegresi(2)
        knn_r.latih(X, y_nilai)
        variabel pred_r = knn_r.prediksi(mt.larik([[1.5, 1.0]]))

        kembali [pred_c[0], pred_c[1], pred_r[0]]
    "##;

    let (res, _) = eksekusi_output(kode);
    assert!(res.is_ok(), "Gagal KNN: {:?}", res.err());
    if let Value::Daftar(list_rc) = res.unwrap() {
        let list = list_rc.borrow();
        assert_eq!(list[0], Value::Angka(0.0));
        assert_eq!(list[1], Value::Angka(1.0));
        assert_eq!(list[2], Value::Angka(15.0)); // (10 + 20) / 2 = 15
    }
}

#[test]
fn test_pohon_keputusan_dan_hutan_acak() {
    let kode = r##"
        impor "matra" sebagai mt

        variabel X = mt.larik([
            [1.0, 2.0], [1.5, 1.8], [2.0, 2.2],
            [8.0, 9.0], [8.5, 8.8], [9.0, 9.2]
        ])
        variabel y = [0.0, 0.0, 0.0, 1.0, 1.0, 1.0]

        # Pohon Keputusan
        variabel pohon = mt.model.PohonKeputusan(5)
        pohon.latih(X, y)
        variabel pred_pohon = pohon.prediksi(X)
        variabel acc_pohon = mt.metrik.akurasi(y, pred_pohon)

        # Hutan Acak
        variabel hutan = mt.model.HutanAcak(5, 5, 42)
        hutan.latih(X, y)
        variabel pred_hutan = hutan.prediksi(X)
        variabel acc_hutan = mt.metrik.akurasi(y, pred_hutan)

        kembali [acc_pohon, acc_hutan]
    "##;

    let (res, _) = eksekusi_output(kode);
    assert!(res.is_ok(), "Gagal Pohon / Hutan: {:?}", res.err());
    if let Value::Daftar(list_rc) = res.unwrap() {
        let list = list_rc.borrow();
        assert_eq!(list[0], Value::Angka(1.0));
        assert_eq!(list[1], Value::Angka(1.0));
    }
}

#[test]
fn test_kmeans_clustering() {
    let kode = r##"
        impor "matra" sebagai mt

        # 2 klaster terpisah jelas
        variabel X = mt.larik([
            [1.0, 1.0], [1.2, 1.1], [0.9, 1.0],
            [20.0, 20.0], [20.2, 20.1], [19.9, 20.0]
        ])

        variabel km = mt.model.KMeans(2, 100, 42)
        km.latih(X)

        variabel label = km.prediksi(X)
        kembali [label[0] == label[1], label[0] != label[3], km.pusat_klaster.bentuk]
    "##;

    let (res, _) = eksekusi_output(kode);
    assert!(res.is_ok(), "Gagal KMeans: {:?}", res.err());
    if let Value::Daftar(list_rc) = res.unwrap() {
        let list = list_rc.borrow();
        assert_eq!(list[0], Value::Bolean(true));  // Titik 0 dan 1 satu klaster
        assert_eq!(list[1], Value::Bolean(true));  // Titik 0 dan 3 beda klaster
        if let Value::Daftar(b) = &list[2] {
            assert_eq!(b.borrow()[0], Value::Angka(2.0)); // 2 pusat klaster
            assert_eq!(b.borrow()[1], Value::Angka(2.0)); // 2 fitur
        }
    }
}

#[test]
fn test_metrik_klasifikasi_komprehensif() {
    let kode = r##"
        impor "matra" sebagai mt

        variabel y_asli = [1.0, 0.0, 1.0, 1.0, 0.0, 1.0]
        variabel y_pred = [1.0, 0.0, 1.0, 0.0, 0.0, 1.0]

        variabel acc = mt.metrik.akurasi(y_asli, y_pred)
        variabel prec = mt.metrik.presisi(y_asli, y_pred, 1.0)
        variabel rec = mt.metrik.recall(y_asli, y_pred, 1.0)
        variabel f1 = mt.metrik.f1_skor(y_asli, y_pred, 1.0)
        variabel konfusi = mt.metrik.matriks_konfusi(y_asli, y_pred)
        variabel laporan = mt.metrik.laporan_klasifikasi(y_asli, y_pred)

        kembali [acc, prec, rec, f1, konfusi.bentuk, laporan]
    "##;

    let (res, _) = eksekusi_output(kode);
    assert!(res.is_ok(), "Gagal metrik: {:?}", res.err());
    if let Value::Daftar(list_rc) = res.unwrap() {
        let list = list_rc.borrow();
        // 5 benar dari 6
        if let Value::Angka(acc) = list[0] {
            assert!((acc - 5.0 / 6.0).abs() < 1e-4);
        }
        // Presisi kelas 1: 3 TP / (3 TP + 0 FP) = 1.0
        assert_eq!(list[1], Value::Angka(1.0));
        // Recall kelas 1: 3 TP / (3 TP + 1 FN) = 0.75
        assert_eq!(list[2], Value::Angka(0.75));
        // Matriks konfusi bentuk [2, 2]
        if let Value::Daftar(b) = &list[4] {
            assert_eq!(b.borrow()[0], Value::Angka(2.0));
            assert_eq!(b.borrow()[1], Value::Angka(2.0));
        }
        // Laporan teks memuat string presisi dan recall
        if let Value::Teks(ref s) = list[5] {
            assert!(s.contains("presisi"));
            assert!(s.contains("f1-skor"));
        }
    }
}

#[test]
fn test_pipa_kerja_dengan_model() {
    let kode = r##"
        impor "matra" sebagai mt

        variabel X_latih = mt.larik([
            [10.0, 20.0], [12.0, 22.0],
            [100.0, 200.0], [110.0, 210.0]
        ])
        variabel y_latih = [0.0, 0.0, 1.0, 1.0]

        variabel pipa = mt.alur.PipaKerja([
            ["penskala", mt.pra.Standarisasi()],
            ["model", mt.model.KNNKlasifikasi(1)]
        ])

        pipa.latih(X_latih, y_latih)

        variabel X_uji = mt.larik([
            [11.0, 21.0],
            [105.0, 205.0]
        ])
        variabel pred = pipa.prediksi(X_uji)

        kembali [pred[0], pred[1]]
    "##;

    let (res, _) = eksekusi_output(kode);
    assert!(res.is_ok(), "Gagal pipa model: {:?}", res.err());
    if let Value::Daftar(list_rc) = res.unwrap() {
        let list = list_rc.borrow();
        assert_eq!(list[0], Value::Angka(0.0));
        assert_eq!(list[1], Value::Angka(1.0));
    }
}
