pub mod alur;
pub mod grafik;
pub mod larik;
pub mod latih;
pub mod metrik;
pub mod model;
pub mod pelatih;
pub mod pra;
pub mod seimbang;
pub mod syaraf;
pub mod tabel;
pub mod tala;
pub mod uji;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::value::Value;
pub use alur::PipaKerja;
pub use grafik::{JenisPlot, MatraPlot};
pub use larik::MatraArray;
pub use latih::{pisah_data, KLipat};
pub use model::{JenisModel, MatraModel};
pub use pelatih::MatraCallback;
pub use pelatih::MatraPelatih;
pub use pra::{JenisTransformer, MatraTransformer};
pub use seimbang::Smote;
pub use syaraf::{MatraJaringan, MatraLapisan, MatraLoss, MatraOptimizer, Parameter};
pub use tabel::MatraTabel;
pub use tala::{PencarianKisi, TalaBayes};
pub use uji::{cek_kebocoran, cek_pergeseran};

// ID fungsi asli Matra (1000+)
pub const MATRA_FN_LARIK: u16 = 1000;
pub const MATRA_FN_NOL: u16 = 1001;
pub const MATRA_FN_SATU: u16 = 1002;
pub const MATRA_FN_PENUH: u16 = 1003;
pub const MATRA_FN_RENTANG: u16 = 1004;
pub const MATRA_FN_SPASI_LINEAR: u16 = 1005;
pub const MATRA_FN_IDENTITAS: u16 = 1006;
pub const MATRA_FN_ACAK: u16 = 1007;
pub const MATRA_FN_TITIK: u16 = 1008;
pub const MATRA_FN_TRANSPOSISI: u16 = 1009;
pub const MATRA_FN_INVERS: u16 = 1010;
pub const MATRA_FN_DETERMINAN: u16 = 1011;
pub const MATRA_FN_JUMLAH: u16 = 1012;
pub const MATRA_FN_RATA_RATA: u16 = 1013;
pub const MATRA_FN_MIN: u16 = 1014;
pub const MATRA_FN_MAX: u16 = 1015;
pub const MATRA_FN_MEDIAN: u16 = 1016;
pub const MATRA_FN_VARIAN: u16 = 1017;
pub const MATRA_FN_DEVIASI_STANDAR: u16 = 1018;
pub const MATRA_FN_UBAH_BENTUK: u16 = 1019;
pub const MATRA_FN_RATAKAN: u16 = 1020;
pub const MATRA_FN_GABUNG: u16 = 1021;
pub const MATRA_FN_AKAR: u16 = 1022;
pub const MATRA_FN_MUTLAK: u16 = 1023;
pub const MATRA_FN_SIN: u16 = 1024;
pub const MATRA_FN_COS: u16 = 1025;
pub const MATRA_FN_TAN: u16 = 1026;
pub const MATRA_FN_EXP: u16 = 1027;
pub const MATRA_FN_LOG: u16 = 1028;

// ID fungsi asli Matra Tabel (1030+)
pub const MATRA_FN_TABEL: u16 = 1030;
pub const MATRA_FN_BACA_CSV: u16 = 1031;
pub const MATRA_FN_TULIS_CSV: u16 = 1032;

// ID fungsi asli Matra Grafik (1040+)
pub const MATRA_FN_GRAFIK_GARIS: u16 = 1040;
pub const MATRA_FN_GRAFIK_SEBAR: u16 = 1041;
pub const MATRA_FN_GRAFIK_BATANG: u16 = 1042;
pub const MATRA_FN_GRAFIK_HISTOGRAM: u16 = 1043;
pub const MATRA_FN_GRAFIK_KONFUSI: u16 = 1044;

// ID fungsi asli Matra Latih, Seimbang, Pra, Alur, Uji, Tala (1050+)
pub const MATRA_FN_PISAH_DATA: u16 = 1050;
pub const MATRA_FN_K_LIPAT: u16 = 1051;
pub const MATRA_FN_VALIDASI_SILANG: u16 = 1052;
pub const MATRA_FN_SMOTE: u16 = 1053;
pub const MATRA_FN_STANDARISASI: u16 = 1054;
pub const MATRA_FN_MIN_MAKS: u16 = 1055;
pub const MATRA_FN_PENGEKOD_LABEL: u16 = 1056;
pub const MATRA_FN_PIPA_KERJA: u16 = 1057;
pub const MATRA_FN_CEK_KEBOCORAN: u16 = 1058;
pub const MATRA_FN_CEK_PERGESERAN: u16 = 1059;
pub const MATRA_FN_PENCARIAN_KISI: u16 = 1060;
pub const MATRA_FN_TALA_BAYES: u16 = 1061;

// ID fungsi asli Matra Model (1070+)
pub const MATRA_FN_REGRESI_LINEAR: u16 = 1070;
pub const MATRA_FN_REGRESI_LOGISTIK: u16 = 1071;
pub const MATRA_FN_KNN_KLASIFIKASI: u16 = 1072;
pub const MATRA_FN_KNN_REGRESI: u16 = 1073;
pub const MATRA_FN_POHON_KEPUTUSAN: u16 = 1074;
pub const MATRA_FN_HUTAN_ACAK: u16 = 1075;
pub const MATRA_FN_KMEANS: u16 = 1076;

// ID fungsi asli Matra Metrik (1080+)
pub const MATRA_FN_METRIK_AKURASI: u16 = 1080;
pub const MATRA_FN_METRIK_PRESISI: u16 = 1081;
pub const MATRA_FN_METRIK_RECALL: u16 = 1082;
pub const MATRA_FN_METRIK_F1: u16 = 1083;
pub const MATRA_FN_METRIK_KONFUSI: u16 = 1084;
pub const MATRA_FN_METRIK_LAPORAN: u16 = 1085;
pub const MATRA_FN_METRIK_MSE: u16 = 1086;
pub const MATRA_FN_METRIK_RMSE: u16 = 1087;
pub const MATRA_FN_METRIK_MAE: u16 = 1088;
pub const MATRA_FN_METRIK_R2: u16 = 1089;
pub const MATRA_FN_METRIK_MAPE: u16 = 1090;

