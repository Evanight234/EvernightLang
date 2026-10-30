mod common;
use common::*;
use evernight_vm::Value;

#[test]
fn test_matra_tabel_pembuatan_dan_metadata() {
    let program = r#"
        impor "matra" sebagai mt

        # 1. Buat dari Kamus
        variabel data_kamus = {
            "nama": ["Budi", "Siti", "Agus"],
            "umur": [25, 30, 28],
            "gaji": [5000000, 7500000, 6000000]
        }
        variabel df1 = mt.tabel(data_kamus)

        # 2. Buat dari Daftar Rekaman
        variabel data_rekaman = [
            {"kota": "Jakarta", "suhu": 32},
            {"kota": "Bandung", "suhu": 24},
            {"kota": "Surabaya", "suhu": 34}
        ]
        variabel df2 = mt.tabel(data_rekaman)

        # 3. Buat dari Matra Larik (Matriks 2D)
        variabel matriks = mt.larik([[1, 2], [3, 4], [5, 6]])
        variabel df3 = mt.tabel(matriks, ["fitur_a", "fitur_b"])

        kembali [
            df1.panjang,
            df1.jumlah_kolom,
            df1.bentuk,
            df2.panjang,
            df2.jumlah_kolom,
            df3.panjang,
            df3.jumlah_kolom,
            df3.kolom
        ]
    "#;

    let res = eksekusi(program).expect("Eksekusi pembuatan tabel gagal");
    if let Value::Daftar(list_rc) = res {
        let list = list_rc.borrow();
        assert_eq!(list[0], Value::Angka(3.0)); // df1.panjang
        assert_eq!(list[1], Value::Angka(3.0)); // df1.jumlah_kolom
        if let Value::Daftar(b) = &list[2] {
            assert_eq!(b.borrow()[0], Value::Angka(3.0));
            assert_eq!(b.borrow()[1], Value::Angka(3.0));
        }
        assert_eq!(list[3], Value::Angka(3.0)); // df2.panjang
        assert_eq!(list[4], Value::Angka(2.0)); // df2.jumlah_kolom
        assert_eq!(list[5], Value::Angka(3.0)); // df3.panjang
        assert_eq!(list[6], Value::Angka(2.0)); // df3.jumlah_kolom
        if let Value::Daftar(k) = &list[7] {
            assert_eq!(k.borrow()[0], Value::Teks("fitur_a".to_string()));
            assert_eq!(k.borrow()[1], Value::Teks("fitur_b".to_string()));
        }
    } else {
        panic!("Hasil bukan daftar");
    }
}

#[test]
fn test_matra_tabel_akses_indeks_dan_mutasi() {
    let program = r#"
        impor "matra" sebagai mt

        variabel df = mt.tabel({
            "nama": ["Budi", "Siti", "Agus"],
            "umur": [25, 30, 28],
            "gaji": [5000000, 7500000, 6000000]
        })

        # Ambil kolom numerik (otomatis jadi MatraArray)
        variabel col_umur = df["umur"]
        variabel rata_umur = col_umur.rata_rata()

        # Ambil baris sebagai kamus
        variabel baris_pertama = df[0]
        variabel baris_kedua = df.baris(1)

        # Mutasi tambah kolom baru
        df["aktif"] = [benar, benar, salah]

        kembali [
            rata_umur,
            baris_pertama["nama"],
            baris_pertama["umur"],
            baris_kedua["nama"],
            baris_kedua["gaji"],
            df.jumlah_kolom,
            df["aktif"][0],
            df["aktif"][2]
        ]
    "#;

    let res = eksekusi(program).expect("Eksekusi akses dan mutasi tabel gagal");
    if let Value::Daftar(list_rc) = res {
        let list = list_rc.borrow();
        // (25 + 30 + 28) / 3 = 27.6666...
        if let Value::Angka(avg) = list[0] {
            assert!((avg - 27.666666666666668).abs() < 1e-6);
        } else {
            panic!("rata_umur bukan angka");
        }
        assert_eq!(list[1], Value::Teks("Budi".to_string()));
        assert_eq!(list[2], Value::Angka(25.0));
        assert_eq!(list[3], Value::Teks("Siti".to_string()));
        assert_eq!(list[4], Value::Angka(7500000.0));
        assert_eq!(list[5], Value::Angka(4.0)); // jumlah_kolom jadi 4
        assert_eq!(list[6], Value::Bolean(true));
        assert_eq!(list[7], Value::Bolean(false));
    } else {
        panic!("Hasil bukan daftar");
    }
}

