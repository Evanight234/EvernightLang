mod common;
use common::*;
use evernight_vm::Value;

#[test]
fn matra_impor_dan_pembuatan_larik() {
    let program = r#"
        impor "matra" sebagai mt
        variabel a = mt.larik([10, 20, 30])
        variabel nol = mt.nol([2, 3])
        variabel satu = mt.satu([3, 2])
        variabel p = mt.penuh([2, 2], 5)
        variabel r = mt.rentang(0, 10, 2)
        variabel sp = mt.spasi_linear(0, 10, 5)
        variabel idn = mt.identitas(3)
        variabel ac = mt.acak([2, 2])

        kembali [
            a.panjang,
            nol.bentuk,
            satu.bentuk,
            p[0][0],
            r.ke_daftar(),
            sp.ke_daftar(),
            idn[0][0],
            idn[1][1],
            idn[0][1],
            ac.panjang
        ]
    "#;

    let res = eksekusi(program).expect("Eksekusi matra gagal");
    if let Value::Daftar(list_rc) = res {
        let list = list_rc.borrow();
        assert_eq!(list[0], Value::Angka(3.0)); // a.panjang
        // nol.bentuk = [2, 3]
        if let Value::Daftar(b) = &list[1] {
            assert_eq!(b.borrow()[0], Value::Angka(2.0));
            assert_eq!(b.borrow()[1], Value::Angka(3.0));
        } else {
            panic!("nol.bentuk bukan daftar");
        }
        // satu.bentuk = [3, 2]
        if let Value::Daftar(b) = &list[2] {
            assert_eq!(b.borrow()[0], Value::Angka(3.0));
            assert_eq!(b.borrow()[1], Value::Angka(2.0));
        }
        // p[0][0] = 5
        assert_eq!(list[3], Value::Angka(5.0));
        // r = [0, 2, 4, 6, 8]
        if let Value::Daftar(r_list) = &list[4] {
            let r = r_list.borrow();
            assert_eq!(r.len(), 5);
            assert_eq!(r[0], Value::Angka(0.0));
            assert_eq!(r[4], Value::Angka(8.0));
        }
        // sp = [0, 2.5, 5, 7.5, 10]
        if let Value::Daftar(sp_list) = &list[5] {
            let sp = sp_list.borrow();
            assert_eq!(sp.len(), 5);
            assert_eq!(sp[0], Value::Angka(0.0));
            assert_eq!(sp[2], Value::Angka(5.0));
            assert_eq!(sp[4], Value::Angka(10.0));
        }
        // idn diagonal = 1, non-diagonal = 0
        assert_eq!(list[6], Value::Angka(1.0));
        assert_eq!(list[7], Value::Angka(1.0));
        assert_eq!(list[8], Value::Angka(0.0));
        // ac 2x2 = 4 elements
        assert_eq!(list[9], Value::Angka(4.0));
    } else {
        panic!("Hasil bukan daftar: {:?}", res);
    }
}

