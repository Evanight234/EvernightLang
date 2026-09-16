use std::fs;
use std::process::Command;

fn get_bin() -> &'static str {
    env!("CARGO_BIN_EXE_evernight")
}

#[test]
fn test_cli_versi() {
    let output = Command::new(get_bin())
        .arg("--versi")
        .output()
        .expect("Gagal menjalankan biner");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("EvernightLanguage v0.1.0"));
}

#[test]
fn test_cli_bantuan() {
    let output = Command::new(get_bin())
        .arg("--bantuan")
        .output()
        .expect("Gagal menjalankan biner");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("PENGGUNAAN:"));
    assert!(stdout.contains("run <berkas.eve>"));
    assert!(stdout.contains("--run"));
    assert!(stdout.contains("--cek"));
    assert!(stdout.contains("--debug"));
    assert!(stdout.contains("--waktu"));
}

#[test]
fn test_cli_cek_valid() {
    let temp_dir = std::env::temp_dir().join("eve_test_cek");
    let _ = fs::create_dir_all(&temp_dir);
    let file = temp_dir.join("valid.eve");
    fs::write(&file, "variabel x = 10\ncetak(x)").unwrap();

    let output = Command::new(get_bin())
        .arg(&file)
        .arg("--cek")
        .output()
        .expect("Gagal menjalankan biner");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Pemeriksaan berhasil"));
    let _ = fs::remove_file(file);
}

#[test]
fn test_cli_default_run() {
    let temp_dir = std::env::temp_dir().join("eve_test_run");
    let _ = fs::create_dir_all(&temp_dir);
    let file = temp_dir.join("run.eve");
    fs::write(&file, "cetak(\"Hello dari default-run!\")").unwrap();

    let output = Command::new(get_bin())
        .arg(&file)
        .output()
        .expect("Gagal menjalankan biner");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Hello dari default-run!"));
    let _ = fs::remove_file(file);
}

#[test]
fn test_cli_subcommand_run() {
    let temp_dir = std::env::temp_dir().join("eve_test_sub_run");
    let _ = fs::create_dir_all(&temp_dir);
    let file = temp_dir.join("sub_run.eve");
    fs::write(&file, "cetak(\"Hello dari evernight run!\")").unwrap();

    let output = Command::new(get_bin())
        .arg("run")
        .arg(&file)
        .output()
        .expect("Gagal menjalankan biner");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Hello dari evernight run!"));
    let _ = fs::remove_file(file);
}

#[test]
fn test_cli_subcommand_run_tanpa_berkas() {
    let output = Command::new(get_bin())
        .arg("run")
        .output()
        .expect("Gagal menjalankan biner");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("BAHAYA [ARG]"));
    assert!(stderr.contains("Subcommand 'run' membutuhkan path berkas .eve!"));
}

#[test]
fn test_cli_argumen() {
    let temp_dir = std::env::temp_dir().join("eve_test_args");
    let _ = fs::create_dir_all(&temp_dir);
    let file = temp_dir.join("args.eve");
    fs::write(
        &file,
        "variabel args = argumen()\ncetak(args[0], \" \", args[1])",
    )
    .unwrap();

    let output = Command::new(get_bin())
        .arg(&file)
        .arg("--")
        .arg("satu")
        .arg("dua")
        .output()
        .expect("Gagal menjalankan biner");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("satu dua"));
    let _ = fs::remove_file(file);
}

#[test]
fn test_cli_bytecode() {
    let temp_dir = std::env::temp_dir().join("eve_test_bc");
    let _ = fs::create_dir_all(&temp_dir);
    let file = temp_dir.join("bc.eve");
    fs::write(&file, "kembali 10 + 20").unwrap();

    let output = Command::new(get_bin())
        .arg(&file)
        .arg("--bytecode")
        .output()
        .expect("Gagal menjalankan biner");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("BYTECODE DISASSEMBLY"));
    assert!(stdout.contains("Konstanta"));
    assert!(stdout.contains("Tambah"));
    let _ = fs::remove_file(file);
}

#[test]
fn test_cli_profiler() {
    let temp_dir = std::env::temp_dir().join("eve_test_profil");
    let _ = fs::create_dir_all(&temp_dir);
    let file = temp_dir.join("profil.eve");
    fs::write(&file, "variabel x = 2\ncetak(x * 3)").unwrap();

    let output = Command::new(get_bin())
        .arg("run")
        .arg(&file)
        .arg("--waktu")
        .output()
        .expect("Gagal menjalankan biner");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("=== PROFIL EvernightLanguage ==="));
    assert!(stdout.contains("Total instruksi:"));
    assert!(stdout.contains("Opcode"));
    assert!(stdout.contains("Kali"));
    let _ = fs::remove_file(file);
}