#[test]
fn test_matra_tabel_slicing_dan_kolom_ops() {
    let program = r#"
        impor "matra" sebagai mt

        variabel df = mt.tabel({
            "id": [1, 2, 3, 4, 5],
            "nilai": [10, 20, 30, 40, 50],
            "skor": [100, 200, 300, 400, 500]
        })

        variabel atas = df.kepala(2)
        variabel bawah = df.ekor(2)
        variabel pilih = df.pilih_kolom(["id", "skor"])
        variabel tanpa_skor = df.kecuali(["skor"])
        variabel ganti = df.ganti_nama_kolom({"nilai": "poin"})

        kembali [
            atas.panjang,
            atas["nilai"][1],
            bawah.panjang,
            bawah["nilai"][0],
            pilih.jumlah_kolom,
            pilih.kolom,
            tanpa_skor.jumlah_kolom,
            tanpa_skor.kolom,
            ganti.kolom
        ]
    "#;

    let res = eksekusi(program).expect("Eksekusi slicing tabel gagal");
    if let Value::Daftar(list_rc) = res {
        let list = list_rc.borrow();
        assert_eq!(list[0], Value::Angka(2.0)); // kepala(2) -> 2 baris
        assert_eq!(list[1], Value::Angka(20.0));
        assert_eq!(list[2], Value::Angka(2.0)); // ekor(2) -> 2 baris
        assert_eq!(list[3], Value::Angka(40.0)); // ekor(2)[0] adalah baris 4 (nilai 40)
        assert_eq!(list[4], Value::Angka(2.0)); // pilih_kolom 2
        assert_eq!(list[6], Value::Angka(2.0)); // kecuali 1 -> 2
        if let Value::Daftar(k) = &list[8] {
            let names: Vec<String> = k.borrow().iter().map(|v| v.to_string()).collect();
            assert!(names.contains(&"poin".to_string()));
            assert!(!names.contains(&"nilai".to_string()));
        }
    } else {
        panic!("Hasil bukan daftar");
    }
}

#[test]
fn test_matra_tabel_pengurutan_dan_saring() {
    let program = r#"
        impor "matra" sebagai mt

        variabel df = mt.tabel({
            "nama": ["Candra", "Andi", "Bima"],
            "skor": [85, 95, 70]
        })

        # Urutkan berdasarkan skor menaik
        variabel urut_skor = df.urutkan("skor", benar)

        # Urutkan berdasarkan nama menurun
        variabel urut_nama = df.urutkan("nama", salah)

        # Saring baris dengan mask boolean
        variabel lolos = df.saring([benar, benar, salah])

        kembali [
            urut_skor["skor"][0],
            urut_skor["skor"][2],
            urut_nama["nama"][0],
            urut_nama["nama"][2],
            lolos.panjang,
            lolos["nama"][0],
            lolos["nama"][1]
        ]
    "#;

    let res = eksekusi(program).expect("Eksekusi pengurutan dan saring gagal");
    if let Value::Daftar(list_rc) = res {
        let list = list_rc.borrow();
        assert_eq!(list[0], Value::Angka(70.0)); // skor min
        assert_eq!(list[1], Value::Angka(95.0)); // skor max
        assert_eq!(list[2], Value::Teks("Candra".to_string())); // nama z-a
        assert_eq!(list[3], Value::Teks("Andi".to_string()));
        assert_eq!(list[4], Value::Angka(2.0)); // lolos 2 baris
        assert_eq!(list[5], Value::Teks("Candra".to_string()));
        assert_eq!(list[6], Value::Teks("Andi".to_string()));
    } else {
        panic!("Hasil bukan daftar");
    }
}

#[test]
fn test_matra_tabel_penanganan_nilai_kosong() {
    let program = r#"
        impor "matra" sebagai mt

        variabel df = mt.tabel({
            "a": [1, kosong, 3],
            "b": [10, 20, 30]
        })

        variabel ada_sebelum = df.ada_kosong()
        variabel bersih = df.buang_kosong()
        variabel ada_setelah = bersih.ada_kosong()

        variabel terisi_skalar = df.isi_kosong(0)
        variabel terisi_kamus = df.isi_kosong({"a": 99})

        kembali [
            ada_sebelum,
            bersih.panjang,
            ada_setelah,
            terisi_skalar["a"][1],
            terisi_kamus["a"][1]
        ]
    "#;

    let res = eksekusi(program).expect("Eksekusi nilai kosong gagal");
    if let Value::Daftar(list_rc) = res {
        let list = list_rc.borrow();
        assert_eq!(list[0], Value::Bolean(true)); // ada_sebelum == benar
        assert_eq!(list[1], Value::Angka(2.0)); // bersih.panjang == 2
        assert_eq!(list[2], Value::Bolean(false)); // ada_setelah == salah
        assert_eq!(list[3], Value::Angka(0.0)); // terisi_skalar
        assert_eq!(list[4], Value::Angka(99.0)); // terisi_kamus
    } else {
        panic!("Hasil bukan daftar");
    }
}