#[test]
fn matra_aritmatika_vektor_dan_broadcasting() {
    let program = r#"
        impor "matra" sebagai mt
        variabel a = mt.larik([1, 2, 3])
        variabel b = mt.larik([10, 20, 30])

        variabel tambah = (a + b).ke_daftar()
        variabel kurang = (b - a).ke_daftar()
        variabel kali = (a * b).ke_daftar()
        variabel bagi = (b / a).ke_daftar()
        variabel skalar_kanan = (a * 10).ke_daftar()
        variabel skalar_kiri = (100 - b).ke_daftar()
        variabel negasi = (-a).ke_daftar()
        variabel pangkat = (a ** 2).ke_daftar()
        variabel timbal_balik = (60 / a).ke_daftar()

        kembali [
            tambah, kurang, kali, bagi,
            skalar_kanan, skalar_kiri, negasi, pangkat, timbal_balik
        ]
    "#;

    let res = eksekusi(program).expect("Eksekusi aritmatika gagal");
    if let Value::Daftar(list_rc) = res {
        let list = list_rc.borrow();
        // a + b = [11, 22, 33]
        if let Value::Daftar(t) = &list[0] {
            assert_eq!(t.borrow()[0], Value::Angka(11.0));
            assert_eq!(t.borrow()[1], Value::Angka(22.0));
            assert_eq!(t.borrow()[2], Value::Angka(33.0));
        }
        // b - a = [9, 18, 27]
        if let Value::Daftar(k) = &list[1] {
            assert_eq!(k.borrow()[0], Value::Angka(9.0));
            assert_eq!(k.borrow()[1], Value::Angka(18.0));
            assert_eq!(k.borrow()[2], Value::Angka(27.0));
        }
        // a * b = [10, 40, 90]
        if let Value::Daftar(m) = &list[2] {
            assert_eq!(m.borrow()[0], Value::Angka(10.0));
            assert_eq!(m.borrow()[1], Value::Angka(40.0));
            assert_eq!(m.borrow()[2], Value::Angka(90.0));
        }
        // b / a = [10, 10, 10]
        if let Value::Daftar(d) = &list[3] {
            assert_eq!(d.borrow()[0], Value::Angka(10.0));
            assert_eq!(d.borrow()[1], Value::Angka(10.0));
            assert_eq!(d.borrow()[2], Value::Angka(10.0));
        }
        // a * 10 = [10, 20, 30]
        if let Value::Daftar(sk) = &list[4] {
            assert_eq!(sk.borrow()[0], Value::Angka(10.0));
            assert_eq!(sk.borrow()[1], Value::Angka(20.0));
            assert_eq!(sk.borrow()[2], Value::Angka(30.0));
        }
        // 100 - b = [90, 80, 70]
        if let Value::Daftar(sk) = &list[5] {
            assert_eq!(sk.borrow()[0], Value::Angka(90.0));
            assert_eq!(sk.borrow()[1], Value::Angka(80.0));
            assert_eq!(sk.borrow()[2], Value::Angka(70.0));
        }
        // -a = [-1, -2, -3]
        if let Value::Daftar(ng) = &list[6] {
            assert_eq!(ng.borrow()[0], Value::Angka(-1.0));
            assert_eq!(ng.borrow()[1], Value::Angka(-2.0));
            assert_eq!(ng.borrow()[2], Value::Angka(-3.0));
        }
        // a ** 2 = [1, 4, 9]
        if let Value::Daftar(p) = &list[7] {
            assert_eq!(p.borrow()[0], Value::Angka(1.0));
            assert_eq!(p.borrow()[1], Value::Angka(4.0));
            assert_eq!(p.borrow()[2], Value::Angka(9.0));
        }
        // 60 / a = [60, 30, 20]
        if let Value::Daftar(rec) = &list[8] {
            assert_eq!(rec.borrow()[0], Value::Angka(60.0));
            assert_eq!(rec.borrow()[1], Value::Angka(30.0));
            assert_eq!(rec.borrow()[2], Value::Angka(20.0));
        }
    } else {
        panic!("Hasil bukan daftar");
    }
}

#[test]
fn matra_indeks_dan_mutasi() {
    let program = r#"
        impor "matra" sebagai mt
        variabel v = mt.larik([10, 20, 30, 40])
        variabel elemen_awal = v[1]
        v[1] = 99
        variabel elemen_ubah = v[1]

        variabel m = mt.larik([[1, 2], [3, 4], [5, 6]])
        variabel baris_1 = m[1]
        variabel dimensi_m = m.dimensi
        variabel ukuran_m = m.ukuran

        kembali [elemen_awal, elemen_ubah, baris_1.ke_daftar(), dimensi_m, ukuran_m]
    "#;

    let res = eksekusi(program).expect("Eksekusi indeks gagal");
    if let Value::Daftar(list_rc) = res {
        let list = list_rc.borrow();
        assert_eq!(list[0], Value::Angka(20.0));
        assert_eq!(list[1], Value::Angka(99.0));
        if let Value::Daftar(b) = &list[2] {
            assert_eq!(b.borrow()[0], Value::Angka(3.0));
            assert_eq!(b.borrow()[1], Value::Angka(4.0));
        }
        assert_eq!(list[3], Value::Angka(2.0)); // 2 dimensi
        assert_eq!(list[4], Value::Angka(6.0)); // 6 elemen total
    } else {
        panic!("Hasil bukan daftar");
    }
}