// ID fungsi asli Matra Syaraf & Pelatih (1091+, 1180+)
pub const MATRA_FN_LINEAR: u16 = 1091;
pub const MATRA_FN_RELU: u16 = 1092;
pub const MATRA_FN_SIGMOID: u16 = 1093;
pub const MATRA_FN_TANH: u16 = 1094;
pub const MATRA_FN_SOFTMAX: u16 = 1095;
pub const MATRA_FN_DROPOUT: u16 = 1096;
pub const MATRA_FN_MODEL_SYARAF: u16 = 1097;
pub const MATRA_FN_LOSS_MSE: u16 = 1098;
pub const MATRA_FN_LOSS_BCE: u16 = 1099;
pub const MATRA_FN_LOSS_CROSS_ENTROPY: u16 = 1180;
pub const MATRA_FN_OPTIM_SGD: u16 = 1181;
pub const MATRA_FN_OPTIM_ADAM: u16 = 1182;
pub const MATRA_FN_PELATIH: u16 = 1183;
pub const MATRA_FN_CB_BERHENTI_AWAL: u16 = 1184;
pub const MATRA_FN_CB_SIMPAN_TERBAIK: u16 = 1185;
pub const MATRA_FN_CB_PENCATAT_RIWAYAT: u16 = 1186;

// ID metode bound Syaraf, Optim, Pelatih (1190+)
pub const MATRA_METHOD_SYARAF_MAJU: u16 = 1190;
pub const MATRA_METHOD_SYARAF_MUNDUR: u16 = 1191;
pub const MATRA_METHOD_SYARAF_PREDIKSI: u16 = 1192;
pub const MATRA_METHOD_SYARAF_PARAMETER: u16 = 1193;
pub const MATRA_METHOD_SYARAF_MODE_LATIH: u16 = 1194;
pub const MATRA_METHOD_SYARAF_MODE_EVAL: u16 = 1195;
pub const MATRA_METHOD_SYARAF_SIMPAN: u16 = 1196;
pub const MATRA_METHOD_SYARAF_MUAT: u16 = 1197;
pub const MATRA_METHOD_OPTIM_LANGKAH: u16 = 1198;
pub const MATRA_METHOD_OPTIM_NOL_GRAD: u16 = 1199;
pub const MATRA_METHOD_PELATIH_PASANG_CB: u16 = 1200;
pub const MATRA_METHOD_PELATIH_LATIH: u16 = 1201;
pub const MATRA_METHOD_PELATIH_EVALUASI: u16 = 1202;

// ID metode bound Matra Larik (1100+)
pub const MATRA_METHOD_KE_DAFTAR: u16 = 1100;
pub const MATRA_METHOD_TRANSPOSISI: u16 = 1101;
pub const MATRA_METHOD_JUMLAH: u16 = 1102;
pub const MATRA_METHOD_RATA_RATA: u16 = 1103;
pub const MATRA_METHOD_MIN: u16 = 1104;
pub const MATRA_METHOD_MAX: u16 = 1105;
pub const MATRA_METHOD_TITIK: u16 = 1106;
pub const MATRA_METHOD_UBAH_BENTUK: u16 = 1107;
pub const MATRA_METHOD_RATAKAN: u16 = 1108;

// ID metode bound Matra Tabel (1120+)
pub const MATRA_METHOD_KEPALA: u16 = 1120;
pub const MATRA_METHOD_EKOR: u16 = 1121;
pub const MATRA_METHOD_BARIS: u16 = 1122;
pub const MATRA_METHOD_PILIH_KOLOM: u16 = 1123;
pub const MATRA_METHOD_HAPUS_KOLOM: u16 = 1124;
pub const MATRA_METHOD_GANTI_NAMA: u16 = 1125;
pub const MATRA_METHOD_URUTKAN: u16 = 1126;
pub const MATRA_METHOD_SARING: u16 = 1127;
pub const MATRA_METHOD_ADA_KOSONG: u16 = 1128;
pub const MATRA_METHOD_BUANG_KOSONG: u16 = 1129;
pub const MATRA_METHOD_ISI_KOSONG: u16 = 1130;
pub const MATRA_METHOD_KE_LARIK: u16 = 1131;
pub const MATRA_METHOD_DESKRIPSI: u16 = 1132;
pub const MATRA_METHOD_KELOMPOKKAN: u16 = 1133;
pub const MATRA_METHOD_TULIS_CSV: u16 = 1134;

// ID metode bound Matra Grafik (1140+)
pub const MATRA_METHOD_PLOT_JUDUL: u16 = 1140;
pub const MATRA_METHOD_PLOT_LABEL_X: u16 = 1141;
pub const MATRA_METHOD_PLOT_LABEL_Y: u16 = 1142;
pub const MATRA_METHOD_PLOT_WARNA: u16 = 1143;
pub const MATRA_METHOD_PLOT_WARNA_LATAR: u16 = 1144;
pub const MATRA_METHOD_PLOT_GRID: u16 = 1145;
pub const MATRA_METHOD_PLOT_UKURAN: u16 = 1146;
pub const MATRA_METHOD_PLOT_SIMPAN_SVG: u16 = 1147;
pub const MATRA_METHOD_PLOT_KE_SVG: u16 = 1148;
pub const MATRA_METHOD_PLOT_TAMPIL: u16 = 1149;