#[test]
fn test_matra_tabel_agregasi_dan_deskripsi() {
    let program = r#"
        impor "matra" sebagai mt

        variabel df = mt.tabel({
            "jurusan": ["IT", "IT", "Bisnis", "Bisnis", "IT"],
            "ipk": [3.5, 3.8, 3.2, 3.6, 3.9],
            "usia": [20, 21, 22, 21, 20]
        })

        variabel ringkasan = df.deskripsi()
        variabel per_jurusan = df.kelompokkan("jurusan", "rata_rata")
        variabel hitung_jurusan = df.kelompokkan("jurusan", "hitung")

        kembali [
            ringkasan.panjang,
            per_jurusan.panjang,
            hitung_jurusan["jurusan"][0],
            hitung_jurusan["jurusan"][1]
        ]
    "#;

    let res = eksekusi(program).expect("Eksekusi agregasi tabel gagal");
    if let Value::Daftar(list_rc) = res {
        let list = list_rc.borrow();
        assert_eq!(list[0], Value::Angka(5.0)); // deskripsi memiliki 5 baris statistik
        assert_eq!(list[1], Value::Angka(2.0)); // 2 grup unik (Bisnis dan IT)
        assert_eq!(list[2], Value::Teks("Bisnis".to_string()));
        assert_eq!(list[3], Value::Teks("IT".to_string()));
    } else {
        panic!("Hasil bukan daftar");
    }
}

#[test]
fn test_matra_tabel_interop_larik_dan_ml() {
    let program = r#"
        impor "matra" sebagai mt

        variabel df = mt.tabel({
            "x1": [1.0, 2.0, 3.0],
            "x2": [4.0, 5.0, 6.0],
            "label": ["A", "B", "A"]
        })

        # Konversi fitur numerik menjadi matriks 2D MatraArray
        variabel X = df.ke_larik(["x1", "x2"])
        variabel bobot = mt.larik([0.5, 0.5])
        variabel prediksi = mt.titik(X, bobot)

        variabel rekaman = df.ke_daftar()

        kembali [
            X.bentuk,
            prediksi.ke_daftar(),
            rekaman[0]["label"]
        ]
    "#;

    let res = eksekusi(program).expect("Eksekusi interop larik gagal");
    if let Value::Daftar(list_rc) = res {
        let list = list_rc.borrow();
        if let Value::Daftar(b) = &list[0] {
            assert_eq!(b.borrow()[0], Value::Angka(3.0)); // 3 baris
            assert_eq!(b.borrow()[1], Value::Angka(2.0)); // 2 kolom
        }
        // Prediksi:
        // [1*0.5 + 4*0.5] = 2.5
        // [2*0.5 + 5*0.5] = 3.5
        // [3*0.5 + 6*0.5] = 4.5
        if let Value::Daftar(p) = &list[1] {
            let pb = p.borrow();
            assert_eq!(pb[0], Value::Angka(2.5));
            assert_eq!(pb[1], Value::Angka(3.5));
            assert_eq!(pb[2], Value::Angka(4.5));
        }
        assert_eq!(list[2], Value::Teks("A".to_string()));
    } else {
        panic!("Hasil bukan daftar");
    }
}

#[test]
fn test_matra_tabel_csv_io() {
    let program = r#"
        impor "matra" sebagai mt

        variabel data = mt.tabel({
            "produk": ["Kopi", "Teh", "Susu"],
            "harga": [15000, 8000, 12000],
            "stok": [50, 100, 30]
        })

        variabel path = "uji_matra_produk.csv"
        data.tulis_csv(path)

        variabel data_baca = mt.baca_csv(path)

        kembali [
            data_baca.panjang,
            data_baca.jumlah_kolom,
            data_baca["produk"][0],
            data_baca["harga"][1],
            data_baca["stok"][2]
        ]
    "#;

    let res = eksekusi(program).expect("Eksekusi CSV I/O gagal");
    let _ = std::fs::remove_file("uji_matra_produk.csv");
    let _ = std::fs::remove_file("crates/evernight_vm/uji_matra_produk.csv");

    if let Value::Daftar(list_rc) = res {
        let list = list_rc.borrow();
        assert_eq!(list[0], Value::Angka(3.0));
        assert_eq!(list[1], Value::Angka(3.0));
        assert_eq!(list[2], Value::Teks("Kopi".to_string()));
        assert_eq!(list[3], Value::Angka(8000.0));
        assert_eq!(list[4], Value::Angka(30.0));
    } else {
        panic!("Hasil bukan daftar");
    }
}