#[test]
fn matra_aljabar_linier() {
    let program = r#"
        impor "matra" sebagai mt
        variabel v1 = mt.larik([1, 2, 3])
        variabel v2 = mt.larik([4, 5, 6])
        variabel titik_vektor = mt.titik(v1, v2)

        variabel m1 = mt.larik([[1, 2], [3, 4]])
        variabel m2 = mt.larik([[2, 0], [1, 2]])
        variabel titik_matriks = mt.titik(m1, m2).ke_daftar()
        variabel titik_metode = m1.titik(m2).ke_daftar()

        variabel transpose = m1.T.ke_daftar()
        variabel det = mt.determinan(m1)
        variabel inv = mt.invers(m1)
        variabel idn = mt.titik(m1, inv)

        kembali [
            titik_vektor,
            titik_matriks,
            titik_metode,
            transpose,
            det,
            idn[0][0],
            idn[1][1],
            idn[0][1]
        ]
    "#;

    let res = eksekusi(program).expect("Eksekusi aljabar linier gagal");
    if let Value::Daftar(list_rc) = res {
        let list = list_rc.borrow();
        // 1*4 + 2*5 + 3*6 = 4 + 10 + 18 = 32
        assert_eq!(list[0], Value::Angka(32.0));

        // m1 x m2 = [[1*2 + 2*1, 1*0 + 2*2], [3*2 + 4*1, 3*0 + 4*2]]
        //         = [[4, 4], [10, 8]]
        if let Value::Daftar(tm) = &list[1] {
            let row0 = match &tm.borrow()[0] { Value::Daftar(d) => d.borrow().clone(), _ => panic!() };
            let row1 = match &tm.borrow()[1] { Value::Daftar(d) => d.borrow().clone(), _ => panic!() };
            assert_eq!(row0[0], Value::Angka(4.0));
            assert_eq!(row0[1], Value::Angka(4.0));
            assert_eq!(row1[0], Value::Angka(10.0));
            assert_eq!(row1[1], Value::Angka(8.0));
        }

        // transpose of [[1, 2], [3, 4]] = [[1, 3], [2, 4]]
        if let Value::Daftar(tr) = &list[3] {
            let row0 = match &tr.borrow()[0] { Value::Daftar(d) => d.borrow().clone(), _ => panic!() };
            let row1 = match &tr.borrow()[1] { Value::Daftar(d) => d.borrow().clone(), _ => panic!() };
            assert_eq!(row0[0], Value::Angka(1.0));
            assert_eq!(row0[1], Value::Angka(3.0));
            assert_eq!(row1[0], Value::Angka(2.0));
            assert_eq!(row1[1], Value::Angka(4.0));
        }

        // det([[1, 2], [3, 4]]) = 1*4 - 2*3 = -2
        assert_eq!(list[4], Value::Angka(-2.0));

        // m1 * inv(m1) should be identity [[1, 0], [0, 1]] (within 1e-6)
        if let Value::Angka(i00) = list[5] {
            assert!((i00 - 1.0).abs() < 1e-6);
        }
        if let Value::Angka(i11) = list[6] {
            assert!((i11 - 1.0).abs() < 1e-6);
        }
        if let Value::Angka(i01) = list[7] {
            assert!(i01.abs() < 1e-6);
        }
    } else {
        panic!("Hasil bukan daftar");
    }
}

#[test]
fn matra_statistik_dan_reduksi_sumbu() {
    let program = r#"
        impor "matra" sebagai mt
        variabel a = mt.larik([2, 4, 4, 4, 5, 5, 7, 9])
        variabel jml = a.jumlah()
        variabel rata = a.rata_rata()
        variabel mn = a.min()
        variabel mx = a.max()
        variabel med = mt.median(a)
        variabel var = mt.varian(a)
        variabel std = mt.deviasi_standar(a)

        variabel m = mt.larik([[1, 2, 3], [4, 5, 6]])
        variabel jml_kolom = mt.jumlah(m, 0).ke_daftar() # sumbu 0: [5, 7, 9]
        variabel jml_baris = mt.jumlah(m, 1).ke_daftar() # sumbu 1: [6, 15]

        kembali [jml, rata, mn, mx, med, var, std, jml_kolom, jml_baris]
    "#;

    let res = eksekusi(program).expect("Eksekusi statistik gagal");
    if let Value::Daftar(list_rc) = res {
        let list = list_rc.borrow();
        assert_eq!(list[0], Value::Angka(40.0)); // 2+4+4+4+5+5+7+9 = 40
        assert_eq!(list[1], Value::Angka(5.0));  // 40 / 8 = 5
        assert_eq!(list[2], Value::Angka(2.0));  // min = 2
        assert_eq!(list[3], Value::Angka(9.0));  // max = 9
        assert_eq!(list[4], Value::Angka(4.5));  // median (4 + 5)/2 = 4.5
        assert_eq!(list[5], Value::Angka(4.0));  // varian = 32 / 8 = 4.0
        assert_eq!(list[6], Value::Angka(2.0));  // std = sqrt(4) = 2.0

        // jml_kolom = [5, 7, 9]
        if let Value::Daftar(k) = &list[7] {
            assert_eq!(k.borrow()[0], Value::Angka(5.0));
            assert_eq!(k.borrow()[1], Value::Angka(7.0));
            assert_eq!(k.borrow()[2], Value::Angka(9.0));
        }

        // jml_baris = [6, 15]
        if let Value::Daftar(b) = &list[8] {
            assert_eq!(b.borrow()[0], Value::Angka(6.0));
            assert_eq!(b.borrow()[1], Value::Angka(15.0));
        }
    } else {
        panic!("Hasil bukan daftar");
    }
}