// ID metode bound Transformer, Pipa, KLipat, Smote (1150+)
pub const MATRA_METHOD_TRANSFORMER_SESUAIKAN: u16 = 1150;
pub const MATRA_METHOD_TRANSFORMER_TRANSFORMASI: u16 = 1151;
pub const MATRA_METHOD_TRANSFORMER_INVERS: u16 = 1152;
pub const MATRA_METHOD_PIPA_TAMBAH: u16 = 1153;
pub const MATRA_METHOD_PIPA_LATIH: u16 = 1154;
pub const MATRA_METHOD_PIPA_TRANSFORMASI: u16 = 1155;
pub const MATRA_METHOD_KLIPAT_SPLIT: u16 = 1156;
pub const MATRA_METHOD_SMOTE_SESUAIKAN: u16 = 1157;
pub const MATRA_METHOD_KISI_KOMBINASI: u16 = 1158;
pub const MATRA_METHOD_TALA_SARAN: u16 = 1159;
pub const MATRA_METHOD_TALA_PERBARUI: u16 = 1160;

// ID metode bound Model & Pipa (1170+)
pub const MATRA_METHOD_MODEL_LATIH: u16 = 1170;
pub const MATRA_METHOD_MODEL_PREDIKSI: u16 = 1171;
pub const MATRA_METHOD_MODEL_PREDIKSI_PROBA: u16 = 1172;
pub const MATRA_METHOD_MODEL_SKOR: u16 = 1173;
pub const MATRA_METHOD_PIPA_PREDIKSI: u16 = 1174;

