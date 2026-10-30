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
fn test_grafik_garis_dan_method_chaining() {
    let kode = r##"
        impor "matra" sebagai mt

        variabel x = [1, 2, 3, 4, 5]
        variabel y = [10, 25, 40, 35, 60]

        variabel plot = mt.grafik.garis(x, y)
            .judul("Pertumbuhan Pengguna")
            .label_x("Bulan")
            .label_y("Jumlah (Ribu)")
            .warna("#3B82F6")
            .ukuran(800, 450)
            .tampil()

        variabel svg = plot.ke_svg()
        kembali svg
    "##;

    let (res, out_lines) = eksekusi_output(kode);
    assert!(res.is_ok(), "Gagal menjalankan grafik garis: {:?}", res.err());

    let svg_str = res.unwrap().to_string();
    assert!(svg_str.contains("<svg"), "Harus menghasilkan string SVG");
    assert!(svg_str.contains("Pertumbuhan Pengguna"), "SVG harus memuat judul");

    let combined_out = out_lines.join("\n");
    assert!(combined_out.contains("Pertumbuhan Pengguna"), "Output terminal harus memuat judul");
    assert!(combined_out.contains("*"), "Output terminal harus memuat titik plot (*)");
    assert!(combined_out.contains("│"), "Output terminal harus memuat sumbu grafik");
}

#[test]
fn test_grafik_sebar_dengan_larik() {
    let kode = r##"
        impor "matra" sebagai mt

        variabel x = mt.larik([10, 20, 30, 40, 50])
        variabel y = mt.larik([15, 28, 32, 45, 58])

        variabel scatter = mt.sebar(x, y)
            .judul("Sebaran Sampel")
            .warna("#10B981")

        kembali scatter.ke_svg()
    "##;

    let res = eksekusi(kode);
    assert!(res.is_ok(), "Gagal grafik sebar: {:?}", res.err());
    let svg = res.unwrap().to_string();
    assert!(svg.contains("<circle"), "SVG sebar harus memuat elemen circle");
}

#[test]
fn test_diagram_batang() {
    let kode = r##"
        impor "matra" sebagai mt

        variabel kategori = ["Jan", "Feb", "Mar", "Apr"]
        variabel omset = [45.5, 78.2, 92.0, 65.4]

        variabel bar = mt.batang(kategori, omset)
            .judul("Omset Bulanan")
            .label_x("Bulan")
            .label_y("Juta Rupiah")
            .tampil()

        kembali bar.ke_svg()
    "##;

    let (res, out_lines) = eksekusi_output(kode);
    assert!(res.is_ok(), "Gagal diagram batang: {:?}", res.err());

    let txt = out_lines.join("\n");
    assert!(txt.contains("Jan"), "Terminal harus ada kategori Jan");
    assert!(txt.contains("Apr"), "Terminal harus ada kategori Apr");
    assert!(txt.contains("Omset Bulanan"), "Terminal harus ada judul");

    let svg = res.unwrap().to_string();
    assert!(svg.contains("<rect"), "SVG batang harus memuat elemen rect");
}

#[test]
fn test_histogram_frekuensi() {
    let kode = r##"
        impor "matra" sebagai mt

        variabel data_nilai = [60, 65, 68, 70, 72, 75, 78, 80, 82, 85, 88, 90, 95, 98]
        variabel hist = mt.histogram(data_nilai, 5)
            .judul("Distribusi Nilai Siswa")
            .label_x("Rentang Skor")
            .label_y("Frekuensi")
            .tampil()

        kembali hist.ke_svg()
    "##;

    let (res, out_lines) = eksekusi_output(kode);
    assert!(res.is_ok(), "Gagal histogram: {:?}", res.err());

    let txt = out_lines.join("\n");
    assert!(txt.contains("Distribusi Nilai Siswa"));
}

#[test]
fn test_matriks_konfusi_ml() {
    let kode = r##"
        impor "matra" sebagai mt

        variabel cm = [
            [85, 15],
            [10, 90]
        ]
        variabel kelas = ["Negatif", "Positif"]

        variabel plot_cm = mt.matriks_konfusi(cm, kelas)
            .judul("Diagnostik Model Evaluasi")
            .tampil()

        kembali plot_cm.ke_svg()
    "##;

    let (res, out_lines) = eksekusi_output(kode);
    assert!(res.is_ok(), "Gagal matriks konfusi: {:?}", res.err());

    let txt = out_lines.join("\n");
    assert!(txt.contains("Diagnostik Model Evaluasi"));
    assert!(txt.contains("Negatif"));
    assert!(txt.contains("Positif"));
    assert!(txt.contains("85"));
    assert!(txt.contains("90"));
}

#[test]
fn test_simpan_dan_baca_svg() {
    let kode = r##"
        impor "matra" sebagai mt

        variabel p = mt.garis([1, 2, 3], [4, 5, 6])
            .judul("Tes Simpan")
            .simpan_svg("test_output_plot.svg")
    "##;

    let res = eksekusi(kode);
    assert!(res.is_ok(), "Gagal simpan_svg: {:?}", res.err());

    let path = std::path::Path::new("test_output_plot.svg");
    assert!(path.exists(), "Berkas SVG harus tercipta di disk");
    let content = std::fs::read_to_string(path).expect("Harus dapat membaca berkas SVG");
    assert!(content.contains("<svg"), "Konten berkas harus berupa SVG valid");
    assert!(content.contains("Tes Simpan"), "Konten berkas harus memuat judul");

    let _ = std::fs::remove_file(path);
}

#[test]
fn test_error_handling_dimensi_berbeda() {
    let kode = r##"
        impor "matra" sebagai mt

        variabel status = "aman"
        coba {
            mt.garis([1, 2, 3], [10, 20])
        } tangkap (e) {
            status = "tertangkap"
        }
        kembali status
    "##;

    let res = eksekusi(kode);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), Value::Teks("tertangkap".to_string()));
}