#[test]
fn matra_manipulasi_bentuk_dan_matematika() {
    let program = r#"
        impor "matra" sebagai mt
        variabel a = mt.larik([1, 2, 3, 4, 5, 6])
        variabel m = a.ubah_bentuk([2, 3])
        variabel bentuk_m = m.bentuk
        variabel rata = m.ratakan().ke_daftar()

        variabel a1 = mt.larik([1, 2])
        variabel a2 = mt.larik([3, 4])
        variabel gb = mt.gabung([a1, a2], 0).ke_daftar()

        variabel akar = mt.akar(mt.larik([4, 9, 16])).ke_daftar()
        variabel mutlak = mt.mutlak(mt.larik([-1, -5, 3])).ke_daftar()

        kembali [bentuk_m, rata, gb, akar, mutlak]
    "#;

    let res = eksekusi(program).expect("Eksekusi manipulasi gagal");
    if let Value::Daftar(list_rc) = res {
        let list = list_rc.borrow();
        // bentuk_m = [2, 3]
        if let Value::Daftar(b) = &list[0] {
            assert_eq!(b.borrow()[0], Value::Angka(2.0));
            assert_eq!(b.borrow()[1], Value::Angka(3.0));
        }
        // rata = [1, 2, 3, 4, 5, 6]
        if let Value::Daftar(r) = &list[1] {
            assert_eq!(r.borrow().len(), 6);
            assert_eq!(r.borrow()[5], Value::Angka(6.0));
        }
        // gb = [1, 2, 3, 4]
        if let Value::Daftar(g) = &list[2] {
            assert_eq!(g.borrow().len(), 4);
            assert_eq!(g.borrow()[0], Value::Angka(1.0));
            assert_eq!(g.borrow()[3], Value::Angka(4.0));
        }
        // akar = [2, 3, 4]
        if let Value::Daftar(ak) = &list[3] {
            assert_eq!(ak.borrow()[0], Value::Angka(2.0));
            assert_eq!(ak.borrow()[1], Value::Angka(3.0));
            assert_eq!(ak.borrow()[2], Value::Angka(4.0));
        }
        // mutlak = [1, 5, 3]
        if let Value::Daftar(mtl) = &list[4] {
            assert_eq!(mtl.borrow()[0], Value::Angka(1.0));
            assert_eq!(mtl.borrow()[1], Value::Angka(5.0));
            assert_eq!(mtl.borrow()[2], Value::Angka(3.0));
        }
    } else {
        panic!("Hasil bukan daftar");
    }
}

#[test]
fn matra_error_handling_dan_try_catch() {
    let program = r#"
        impor "matra" sebagai mt
        variabel hasil_tangkap = []

        # 1. Pembagian dengan nol pada larik
        coba {
            variabel a = mt.larik([1, 2, 3])
            variabel b = mt.larik([1, 0, 3])
            variabel c = a / b
        } tangkap (e) {
            tambah(hasil_tangkap, "ERROR_BAGI_NOL")
        }

        # 2. Invers matriks singular
        coba {
            variabel singular = mt.larik([[1, 2], [2, 4]])
            variabel inv = mt.invers(singular)
        } tangkap (e) {
            tambah(hasil_tangkap, "ERROR_SINGULAR")
        }

        # 3. Dimensi perkalian titik tidak cocok
        coba {
            variabel v1 = mt.larik([1, 2])
            variabel v2 = mt.larik([1, 2, 3])
            variabel t = mt.titik(v1, v2)
        } tangkap (e) {
            tambah(hasil_tangkap, "ERROR_DIMENSI_TITIK")
        }

        kembali hasil_tangkap
    "#;

    let res = eksekusi(program).expect("Eksekusi try-catch gagal");
    if let Value::Daftar(list_rc) = res {
        let list = list_rc.borrow();
        assert_eq!(list.len(), 3);
        assert_eq!(list[0], Value::Teks("ERROR_BAGI_NOL".to_string()));
        assert_eq!(list[1], Value::Teks("ERROR_SINGULAR".to_string()));
        assert_eq!(list[2], Value::Teks("ERROR_DIMENSI_TITIK".to_string()));
    } else {
        panic!("Hasil bukan daftar: {:?}", res);
    }
}