/// Membangun kamus ekspor untuk modul `matra`
pub fn buat_modul_matra() -> Value {
    let mut map = HashMap::new();

    // Fungsi-fungsi pembangun & aljabar linier
    map.insert(
        "larik".to_string(),
        Value::FungsiAsli {
            name: "larik".to_string(),
            id: MATRA_FN_LARIK,
        },
    );
    map.insert(
        "vektor".to_string(),
        Value::FungsiAsli {
            name: "vektor".to_string(),
            id: MATRA_FN_LARIK,
        },
    );
    map.insert(
        "matriks".to_string(),
        Value::FungsiAsli {
            name: "matriks".to_string(),
            id: MATRA_FN_LARIK,
        },
    );
    map.insert(
        "nol".to_string(),
        Value::FungsiAsli {
            name: "nol".to_string(),
            id: MATRA_FN_NOL,
        },
    );
    map.insert(
        "satu".to_string(),
        Value::FungsiAsli {
            name: "satu".to_string(),
            id: MATRA_FN_SATU,
        },
    );
    map.insert(
        "penuh".to_string(),
        Value::FungsiAsli {
            name: "penuh".to_string(),
            id: MATRA_FN_PENUH,
        },
    );
    map.insert(
        "rentang".to_string(),
        Value::FungsiAsli {
            name: "rentang".to_string(),
            id: MATRA_FN_RENTANG,
        },
    );
    map.insert(
        "spasi_linear".to_string(),
        Value::FungsiAsli {
            name: "spasi_linear".to_string(),
            id: MATRA_FN_SPASI_LINEAR,
        },
    );
    map.insert(
        "identitas".to_string(),
        Value::FungsiAsli {
            name: "identitas".to_string(),
            id: MATRA_FN_IDENTITAS,
        },
    );
    map.insert(
        "acak".to_string(),
        Value::FungsiAsli {
            name: "acak".to_string(),
            id: MATRA_FN_ACAK,
        },
    );
    map.insert(
        "titik".to_string(),
        Value::FungsiAsli {
            name: "titik".to_string(),
            id: MATRA_FN_TITIK,
        },
    );
    map.insert(
        "transposisi".to_string(),
        Value::FungsiAsli {
            name: "transposisi".to_string(),
            id: MATRA_FN_TRANSPOSISI,
        },
    );
    map.insert(
        "invers".to_string(),
        Value::FungsiAsli {
            name: "invers".to_string(),
            id: MATRA_FN_INVERS,
        },
    );
    map.insert(
        "determinan".to_string(),
        Value::FungsiAsli {
            name: "determinan".to_string(),
            id: MATRA_FN_DETERMINAN,
        },
    );
    map.insert(
        "jumlah".to_string(),
        Value::FungsiAsli {
            name: "jumlah".to_string(),
            id: MATRA_FN_JUMLAH,
        },
    );
    map.insert(
        "rata_rata".to_string(),
        Value::FungsiAsli {
            name: "rata_rata".to_string(),
            id: MATRA_FN_RATA_RATA,
        },
    );
    map.insert(
        "min".to_string(),
        Value::FungsiAsli {
            name: "min".to_string(),
            id: MATRA_FN_MIN,
        },
    );
    map.insert(
        "max".to_string(),
        Value::FungsiAsli {
            name: "max".to_string(),
            id: MATRA_FN_MAX,
        },
    );
    map.insert(
        "median".to_string(),
        Value::FungsiAsli {
            name: "median".to_string(),
            id: MATRA_FN_MEDIAN,
        },
    );
    map.insert(
        "varian".to_string(),
        Value::FungsiAsli {
            name: "varian".to_string(),
            id: MATRA_FN_VARIAN,
        },
    );
    map.insert(
        "deviasi_standar".to_string(),
        Value::FungsiAsli {
            name: "deviasi_standar".to_string(),
            id: MATRA_FN_DEVIASI_STANDAR,
        },
    );
    map.insert(
        "ubah_bentuk".to_string(),
        Value::FungsiAsli {
            name: "ubah_bentuk".to_string(),
            id: MATRA_FN_UBAH_BENTUK,
        },
    );
    map.insert(
        "ratakan".to_string(),
        Value::FungsiAsli {
            name: "ratakan".to_string(),
            id: MATRA_FN_RATAKAN,
        },
    );
    map.insert(
        "gabung".to_string(),
        Value::FungsiAsli {
            name: "gabung".to_string(),
            id: MATRA_FN_GABUNG,
        },
    );
    map.insert(
        "akar".to_string(),
        Value::FungsiAsli {
            name: "akar".to_string(),
            id: MATRA_FN_AKAR,
        },
    );
    map.insert(
        "mutlak".to_string(),
        Value::FungsiAsli {
            name: "mutlak".to_string(),
            id: MATRA_FN_MUTLAK,
        },
    );
    map.insert(
        "abs".to_string(),
        Value::FungsiAsli {
            name: "abs".to_string(),
            id: MATRA_FN_MUTLAK,
        },
    );
    map.insert(
        "sin".to_string(),
        Value::FungsiAsli {
            name: "sin".to_string(),
            id: MATRA_FN_SIN,
        },
    );
    map.insert(
        "cos".to_string(),
        Value::FungsiAsli {
            name: "cos".to_string(),
            id: MATRA_FN_COS,
        },
    );
    map.insert(
        "tan".to_string(),
        Value::FungsiAsli {
            name: "tan".to_string(),
            id: MATRA_FN_TAN,
        },
    );
    map.insert(
        "exp".to_string(),
        Value::FungsiAsli {
            name: "exp".to_string(),
            id: MATRA_FN_EXP,
        },
    );
    map.insert(
        "log".to_string(),
        Value::FungsiAsli {
            name: "log".to_string(),
            id: MATRA_FN_LOG,
        },
    );

    // Fungsi-fungsi pengolah data tabular / DataFrame (matra.tabel)
    map.insert(
        "tabel".to_string(),
        Value::FungsiAsli {
            name: "tabel".to_string(),
            id: MATRA_FN_TABEL,
        },
    );
    map.insert(
        "dataframe".to_string(),
        Value::FungsiAsli {
            name: "dataframe".to_string(),
            id: MATRA_FN_TABEL,
        },
    );
    map.insert(
        "baca_csv".to_string(),
        Value::FungsiAsli {
            name: "baca_csv".to_string(),
            id: MATRA_FN_BACA_CSV,
        },
    );
    map.insert(
        "tulis_csv".to_string(),
        Value::FungsiAsli {
            name: "tulis_csv".to_string(),
            id: MATRA_FN_TULIS_CSV,
        },
    );

    // Submodul larik (sebagai kamus bersarang untuk sintaks mt.larik.nol dsb)
    let mut larik_map = HashMap::new();
    for (k, v) in map.iter() {
        larik_map.insert(k.clone(), v.clone());
    }
    map.insert(
        "modul_larik".to_string(),
        Value::Kamus(Rc::new(RefCell::new(larik_map))),
    );

    // Submodul tabel (sebagai kamus bersarang untuk sintaks mt.tabel.baca_csv dsb)
    let mut tabel_map = HashMap::new();
    tabel_map.insert(
        "baca_csv".to_string(),
        Value::FungsiAsli {
            name: "baca_csv".to_string(),
            id: MATRA_FN_BACA_CSV,
        },
    );
    tabel_map.insert(
        "tulis_csv".to_string(),
        Value::FungsiAsli {
            name: "tulis_csv".to_string(),
            id: MATRA_FN_TULIS_CSV,
        },
    );
    map.insert(
        "modul_tabel".to_string(),
        Value::Kamus(Rc::new(RefCell::new(tabel_map))),
    );

    // Fungsi-fungsi visualisasi data (matra.grafik)
    let fn_garis = Value::FungsiAsli {
        name: "garis".to_string(),
        id: MATRA_FN_GRAFIK_GARIS,
    };
    let fn_sebar = Value::FungsiAsli {
        name: "sebar".to_string(),
        id: MATRA_FN_GRAFIK_SEBAR,
    };
    let fn_batang = Value::FungsiAsli {
        name: "batang".to_string(),
        id: MATRA_FN_GRAFIK_BATANG,
    };
    let fn_histogram = Value::FungsiAsli {
        name: "histogram".to_string(),
        id: MATRA_FN_GRAFIK_HISTOGRAM,
    };
    let fn_konfusi = Value::FungsiAsli {
        name: "matriks_konfusi".to_string(),
        id: MATRA_FN_GRAFIK_KONFUSI,
    };

    map.insert("garis".to_string(), fn_garis.clone());
    map.insert("sebar".to_string(), fn_sebar.clone());
    map.insert("batang".to_string(), fn_batang.clone());
    map.insert("histogram".to_string(), fn_histogram.clone());
    map.insert("matriks_konfusi".to_string(), fn_konfusi.clone());

    let mut grafik_map = HashMap::new();
    grafik_map.insert("garis".to_string(), fn_garis.clone());
    grafik_map.insert("plot".to_string(), fn_garis.clone());
    grafik_map.insert("sebar".to_string(), fn_sebar.clone());
    grafik_map.insert("scatter".to_string(), fn_sebar.clone());
    grafik_map.insert("batang".to_string(), fn_batang.clone());
    grafik_map.insert("bar".to_string(), fn_batang.clone());
    grafik_map.insert("histogram".to_string(), fn_histogram.clone());
    grafik_map.insert("hist".to_string(), fn_histogram.clone());
    grafik_map.insert("matriks_konfusi".to_string(), fn_konfusi.clone());
    grafik_map.insert("peta_panas".to_string(), fn_konfusi.clone());
    grafik_map.insert("heatmap".to_string(), fn_konfusi.clone());

    map.insert(
        "grafik".to_string(),
        Value::Kamus(Rc::new(RefCell::new(grafik_map.clone()))),
    );
    map.insert(
        "modul_grafik".to_string(),
        Value::Kamus(Rc::new(RefCell::new(grafik_map))),
    );

    // =========================================================================
    // FASE 9D: SUITE TRAINING, PREPROCESSING, PIPELINE, TESTING & TUNING
    // =========================================================================

    // Fungsi-fungsi pembagian data & evaluasi (matra.latih)
    let fn_pisah_data = Value::FungsiAsli {
        name: "pisah_data".to_string(),
        id: MATRA_FN_PISAH_DATA,
    };
    let fn_klipat = Value::FungsiAsli {
        name: "KLipat".to_string(),
        id: MATRA_FN_K_LIPAT,
    };
    let fn_validasi_silang = Value::FungsiAsli {
        name: "validasi_silang".to_string(),
        id: MATRA_FN_VALIDASI_SILANG,
    };

    map.insert("pisah_data".to_string(), fn_pisah_data.clone());
    map.insert("KLipat".to_string(), fn_klipat.clone());
    map.insert("validasi_silang".to_string(), fn_validasi_silang.clone());

    let mut latih_map = HashMap::new();
    latih_map.insert("pisah_data".to_string(), fn_pisah_data.clone());
    latih_map.insert("split".to_string(), fn_pisah_data.clone());
    latih_map.insert("KLipat".to_string(), fn_klipat.clone());
    latih_map.insert("KFold".to_string(), fn_klipat.clone());
    latih_map.insert("LipatanBerlapis".to_string(), fn_klipat.clone());
    latih_map.insert("validasi_silang".to_string(), fn_validasi_silang.clone());
    latih_map.insert("cross_val_score".to_string(), fn_validasi_silang.clone());
    map.insert(
        "latih".to_string(),
        Value::Kamus(Rc::new(RefCell::new(latih_map.clone()))),
    );
    map.insert(
        "modul_latih".to_string(),
        Value::Kamus(Rc::new(RefCell::new(latih_map))),
    );

    // Fungsi penyeimbang data (matra.seimbang)
    let fn_smote = Value::FungsiAsli {
        name: "SMOTE".to_string(),
        id: MATRA_FN_SMOTE,
    };
    map.insert("SMOTE".to_string(), fn_smote.clone());
    map.insert("smote".to_string(), fn_smote.clone());

    let mut seimbang_map = HashMap::new();
    seimbang_map.insert("SMOTE".to_string(), fn_smote.clone());
    seimbang_map.insert("smote".to_string(), fn_smote.clone());
    map.insert(
        "seimbang".to_string(),
        Value::Kamus(Rc::new(RefCell::new(seimbang_map.clone()))),
    );
    map.insert(
        "modul_seimbang".to_string(),
        Value::Kamus(Rc::new(RefCell::new(seimbang_map))),
    );

    // Fungsi prapemrosesan data (matra.pra)
    let fn_standarisasi = Value::FungsiAsli {
        name: "Standarisasi".to_string(),
        id: MATRA_FN_STANDARISASI,
    };
    let fn_min_maks = Value::FungsiAsli {
        name: "PenskalaMinMaks".to_string(),
        id: MATRA_FN_MIN_MAKS,
    };
    let fn_pengekod_label = Value::FungsiAsli {
        name: "PengekodLabel".to_string(),
        id: MATRA_FN_PENGEKOD_LABEL,
    };

    map.insert("Standarisasi".to_string(), fn_standarisasi.clone());
    map.insert("standarisasi".to_string(), fn_standarisasi.clone());
    map.insert("PenskalaMinMaks".to_string(), fn_min_maks.clone());
    map.insert("min_maks".to_string(), fn_min_maks.clone());
    map.insert("PengekodLabel".to_string(), fn_pengekod_label.clone());
    map.insert("pengekod_label".to_string(), fn_pengekod_label.clone());

    let mut pra_map = HashMap::new();
    pra_map.insert("Standarisasi".to_string(), fn_standarisasi.clone());
    pra_map.insert("standarisasi".to_string(), fn_standarisasi.clone());
    pra_map.insert("StandardScaler".to_string(), fn_standarisasi.clone());
    pra_map.insert("PenskalaMinMaks".to_string(), fn_min_maks.clone());
    pra_map.insert("min_maks".to_string(), fn_min_maks.clone());
    pra_map.insert("penskala_min_maks".to_string(), fn_min_maks.clone());
    pra_map.insert("MinMaxScaler".to_string(), fn_min_maks.clone());
    pra_map.insert("PengekodLabel".to_string(), fn_pengekod_label.clone());
    pra_map.insert("pengekod_label".to_string(), fn_pengekod_label.clone());
    pra_map.insert("LabelEncoder".to_string(), fn_pengekod_label.clone());
    map.insert(
        "pra".to_string(),
        Value::Kamus(Rc::new(RefCell::new(pra_map.clone()))),
    );
    map.insert(
        "modul_pra".to_string(),
        Value::Kamus(Rc::new(RefCell::new(pra_map))),
    );

    // Pipeline alur kerja (matra.alur)
    let fn_pipa = Value::FungsiAsli {
        name: "PipaKerja".to_string(),
        id: MATRA_FN_PIPA_KERJA,
    };
    map.insert("PipaKerja".to_string(), fn_pipa.clone());
    map.insert("pipa_kerja".to_string(), fn_pipa.clone());
    map.insert("Pipeline".to_string(), fn_pipa.clone());

    let mut alur_map = HashMap::new();
    alur_map.insert("PipaKerja".to_string(), fn_pipa.clone());
    alur_map.insert("pipa_kerja".to_string(), fn_pipa.clone());
    alur_map.insert("pipa".to_string(), fn_pipa.clone());
    alur_map.insert("Pipeline".to_string(), fn_pipa.clone());
    alur_map.insert("pipeline".to_string(), fn_pipa.clone());
    map.insert(
        "alur".to_string(),
        Value::Kamus(Rc::new(RefCell::new(alur_map.clone()))),
    );
    map.insert(
        "modul_alur".to_string(),
        Value::Kamus(Rc::new(RefCell::new(alur_map))),
    );

    // Pengujian diagnostik & integritas data (matra.uji)
    let fn_kebocoran = Value::FungsiAsli {
        name: "cek_kebocoran".to_string(),
        id: MATRA_FN_CEK_KEBOCORAN,
    };
    let fn_pergeseran = Value::FungsiAsli {
        name: "cek_pergeseran".to_string(),
        id: MATRA_FN_CEK_PERGESERAN,
    };

    map.insert("cek_kebocoran".to_string(), fn_kebocoran.clone());
    map.insert("cek_pergeseran".to_string(), fn_pergeseran.clone());

    let mut uji_map = HashMap::new();
    uji_map.insert("cek_kebocoran".to_string(), fn_kebocoran.clone());
    uji_map.insert("cek_leakage".to_string(), fn_kebocoran.clone());
    uji_map.insert("cek_pergeseran".to_string(), fn_pergeseran.clone());
    uji_map.insert("cek_drift".to_string(), fn_pergeseran.clone());
    map.insert(
        "uji".to_string(),
        Value::Kamus(Rc::new(RefCell::new(uji_map.clone()))),
    );
    map.insert(
        "modul_uji".to_string(),
        Value::Kamus(Rc::new(RefCell::new(uji_map))),
    );

    // Penyetelan hyperparameter (matra.tala)
    let fn_kisi = Value::FungsiAsli {
        name: "PencarianKisi".to_string(),
        id: MATRA_FN_PENCARIAN_KISI,
    };
    let fn_bayes = Value::FungsiAsli {
        name: "TalaBayes".to_string(),
        id: MATRA_FN_TALA_BAYES,
    };

    map.insert("PencarianKisi".to_string(), fn_kisi.clone());
    map.insert("TalaBayes".to_string(), fn_bayes.clone());

    let mut tala_map = HashMap::new();
    tala_map.insert("PencarianKisi".to_string(), fn_kisi.clone());
    tala_map.insert("GridSearchCV".to_string(), fn_kisi.clone());
    tala_map.insert("TalaBayes".to_string(), fn_bayes.clone());
    tala_map.insert("Optuna".to_string(), fn_bayes.clone());
    map.insert(
        "tala".to_string(),
        Value::Kamus(Rc::new(RefCell::new(tala_map.clone()))),
    );
    map.insert(
        "modul_tala".to_string(),
        Value::Kamus(Rc::new(RefCell::new(tala_map))),
    );

    // =========================================================================
    // FASE 9E: MODEL MACHINE LEARNING KLASIK & METRIK EVALUASI
    // =========================================================================

    // Fungsi-fungsi pembangun model (matra.model)
    let fn_reg_lin = Value::FungsiAsli {
        name: "RegresiLinear".to_string(),
        id: MATRA_FN_REGRESI_LINEAR,
    };
    let fn_reg_log = Value::FungsiAsli {
        name: "RegresiLogistik".to_string(),
        id: MATRA_FN_REGRESI_LOGISTIK,
    };
    let fn_knn_kla = Value::FungsiAsli {
        name: "KNNKlasifikasi".to_string(),
        id: MATRA_FN_KNN_KLASIFIKASI,
    };
    let fn_knn_reg = Value::FungsiAsli {
        name: "KNNRegresi".to_string(),
        id: MATRA_FN_KNN_REGRESI,
    };
    let fn_pohon = Value::FungsiAsli {
        name: "PohonKeputusan".to_string(),
        id: MATRA_FN_POHON_KEPUTUSAN,
    };
    let fn_hutan = Value::FungsiAsli {
        name: "HutanAcak".to_string(),
        id: MATRA_FN_HUTAN_ACAK,
    };
    let fn_kmeans = Value::FungsiAsli {
        name: "KMeans".to_string(),
        id: MATRA_FN_KMEANS,
    };

    map.insert("RegresiLinear".to_string(), fn_reg_lin.clone());
    map.insert("regresi_linear".to_string(), fn_reg_lin.clone());
    map.insert("RegresiLogistik".to_string(), fn_reg_log.clone());
    map.insert("regresi_logistik".to_string(), fn_reg_log.clone());
    map.insert("KNNKlasifikasi".to_string(), fn_knn_kla.clone());
    map.insert("knn_klasifikasi".to_string(), fn_knn_kla.clone());
    map.insert("KNNRegresi".to_string(), fn_knn_reg.clone());
    map.insert("knn_regresi".to_string(), fn_knn_reg.clone());
    map.insert("KNN".to_string(), fn_knn_kla.clone());
    map.insert("knn".to_string(), fn_knn_kla.clone());
    map.insert("PohonKeputusan".to_string(), fn_pohon.clone());
    map.insert("pohon_keputusan".to_string(), fn_pohon.clone());
    map.insert("HutanAcak".to_string(), fn_hutan.clone());
    map.insert("hutan_acak".to_string(), fn_hutan.clone());
    map.insert("KMeans".to_string(), fn_kmeans.clone());
    map.insert("kmeans".to_string(), fn_kmeans.clone());

    let mut model_map = HashMap::new();
    model_map.insert("RegresiLinear".to_string(), fn_reg_lin.clone());
    model_map.insert("regresi_linear".to_string(), fn_reg_lin.clone());
    model_map.insert("LinearRegression".to_string(), fn_reg_lin.clone());
    model_map.insert("linear_regression".to_string(), fn_reg_lin);
    model_map.insert("RegresiLogistik".to_string(), fn_reg_log.clone());
    model_map.insert("regresi_logistik".to_string(), fn_reg_log.clone());
    model_map.insert("LogisticRegression".to_string(), fn_reg_log.clone());
    model_map.insert("logistic_regression".to_string(), fn_reg_log);
    model_map.insert("KNNKlasifikasi".to_string(), fn_knn_kla.clone());
    model_map.insert("knn_klasifikasi".to_string(), fn_knn_kla.clone());
    model_map.insert("KNNClassifier".to_string(), fn_knn_kla.clone());
    model_map.insert("KNNRegresi".to_string(), fn_knn_reg.clone());
    model_map.insert("knn_regresi".to_string(), fn_knn_reg.clone());
    model_map.insert("KNNRegressor".to_string(), fn_knn_reg);
    model_map.insert("KNN".to_string(), fn_knn_kla.clone());
    model_map.insert("knn".to_string(), fn_knn_kla);
    model_map.insert("PohonKeputusan".to_string(), fn_pohon.clone());
    model_map.insert("pohon_keputusan".to_string(), fn_pohon.clone());
    model_map.insert("DecisionTree".to_string(), fn_pohon.clone());
    model_map.insert("decision_tree".to_string(), fn_pohon);
    model_map.insert("HutanAcak".to_string(), fn_hutan.clone());
    model_map.insert("hutan_acak".to_string(), fn_hutan.clone());
    model_map.insert("RandomForest".to_string(), fn_hutan.clone());
    model_map.insert("random_forest".to_string(), fn_hutan);
    model_map.insert("KMeans".to_string(), fn_kmeans.clone());
    model_map.insert("kmeans".to_string(), fn_kmeans);
    map.insert(
        "model".to_string(),
        Value::Kamus(Rc::new(RefCell::new(model_map.clone()))),
    );
    map.insert(
        "modul_model".to_string(),
        Value::Kamus(Rc::new(RefCell::new(model_map))),
    );

    // Fungsi-fungsi metrik evaluasi (matra.metrik)
    let fn_akurasi = Value::FungsiAsli {
        name: "akurasi".to_string(),
        id: MATRA_FN_METRIK_AKURASI,
    };
    let fn_presisi = Value::FungsiAsli {
        name: "presisi".to_string(),
        id: MATRA_FN_METRIK_PRESISI,
    };
    let fn_recall = Value::FungsiAsli {
        name: "recall".to_string(),
        id: MATRA_FN_METRIK_RECALL,
    };
    let fn_f1 = Value::FungsiAsli {
        name: "f1_skor".to_string(),
        id: MATRA_FN_METRIK_F1,
    };
    let fn_m_konfusi = Value::FungsiAsli {
        name: "matriks_konfusi".to_string(),
        id: MATRA_FN_METRIK_KONFUSI,
    };
    let fn_laporan = Value::FungsiAsli {
        name: "laporan_klasifikasi".to_string(),
        id: MATRA_FN_METRIK_LAPORAN,
    };
    let fn_mse = Value::FungsiAsli {
        name: "mse".to_string(),
        id: MATRA_FN_METRIK_MSE,
    };
    let fn_rmse = Value::FungsiAsli {
        name: "rmse".to_string(),
        id: MATRA_FN_METRIK_RMSE,
    };
    let fn_mae = Value::FungsiAsli {
        name: "mae".to_string(),
        id: MATRA_FN_METRIK_MAE,
    };
    let fn_r2 = Value::FungsiAsli {
        name: "r2_skor".to_string(),
        id: MATRA_FN_METRIK_R2,
    };
    let fn_mape = Value::FungsiAsli {
        name: "mape".to_string(),
        id: MATRA_FN_METRIK_MAPE,
    };

    map.insert("akurasi".to_string(), fn_akurasi.clone());
    map.insert("presisi".to_string(), fn_presisi.clone());
    map.insert("recall".to_string(), fn_recall.clone());
    map.insert("f1_skor".to_string(), fn_f1.clone());
    map.insert("laporan_klasifikasi".to_string(), fn_laporan.clone());
    map.insert("mse".to_string(), fn_mse.clone());
    map.insert("rmse".to_string(), fn_rmse.clone());
    map.insert("mae".to_string(), fn_mae.clone());
    map.insert("r2_skor".to_string(), fn_r2.clone());
    map.insert("mape".to_string(), fn_mape.clone());

    let mut metrik_map = HashMap::new();
    metrik_map.insert("akurasi".to_string(), fn_akurasi.clone());
    metrik_map.insert("accuracy".to_string(), fn_akurasi);
    metrik_map.insert("presisi".to_string(), fn_presisi.clone());
    metrik_map.insert("precision".to_string(), fn_presisi);
    metrik_map.insert("recall".to_string(), fn_recall.clone());
    metrik_map.insert("f1_skor".to_string(), fn_f1.clone());
    metrik_map.insert("f1_score".to_string(), fn_f1);
    metrik_map.insert("matriks_konfusi".to_string(), fn_m_konfusi.clone());
    metrik_map.insert("confusion_matrix".to_string(), fn_m_konfusi);
    metrik_map.insert("laporan_klasifikasi".to_string(), fn_laporan.clone());
    metrik_map.insert("classification_report".to_string(), fn_laporan);
    metrik_map.insert("mse".to_string(), fn_mse.clone());
    metrik_map.insert("rmse".to_string(), fn_rmse.clone());
    metrik_map.insert("mae".to_string(), fn_mae.clone());
    metrik_map.insert("r2_skor".to_string(), fn_r2.clone());
    metrik_map.insert("r2_score".to_string(), fn_r2);
    metrik_map.insert("mape".to_string(), fn_mape.clone());
    map.insert(
        "metrik".to_string(),
        Value::Kamus(Rc::new(RefCell::new(metrik_map.clone()))),
    );
    map.insert(
        "modul_metrik".to_string(),
        Value::Kamus(Rc::new(RefCell::new(metrik_map))),
    );

    // =========================================================================
    // FASE 9F: DEEP LEARNING (matra.syaraf) & AUTOMATED TRAINING (matra.pelatih)
    // =========================================================================

    // Fungsi-fungsi Jaringan Syaraf
    let fn_linear = Value::FungsiAsli {
        name: "Linear".to_string(),
        id: MATRA_FN_LINEAR,
    };
    let fn_relu = Value::FungsiAsli {
        name: "Relu".to_string(),
        id: MATRA_FN_RELU,
    };
    let fn_sigmoid = Value::FungsiAsli {
        name: "Sigmoid".to_string(),
        id: MATRA_FN_SIGMOID,
    };
    let fn_tanh = Value::FungsiAsli {
        name: "Tanh".to_string(),
        id: MATRA_FN_TANH,
    };
    let fn_softmax = Value::FungsiAsli {
        name: "Softmax".to_string(),
        id: MATRA_FN_SOFTMAX,
    };
    let fn_dropout = Value::FungsiAsli {
        name: "Dropout".to_string(),
        id: MATRA_FN_DROPOUT,
    };
    let fn_model_syaraf = Value::FungsiAsli {
        name: "ModelSyaraf".to_string(),
        id: MATRA_FN_MODEL_SYARAF,
    };
    let fn_loss_mse = Value::FungsiAsli {
        name: "KerugianMSE".to_string(),
        id: MATRA_FN_LOSS_MSE,
    };
    let fn_loss_bce = Value::FungsiAsli {
        name: "KerugianBCE".to_string(),
        id: MATRA_FN_LOSS_BCE,
    };
    let fn_loss_ce = Value::FungsiAsli {
        name: "KerugianKategori".to_string(),
        id: MATRA_FN_LOSS_CROSS_ENTROPY,
    };
    let fn_sgd = Value::FungsiAsli {
        name: "SGD".to_string(),
        id: MATRA_FN_OPTIM_SGD,
    };
    let fn_adam = Value::FungsiAsli {
        name: "Adam".to_string(),
        id: MATRA_FN_OPTIM_ADAM,
    };

    let mut syaraf_map = HashMap::new();
    syaraf_map.insert("Linear".to_string(), fn_linear.clone());
    syaraf_map.insert("linear".to_string(), fn_linear);
    syaraf_map.insert("Relu".to_string(), fn_relu.clone());
    syaraf_map.insert("relu".to_string(), fn_relu);
    syaraf_map.insert("Sigmoid".to_string(), fn_sigmoid.clone());
    syaraf_map.insert("sigmoid".to_string(), fn_sigmoid);
    syaraf_map.insert("Tanh".to_string(), fn_tanh.clone());
    syaraf_map.insert("tanh".to_string(), fn_tanh);
    syaraf_map.insert("Softmax".to_string(), fn_softmax.clone());
    syaraf_map.insert("softmax".to_string(), fn_softmax);
    syaraf_map.insert("Dropout".to_string(), fn_dropout.clone());
    syaraf_map.insert("dropout".to_string(), fn_dropout);
    syaraf_map.insert("Model".to_string(), fn_model_syaraf.clone());
    syaraf_map.insert("model".to_string(), fn_model_syaraf.clone());
    syaraf_map.insert("Sekuensial".to_string(), fn_model_syaraf.clone());
    syaraf_map.insert("sekuensial".to_string(), fn_model_syaraf);
    syaraf_map.insert("KerugianMSE".to_string(), fn_loss_mse.clone());
    syaraf_map.insert("mse".to_string(), fn_loss_mse.clone());
    syaraf_map.insert("KerugianBCE".to_string(), fn_loss_bce.clone());
    syaraf_map.insert("bce".to_string(), fn_loss_bce.clone());
    syaraf_map.insert("KerugianKategori".to_string(), fn_loss_ce.clone());
    syaraf_map.insert("CrossEntropy".to_string(), fn_loss_ce.clone());
    syaraf_map.insert("cross_entropy".to_string(), fn_loss_ce);
    syaraf_map.insert("SGD".to_string(), fn_sgd.clone());
    syaraf_map.insert("sgd".to_string(), fn_sgd);
    syaraf_map.insert("Adam".to_string(), fn_adam.clone());
    syaraf_map.insert("adam".to_string(), fn_adam);

    map.insert(
        "syaraf".to_string(),
        Value::Kamus(Rc::new(RefCell::new(syaraf_map.clone()))),
    );
    map.insert(
        "modul_syaraf".to_string(),
        Value::Kamus(Rc::new(RefCell::new(syaraf_map))),
    );

    // Fungsi-fungsi Pelatih Otomatis
    let fn_pelatih = Value::FungsiAsli {
        name: "Pelatih".to_string(),
        id: MATRA_FN_PELATIH,
    };
    let fn_cb_stop = Value::FungsiAsli {
        name: "BerhentiAwal".to_string(),
        id: MATRA_FN_CB_BERHENTI_AWAL,
    };
    let fn_cb_save = Value::FungsiAsli {
        name: "SimpanTerbaik".to_string(),
        id: MATRA_FN_CB_SIMPAN_TERBAIK,
    };
    let fn_cb_log = Value::FungsiAsli {
        name: "PencatatRiwayat".to_string(),
        id: MATRA_FN_CB_PENCATAT_RIWAYAT,
    };

    let mut pelatih_map = HashMap::new();
    pelatih_map.insert("Pelatih".to_string(), fn_pelatih.clone());
    pelatih_map.insert("pelatih".to_string(), fn_pelatih);
    pelatih_map.insert("BerhentiAwal".to_string(), fn_cb_stop.clone());
    pelatih_map.insert("berhenti_awal".to_string(), fn_cb_stop.clone());
    pelatih_map.insert("EarlyStopping".to_string(), fn_cb_stop);
    pelatih_map.insert("SimpanTerbaik".to_string(), fn_cb_save.clone());
    pelatih_map.insert("simpan_terbaik".to_string(), fn_cb_save.clone());
    pelatih_map.insert("ModelCheckpoint".to_string(), fn_cb_save);
    pelatih_map.insert("PencatatRiwayat".to_string(), fn_cb_log.clone());
    pelatih_map.insert("pencatat_riwayat".to_string(), fn_cb_log);

    map.insert(
        "pelatih".to_string(),
        Value::Kamus(Rc::new(RefCell::new(pelatih_map.clone()))),
    );
    map.insert(
        "modul_pelatih".to_string(),
        Value::Kamus(Rc::new(RefCell::new(pelatih_map))),
    );

    Value::Kamus(Rc::new(RefCell::new(map)))
}