#[test]
fn test_cli_debug_trace() {
    let temp_dir = std::env::temp_dir().join("eve_test_debug");
    let _ = fs::create_dir_all(&temp_dir);
    let file = temp_dir.join("debug.eve");
    fs::write(&file, "cetak(1 + 2)").unwrap();

    let output = Command::new(get_bin())
        .arg("run")
        .arg(&file)
        .arg("--debug")
        .output()
        .expect("Gagal menjalankan biner");

    assert!(output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("[0000]"));
    assert!(stderr.contains("Tambah"));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("3"));
    let _ = fs::remove_file(file);
}

#[test]
fn test_cli_format_merapikan() {
    let dir = std::env::temp_dir().join("eve_test_format");
    let _ = fs::create_dir_all(&dir);
    let f = dir.join("rapikan.eve");
    fs::write(&f, "variabel x=1+2\ncetak(x)\n").unwrap();

    let out = Command::new(get_bin())
        .arg("format")
        .arg(&f)
        .output()
        .expect("gagal jalankan biner");
    assert!(out.status.success());
    let isi = fs::read_to_string(&f).unwrap();
    assert_eq!(isi, "variabel x = 1 + 2\ncetak(x)\n");
    let _ = fs::remove_file(f);
}

#[test]
fn test_cli_format_cek_gagal_bila_belum_rapi() {
    let dir = std::env::temp_dir().join("eve_test_format_cek");
    let _ = fs::create_dir_all(&dir);
    let f = dir.join("kotor.eve");
    fs::write(&f, "variabel x=1\n").unwrap();

    let out = Command::new(get_bin())
        .arg("format")
        .arg(&f)
        .arg("--cek")
        .output()
        .expect("gagal jalankan biner");
    assert!(
        !out.status.success(),
        "harus exit 1 untuk berkas belum rapi"
    );
    let _ = fs::remove_file(f);
}

#[test]
fn test_cli_format_cek_lolos_bila_rapi() {
    let dir = std::env::temp_dir().join("eve_test_format_rapi");
    let _ = fs::create_dir_all(&dir);
    let f = dir.join("rapi.eve");
    fs::write(&f, "variabel x = 1\ncetak(x)\n").unwrap();

    let out = Command::new(get_bin())
        .arg("format")
        .arg(&f)
        .arg("--cek")
        .output()
        .expect("gagal jalankan biner");
    assert!(out.status.success());
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.contains("Sudah rapi"));
    let _ = fs::remove_file(f);
}

#[test]
fn test_cli_format_keluar_ke_berkas_lain() {
    let dir = std::env::temp_dir().join("eve_test_format_keluar");
    let _ = fs::create_dir_all(&dir);
    let asal = dir.join("asal.eve");
    let tujuan = dir.join("tujuan.eve");
    fs::write(&asal, "variabel x=1\n").unwrap();

    let out = Command::new(get_bin())
        .arg("format")
        .arg(&asal)
        .arg("--keluar")
        .arg(&tujuan)
        .output()
        .expect("gagal jalankan biner");
    assert!(out.status.success());
    assert_eq!(
        fs::read_to_string(&asal).unwrap(),
        "variabel x=1\n",
        "asal tidak boleh berubah"
    );
    assert_eq!(fs::read_to_string(&tujuan).unwrap(), "variabel x = 1\n");
    let _ = fs::remove_file(asal);
    let _ = fs::remove_file(tujuan);
}

#[test]
fn test_cli_format_pertahankan_komentar() {
    let dir = std::env::temp_dir().join("eve_test_format_komentar");
    let _ = fs::create_dir_all(&dir);
    let f = dir.join("komentar.eve");
    fs::write(&f, "# catatan atas\nvariabel x = 1  # ekor\n").unwrap();

    let out = Command::new(get_bin())
        .arg("format")
        .arg(&f)
        .output()
        .expect("gagal jalankan biner");
    assert!(out.status.success());
    let isi = fs::read_to_string(&f).unwrap();
    assert!(isi.contains("# catatan atas"), "isi: {}", isi);
    assert!(isi.contains("# ekor"), "isi: {}", isi);
    let _ = fs::remove_file(f);
}

#[test]
fn test_cli_format_tanpa_argumen() {
    let out = Command::new(get_bin())
        .arg("format")
        .output()
        .expect("gagal jalankan biner");
    assert!(!out.status.success());
    let s = String::from_utf8_lossy(&out.stderr);
    assert!(s.contains("BAHAYA [ARG]"));
}

#[test]
fn test_cli_lint_bersih() {
    let dir = std::env::temp_dir().join("eve_test_lint_bersih");
    let _ = fs::create_dir_all(&dir);
    let f = dir.join("bersih.eve");
    fs::write(
        &f,
        "fungsi tambah(a, b) {\n    kembali a + b\n}\ncetak(tambah(1, 2))\n",
    )
    .unwrap();

    let out = Command::new(get_bin())
        .arg("lint")
        .arg(&f)
        .arg("--tanpa-warna")
        .output()
        .expect("gagal jalankan biner");
    assert!(
        out.status.success(),
        "harus bersih: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.contains("Bersih"));
    let _ = fs::remove_file(f);
}

#[test]
fn test_cli_lint_menemukan_pelanggaran() {
    let dir = std::env::temp_dir().join("eve_test_lint_kotor");
    let _ = fs::create_dir_all(&dir);
    let f = dir.join("kotor.eve");
    fs::write(&f, "variabel namaSaya = 1\ncetak(namaSaya)\n").unwrap();

    let out = Command::new(get_bin())
        .arg("lint")
        .arg(&f)
        .arg("--tanpa-warna")
        .output()
        .expect("gagal jalankan biner");
    assert!(!out.status.success(), "harus exit 1 bila ada peringatan");
    let s = String::from_utf8_lossy(&out.stderr);
    assert!(s.contains("WKHURUF"), "stderr: {}", s);
    let _ = fs::remove_file(f);
}

#[test]
fn test_cli_lint_berkas_ber_bom() {
    // Regresi: editor Windows sering menambah BOM UTF-8.
    let dir = std::env::temp_dir().join("eve_test_lint_bom");
    let _ = fs::create_dir_all(&dir);
    let f = dir.join("bom.eve");
    let mut isi = vec![0xEF, 0xBB, 0xBF];
    isi.extend_from_slice(b"cetak(1)\n");
    fs::write(&f, &isi).unwrap();

    let out = Command::new(get_bin())
        .arg("lint")
        .arg(&f)
        .arg("--tanpa-warna")
        .output()
        .expect("gagal jalankan biner");
    let s = String::from_utf8_lossy(&out.stderr);
    assert!(
        !s.contains("Karakter tidak dikenal"),
        "BOM harus diabaikan: {}",
        s
    );
    let _ = fs::remove_file(f);
}

#[test]
fn test_cli_pkg_help() {
    let out = Command::new(get_bin())
        .args(["pkg", "--bantuan"])
        .output()
        .expect("gagal jalankan biner");
    assert!(out.status.success());
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.contains("ever pkg"));
    assert!(s.contains("init"));
    assert!(s.contains("jalankan"));
    assert!(s.contains("daftar"));
}

#[test]
fn test_cli_pkg_init_dan_jalankan() {
    let dir = std::env::temp_dir().join("eve_test_pkg_init");
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();

    let out = Command::new(get_bin())
        .args(["pkg", "init", "proyek-uji"])
        .current_dir(&dir)
        .output()
        .expect("gagal jalankan biner");
    assert!(
        out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(dir.join("eve.json").exists());
    assert!(dir.join("utama.eve").exists());

    let isi = fs::read_to_string(dir.join("eve.json")).unwrap();
    assert!(isi.contains("proyek-uji"), "manifest: {}", isi);

    let out = Command::new(get_bin())
        .args(["pkg", "jalankan"])
        .current_dir(&dir)
        .output()
        .expect("gagal jalankan biner");
    assert!(
        out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.contains("Halo dari proyek-uji"), "stdout: {}", s);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_cli_pkg_init_menolak_duplikat() {
    let dir = std::env::temp_dir().join("eve_test_pkg_dobel");
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();

    let _ = Command::new(get_bin())
        .args(["pkg", "init"])
        .current_dir(&dir)
        .output()
        .unwrap();
    let out = Command::new(get_bin())
        .args(["pkg", "init"])
        .current_dir(&dir)
        .output()
        .unwrap();
    assert!(!out.status.success(), "init kedua harus gagal");
    let e = String::from_utf8_lossy(&out.stderr);
    assert!(e.contains("BAHAYA [PKG]"), "stderr: {}", e);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_cli_pkg_daftar_tanpa_berkas() {
    let dir = std::env::temp_dir().join("eve_test_pkg_kosong");
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();

    let out = Command::new(get_bin())
        .args(["pkg", "daftar"])
        .current_dir(&dir)
        .output()
        .expect("gagal jalankan biner");
    assert!(out.status.success());
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.contains("Tidak ada berkas .eve"));

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn test_cli_pkg_subperintah_tidak_dikenal() {
    let out = Command::new(get_bin())
        .args(["pkg", "ngawur"])
        .output()
        .expect("gagal jalankan biner");
    assert!(!out.status.success());
    let e = String::from_utf8_lossy(&out.stderr);
    assert!(e.contains("BAHAYA [PKG]"));
}

#[test]
fn test_cli_file_not_found() {
    let output = Command::new(get_bin())
        .arg("berkas_khayalan_tidak_ada.eve")
        .output()
        .expect("Gagal menjalankan biner");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("BAHAYA [FILE]"));
}

#[test]
fn test_cli_syntax_error_snippet() {
    let temp_dir = std::env::temp_dir().join("eve_test_err");
    let _ = fs::create_dir_all(&temp_dir);
    let file = temp_dir.join("err.eve");
    fs::write(&file, "variabel 123invalid = 1").unwrap();

    let output = Command::new(get_bin())
        .arg(&file)
        .arg("--tanpa-warna")
        .output()
        .expect("Gagal menjalankan biner");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("BAHAYA"));
    assert!(stderr.contains("1 | variabel 123invalid = 1"));
    assert!(stderr.contains("^"));
    let _ = fs::remove_file(file);
}
