//! Logika pemasangan installer (Fase 6F).
//!
//! Ini adalah pindahan dari `paket/evernight-0.1.0-windows-x64/install.ps1`
//! yang sudah terbukti bekerja (PATH, asosiasi `.eve`, deteksi editor,
//! uninstall). Ditulis ulang dalam Rust agar installer mandiri satu berkas.
//!
//! Operasi sistem memakai `reg.exe` dan PowerShell -- tanpa FFI, tanpa dependensi
//! tambahan, dan perilakunya sama dengan skrip yang sudah teruji.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub const NAMA_PRODUK: &str = "EvernightLanguage";
pub const VERSI: &str = "0.1.0";
pub const PROGID: &str = "EvernightFile";
pub const LABEL_TIPE: &str = "Evernight files";
pub const KUNCI_UNINSTALL: &str = r"HKCU\Software\Microsoft\Windows\CurrentVersion\Uninstall\EvernightLanguage";

/// Berkas yang dibawa installer (di-embed saat kompilasi).
pub struct Payload {
    pub evernight_exe: &'static [u8],
    pub vsix: &'static [u8],
    pub panduan: &'static [u8],
    pub icon_ico: &'static [u8],
    pub logo_png: &'static [u8],
    pub lisensi: &'static [u8],
}

/// Pilihan pengguna dari halaman wizard.
#[derive(Clone, Debug)]
pub struct Rencana {
    pub tujuan: PathBuf,
    pub per_mesin: bool,
    pub tambah_path: bool,
    pub asosiasi_eve: bool,
    pub pasang_ekstensi: bool,
    pub buat_pintasan: bool,
}

impl Default for Rencana {
    fn default() -> Self {
        Self {
            tujuan: PathBuf::new(),
            per_mesin: false,
            tambah_path: true,
            asosiasi_eve: true,
            pasang_ekstensi: true,
            buat_pintasan: true,
        }
    }
}

impl Rencana {
    /// Folder tujuan default untuk mode per-user.
    pub fn tujuan_default(per_mesin: bool) -> PathBuf {
        if per_mesin {
            PathBuf::from(r"C:\Program Files\Evernight")
        } else {
            let lokal = std::env::var("LOCALAPPDATA").unwrap_or_else(|_| ".".to_string());
            Path::new(&lokal).join("Programs").join("Evernight")
        }
    }

    /// Folder `bin` hasil pemasangan.
    pub fn bin(&self) -> PathBuf {
        self.tujuan.join("bin")
    }

    pub fn exe(&self) -> PathBuf {
        self.bin().join("evernight.exe")
    }
}

/// Pesan kemajuan dari pekerja pemasangan ke UI.
#[derive(Debug, Clone)]
pub enum Pesan {
    Langkah { persen: f32, teks: String },
    Selesai(Result<(), String>),
}

/// Editor keluarga VS Code yang terdeteksi.
#[derive(Clone, Debug)]
pub struct Editor {
    pub nama: String,
    pub cli: PathBuf,
}

/// Deteksi editor keluarga VS Code (logika sama dengan 6C).
///
/// Sengaja TIDAK memakai keberadaan folder data (`.cursor`, `.windsurf`) --
/// folder itu bisa ada tanpa aplikasi terpasang.
pub fn deteksi_editor() -> Vec<Editor> {
    const KANDIDAT: &[(&str, &str)] = &[
        ("Antigravity IDE", r"Programs\Antigravity IDE\bin\antigravity-ide.cmd"),
        ("Visual Studio Code", r"Programs\Microsoft VS Code\bin\code.cmd"),
        ("VS Code Insiders", r"Programs\Microsoft VS Code Insiders\bin\code-insiders.cmd"),
        ("Cursor", r"Programs\cursor\resources\app\bin\cursor.cmd"),
        ("Windsurf", r"Programs\Windsurf\bin\windsurf.cmd"),
        ("VSCodium", r"Programs\VSCodium\bin\codium.cmd"),
    ];

    let lokal = std::env::var("LOCALAPPDATA").unwrap_or_default();
    let mut hasil = Vec::new();
    for (nama, relatif) in KANDIDAT {
        let path = Path::new(&lokal).join(relatif);
        if path.exists() {
            hasil.push(Editor {
                nama: nama.to_string(),
                cli: path,
            });
        }
    }
    hasil
}

// ---------------------------------------------------------------------------
// Pemasangan
// ---------------------------------------------------------------------------

/// Jalankan seluruh rangkaian pemasangan. Melaporkan kemajuan lewat `lapor`.
pub fn jalankan(rencana: &Rencana, payload: &Payload, lapor: &mut dyn FnMut(Pesan)) -> Result<(), String> {
    let total = 7.0f32;
    let mut n = 0.0f32;
    // Helper: majukan langkah lalu lapor. Ditulis sebagai fungsi bebas agar
    // tidak menahan borrow `lapor` (yang juga dipakai untuk pesan peringatan).
    fn maju(n: &mut f32, total: f32, lapor: &mut dyn FnMut(Pesan), teks: &str) {
        *n += 1.0;
        lapor(Pesan::Langkah {
            persen: (*n / total).min(1.0),
            teks: teks.to_string(),
        });
    }

    // 1. Siapkan folder tujuan
    maju(&mut n, total, lapor, "Menyiapkan folder tujuan...");
    fs::create_dir_all(rencana.bin()).map_err(|e| format!("Gagal membuat folder bin: {}", e))?;
    fs::create_dir_all(rencana.tujuan.join("assets")).map_err(|e| format!("Gagal membuat assets: {}", e))?;
    fs::create_dir_all(rencana.tujuan.join("docs")).map_err(|e| format!("Gagal membuat docs: {}", e))?;
    fs::create_dir_all(rencana.tujuan.join("extensions"))
        .map_err(|e| format!("Gagal membuat extensions: {}", e))?;

    // 2. Tulis berkas program
    maju(&mut n, total, lapor, "Menyalin evernight.exe...");
    fs::write(rencana.exe(), payload.evernight_exe)
        .map_err(|e| format!("Gagal menulis evernight.exe: {}", e))?;

    maju(&mut n, total, lapor, "Menyalin aset & dokumentasi...");
    fs::write(rencana.tujuan.join("assets").join("icon.ico"), payload.icon_ico)
        .map_err(|e| format!("Gagal menulis icon.ico: {}", e))?;
    fs::write(rencana.tujuan.join("assets").join("logo.png"), payload.logo_png)
        .map_err(|e| format!("Gagal menulis logo.png: {}", e))?;
    fs::write(rencana.tujuan.join("docs").join("PANDUAN.txt"), payload.panduan)
        .map_err(|e| format!("Gagal menulis PANDUAN.txt: {}", e))?;
    fs::write(rencana.tujuan.join("LICENSE"), payload.lisensi)
        .map_err(|e| format!("Gagal menulis LICENSE: {}", e))?;

    let vsix_path = rencana
        .tujuan
        .join("extensions")
        .join("evernight-language-0.1.0.vsix");
    fs::write(&vsix_path, payload.vsix).map_err(|e| format!("Gagal menulis .vsix: {}", e))?;

    // Salin installer ini sendiri sebagai `uninstall.exe` di folder instalasi.
    // Pola standar installer: entri "Apps & Features" tetap berfungsi walau
    // Setup.exe asli sudah dipindahkan atau dihapus oleh pengguna.
    if let Ok(diri) = std::env::current_exe() {
        let _ = fs::copy(&diri, rencana.tujuan.join("uninstall.exe"));
    }

    // 3. PATH
    maju(&mut n, total, lapor, "Mengatur PATH...");
    if rencana.tambah_path {
        let bin = rencana.bin().to_string_lossy().to_string();
        if let Err(e) = tambah_path(&bin, rencana.per_mesin) {
            // PATH gagal tidak fatal -- beri tahu lewat teks langkah berikutnya.
            lapor(Pesan::Langkah {
                persen: (n / total).min(1.0),
                teks: format!("Peringatan PATH: {}", e),
            });
        }
    }

    // 4. Asosiasi berkas .eve
    maju(&mut n, total, lapor, "Mengasosiasikan berkas .eve...");
    if rencana.asosiasi_eve {
        if let Err(e) = asosiasi_eve(rencana) {
            lapor(Pesan::Langkah {
                persen: (n / total).min(1.0),
                teks: format!("Peringatan asosiasi: {}", e),
            });
        }
    }

    // 5. Ekstensi editor
    maju(&mut n, total, lapor, "Memasang ekstensi editor...");
    if rencana.pasang_ekstensi {
        for ed in deteksi_editor() {
            let _ = pasang_ekstensi(&ed, &vsix_path);
        }
    }

    // 6. Pintasan
    maju(&mut n, total, lapor, "Membuat pintasan...");
    if rencana.buat_pintasan {
        let _ = buat_pintasan(rencana);
    }

    // 7. Daftarkan di Apps & Features
    maju(&mut n, total, lapor, "Mendaftarkan di Apps & Features...");
    daftar_uninstall(rencana).map_err(|e| format!("Gagal mendaftarkan uninstaller: {}", e))?;

    Ok(())
}

// ---------------------------------------------------------------------------
// PATH
// ---------------------------------------------------------------------------

/// Tambahkan folder `bin` ke PATH pengguna (atau mesin bila elevated).
///
/// Idempoten: bila sudah terdaftar, tidak menambah apa pun.
pub fn tambah_path(bin: &str, per_mesin: bool) -> Result<(), String> {
    let skop = if per_mesin { "Machine" } else { "User" };
    let skrip = format!(
        "$s='{skop}'; $b='{bin}'; \
         $p=[Environment]::GetEnvironmentVariable('Path',$s); \
         if ($null -eq $p) {{ $p='' }}; \
         $ada = ($p -split ';') | Where-Object {{ $_ -ne '' -and $_.TrimEnd('\\') -ieq $b.TrimEnd('\\') }}; \
         if ($ada) {{ Write-Output 'SUDAH' }} else {{ \
           $baru = if ($p -eq '') {{ $b }} else {{ $p.TrimEnd(';') + ';' + $b }}; \
           [Environment]::SetEnvironmentVariable('Path',$baru,$s); Write-Output 'DITAMBAH' }}",
        skop = skop,
        bin = bin.replace('\'', "''")
    );
    let keluaran = powershell(&skrip)?;
    if keluaran.contains("DITAMBAH") || keluaran.contains("SUDAH") {
        Ok(())
    } else {
        Err(format!("keluaran tak terduga: {}", keluaran.trim()))
    }
}

/// Cabut folder `bin` dari PATH pengguna/mesin.
pub fn hapus_path(bin: &str, per_mesin: bool) -> Result<(), String> {
    let skop = if per_mesin { "Machine" } else { "User" };
    let skrip = format!(
        "$s='{skop}'; $b='{bin}'; \
         $p=[Environment]::GetEnvironmentVariable('Path',$s); \
         if ($null -eq $p) {{ return }}; \
         $bagian = ($p -split ';') | Where-Object {{ $_ -ne '' -and $_.TrimEnd('\\') -ine $b.TrimEnd('\\') }}; \
         [Environment]::SetEnvironmentVariable('Path', ($bagian -join ';'), $s)",
        skop = skop,
        bin = bin.replace('\'', "''")
    );
    powershell(&skrip).map(|_| ())
}

// ---------------------------------------------------------------------------
// Asosiasi berkas .eve
// ---------------------------------------------------------------------------

/// Daftarkan asosiasi `.eve` â†’ "Evernight files" pada hive pengguna.
pub fn asosiasi_eve(rencana: &Rencana) -> Result<(), String> {
    let exe = rencana.exe().to_string_lossy().to_string();
    let ikon = rencana
        .tujuan
        .join("assets")
        .join("icon.ico")
        .to_string_lossy()
        .to_string();

    reg_add(r"HKCU\Software\Classes\.eve", None, PROGID)?;
    reg_add(r"HKCU\Software\Classes\EvernightFile", None, LABEL_TIPE)?;
    reg_add(
        r"HKCU\Software\Classes\EvernightFile\DefaultIcon",
        None,
        &ikon,
    )?;
    reg_add(
        r"HKCU\Software\Classes\EvernightFile\shell\open\command",
        None,
        &format!("\"{}\" run \"%1\"", exe),
    )?;

    // Beri tahu Explorer agar ikon/label langsung diperbarui.
    let _ = powershell(
        "Add-Type -Namespace W -Name S -MemberDefinition \
         '[DllImport(\"shell32.dll\")] public static extern void SHChangeNotify(int e,uint f,IntPtr a,IntPtr b);'; \
         [W.S]::SHChangeNotify(0x08000000,0,[IntPtr]::Zero,[IntPtr]::Zero)",
    );

    Ok(())
}

/// Cabut asosiasi `.eve` yang kita buat.
pub fn hapus_asosiasi_eve() -> Result<(), String> {
    let _ = reg_delete(r"HKCU\Software\Classes\.eve");
    let _ = reg_delete(r"HKCU\Software\Classes\EvernightFile");
    let _ = reg_delete(r"HKCU\Software\Classes\Applications\evernight.exe");
    let _ = powershell(
        "Add-Type -Namespace W -Name S -MemberDefinition \
         '[DllImport(\"shell32.dll\")] public static extern void SHChangeNotify(int e,uint f,IntPtr a,IntPtr b);'; \
         [W.S]::SHChangeNotify(0x08000000,0,[IntPtr]::Zero,[IntPtr]::Zero)",
    );
    Ok(())
}

// ---------------------------------------------------------------------------
// Ekstensi editor
// ---------------------------------------------------------------------------

/// Pasang `.vsix` ke satu editor memakai CLI-nya.
///
/// Beberapa CLI menulis peringatan ke stderr walau berhasil, jadi keberhasilan
/// ditentukan dari keluaran + kode keluar, bukan dari ada/tidaknya stderr.
pub fn pasang_ekstensi(editor: &Editor, vsix: &Path) -> Result<(), String> {
    let keluaran = Command::new(&editor.cli)
        .arg("--install-extension")
        .arg(vsix)
        .arg("--force")
        .output()
        .map_err(|e| format!("Gagal menjalankan {}: {}", editor.nama, e))?;

    let gabung = format!(
        "{}{}",
        String::from_utf8_lossy(&keluaran.stdout),
        String::from_utf8_lossy(&keluaran.stderr)
    );
    if gabung.contains("successfully installed")
        || gabung.contains("already installed")
        || keluaran.status.success()
    {
        Ok(())
    } else {
        Err(format!("{} menolak pemasangan", editor.nama))
    }
}

/// Copot ekstensi dari satu editor.
pub fn copot_ekstensi(editor: &Editor) -> Result<(), String> {
    let keluaran = Command::new(&editor.cli)
        .arg("--uninstall-extension")
        .arg("satriyo.evernight-language")
        .output()
        .map_err(|e| format!("Gagal menjalankan {}: {}", editor.nama, e))?;
    let gabung = format!(
        "{}{}",
        String::from_utf8_lossy(&keluaran.stdout),
        String::from_utf8_lossy(&keluaran.stderr)
    );
    if gabung.contains("not installed") || gabung.contains("not found") {
        return Ok(());
    }
    if keluaran.status.success() {
        Ok(())
    } else {
        Err(format!("{} gagal mencopot", editor.nama))
    }
}

// ---------------------------------------------------------------------------
// Pintasan
// ---------------------------------------------------------------------------

/// Buat pintasan Start Menu ke folder instalasi.
fn buat_pintasan(rencana: &Rencana) -> Result<(), String> {
    let menu = std::env::var("APPDATA").unwrap_or_default();
    let dir = Path::new(&menu)
        .join("Microsoft")
        .join("Windows")
        .join("Start Menu")
        .join("Programs")
        .join(NAMA_PRODUK);
    fs::create_dir_all(&dir).map_err(|e| format!("Gagal membuat folder Start Menu: {}", e))?;

    let lnk = dir.join(format!("{}.lnk", NAMA_PRODUK));
    let target = rencana.exe().to_string_lossy().to_string();
    let ikon = rencana
        .tujuan
        .join("assets")
        .join("icon.ico")
        .to_string_lossy()
        .to_string();

    let skrip = format!(
        "$w=New-Object -ComObject WScript.Shell; \
         $s=$w.CreateShortcut('{lnk}'); \
         $s.TargetPath='{target}'; \
         $s.WorkingDirectory='{kerja}'; \
         $s.IconLocation='{ikon}'; \
         $s.Description='{nama} - Bahasa pemrograman Indonesia'; \
         $s.Save()",
        lnk = lnk.to_string_lossy().replace('\'', "''"),
        target = target.replace('\'', "''"),
        kerja = rencana.tujuan.to_string_lossy().replace('\'', "''"),
        ikon = ikon.replace('\'', "''"),
        nama = NAMA_PRODUK
    );
    powershell(&skrip).map(|_| ())
}

fn hapus_pintasan() {
    let menu = std::env::var("APPDATA").unwrap_or_default();
    let dir = Path::new(&menu)
        .join("Microsoft")
        .join("Windows")
        .join("Start Menu")
        .join("Programs")
        .join(NAMA_PRODUK);
    let _ = fs::remove_dir_all(dir);
}

// ---------------------------------------------------------------------------
// Registrasi Apps & Features
// ---------------------------------------------------------------------------

/// Daftarkan produk di *Apps & Features* (hive pengguna).
///
/// Catatan: entri per-user (HKCU) tidak ditampilkan di semua versi Windows.
/// Untuk pemasangan per-machine, kunci HKLM dipakai sebagai gantinya.
fn daftar_uninstall(rencana: &Rencana) -> Result<(), String> {
    let kunci = kunci_uninstall(rencana.per_mesin);
    let ukuran = hitung_ukuran(&rencana.tujuan);
    // Pencopotan dijalankan oleh `uninstall.exe` (salinan installer ini sendiri),
    // BUKAN `evernight.exe` — compiler tidak mengenal flag `--uninstall`.
    let copot_exe = rencana.tujuan.join("uninstall.exe");
    let exe = if copot_exe.exists() {
        copot_exe.to_string_lossy().to_string()
    } else {
        // Cadangan: jalankan installer yang sedang berjalan.
        std::env::current_exe()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| rencana.exe().to_string_lossy().to_string())
    };
    let ikon = rencana
        .tujuan
        .join("assets")
        .join("icon.ico")
        .to_string_lossy()
        .to_string();

    reg_add(&kunci, Some("DisplayName"), NAMA_PRODUK)?;
    reg_add(&kunci, Some("DisplayVersion"), VERSI)?;
    reg_add(&kunci, Some("Publisher"), "Satriyo")?;
    reg_add(&kunci, Some("DisplayIcon"), &ikon)?;
    reg_add(
        &kunci,
        Some("UninstallString"),
        &format!("\"{}\" --uninstall", exe),
    )?;
    reg_add(
        &kunci,
        Some("QuietUninstallString"),
        &format!("\"{}\" --uninstall --diam", exe),
    )?;
    reg_add(&kunci, Some("InstallLocation"), &rencana.tujuan.to_string_lossy())?;
    reg_add(&kunci, Some("NoModify"), "1")?;
    reg_add(&kunci, Some("NoRepair"), "1")?;
    reg_add(
        &kunci,
        Some("EstimatedSize"),
        &ukuran.to_string(),
    )?;
    Ok(())
}

fn kunci_uninstall(per_mesin: bool) -> String {
    if per_mesin {
        r"HKLM\Software\Microsoft\Windows\CurrentVersion\Uninstall\EvernightLanguage".to_string()
    } else {
        KUNCI_UNINSTALL.to_string()
    }
}

/// Ukuran folder dalam KB (dipakai `EstimatedSize`).
fn hitung_ukuran(dir: &Path) -> u64 {
    let mut total = 0u64;
    if let Ok(isi) = fs::read_dir(dir) {
        for e in isi.flatten() {
            if let Ok(m) = e.metadata() {
                if m.is_dir() {
                    total += hitung_ukuran(&e.path());
                } else {
                    total += m.len();
                }
            }
        }
    }
    total / 1024
}

// ---------------------------------------------------------------------------
// Pencopotan
// ---------------------------------------------------------------------------

/// Jalankan pencopotan penuh.
pub fn copot(lapor: &mut dyn FnMut(Pesan)) -> Result<(), String> {
    let tujuan = baca_install_location().unwrap_or_else(|| {
        let lokal = std::env::var("LOCALAPPDATA").unwrap_or_default();
        Path::new(&lokal).join("Programs").join("Evernight")
    });

    lapor(Pesan::Langkah {
        persen: 0.2,
        teks: "Mencabut asosiasi berkas...".to_string(),
    });
    hapus_asosiasi_eve()?;

    lapor(Pesan::Langkah {
        persen: 0.4,
        teks: "Mencabut PATH...".to_string(),
    });
    let bin = tujuan.join("bin").to_string_lossy().to_string();
    let _ = hapus_path(&bin, false);
    let _ = hapus_path(&bin, true);

    lapor(Pesan::Langkah {
        persen: 0.6,
        teks: "Mencopot ekstensi editor...".to_string(),
    });
    for ed in deteksi_editor() {
        let _ = copot_ekstensi(&ed);
    }
    hapus_pintasan();

    lapor(Pesan::Langkah {
        persen: 0.8,
        teks: "Menghapus registri...".to_string(),
    });
    let _ = reg_delete(&kunci_uninstall(false));
    let _ = reg_delete(&kunci_uninstall(true));

    lapor(Pesan::Langkah {
        persen: 0.9,
        teks: "Menghapus berkas program...".to_string(),
    });
    if tujuan.exists() {
        // Hapus semua ISI folder selagi kita masih berjalan. Folder itu sendiri
        // belum bisa dihapus bila `uninstall.exe` berada di dalamnya, karena
        // Windows mengunci executable yang sedang berjalan.
        let diri = std::env::current_exe().ok();
        hapus_isi(&tujuan, diri.as_deref());

        // Bila folder sudah kosong (kita dijalankan dari tempat lain), selesai.
        if fs::remove_dir(&tujuan).is_err() {
            // Sisakan folder + diri sendiri, lalu jadwalkan pembersihan setelah
            // proses ini keluar (cmd menunggu PID kita lalu `rmdir /S /Q`).
            if !jadwalkan_hapus_diri(&tujuan) {
                return Err("Gagal menjadwalkan penghapusan folder".to_string());
            }
        }
    }

    Ok(())
}

/// Hapus seluruh isi `dir`, kecuali berkas `kecuali` (diri sendiri) bila ada.
fn hapus_isi(dir: &Path, kecuali: Option<&Path>) {
    let kecuali = kecuali.and_then(|p| p.canonicalize().ok());
    let isi = match fs::read_dir(dir) {
        Ok(i) => i,
        Err(_) => return,
    };
    for entri in isi.flatten() {
        let path = entri.path();
        if let Some(k) = &kecuali {
            if path.canonicalize().ok().as_ref() == Some(k) {
                continue; // jangan hapus diri sendiri
            }
        }
        if path.is_dir() {
            let _ = fs::remove_dir_all(&path);
        } else {
            let _ = fs::remove_file(&path);
        }
    }
}

/// Jadwalkan penghapusan folder sendiri setelah proses ini keluar.
///
/// Trik standar Windows: jalankan proses `cmd` yang langsung `rmdir` sambil
/// menunggu proses kita benar-benar berhenti (`taskkill` pada PID sendiri
/// akan gagal dengan sendirinya, jadi cukup menunggu lewat perulangan).
///
/// Dua tahap: (1) `move` folder ke nama sementara agar kunci berkas lepas,
/// lalu (2) hapus. Bila tahap 1 gagal (folder masih terkunci), dicoba hanya
/// dengan jeda lebih lama.
fn jadwalkan_hapus_diri(dir: &Path) -> bool {
    let dir_s = dir.to_string_lossy().replace('"', "");
    let parent = dir
        .parent()
        .map(|p| p.to_string_lossy().replace('"', ""))
        .unwrap_or_default();
    let nama = dir
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "Evernight".to_string());
    let sementara = format!("{}\\~{}.hapus", parent, nama);

    // Perintah dijalankan oleh cmd tersembunyi: tunggu 3 detik (proses kita
    // selesai & handle dilepas), lalu pindahkan & hapus.
    let perintah = format!(
        "timeout /t 3 /nobreak >nul & \
         if exist \"{dir}\" rmdir /S /Q \"{dir}\" & \
         if exist \"{dir}\" ( move /Y \"{dir}\" \"{tmp}\" >nul 2>&1 & rmdir /S /Q \"{tmp}\" )",
        dir = dir_s,
        tmp = sementara
    );

    let skrip = format!(
        "Start-Process -WindowStyle Hidden -FilePath 'cmd.exe' -ArgumentList '/C','{}'",
        perintah.replace('\'', "''")
    );
    powershell(&skrip).is_ok()
}

/// Lokasi pemasangan yang tercatat (dari registri, atau default per-user).
pub fn baca_tujuan_terpasang() -> Option<PathBuf> {
    if let Some(p) = baca_install_location() {
        return Some(p);
    }
    Some(Rencana::tujuan_default(false))
}

/// Baca lokasi pemasangan dari registri uninstall.
fn baca_install_location() -> Option<PathBuf> {
    let keluaran = reg_query(KUNCI_UNINSTALL, "InstallLocation").ok()?;
    let baris = keluaran
        .lines()
        .find(|l| l.contains("InstallLocation"))?;
    let nilai = baris.split("REG_SZ").nth(1)?.trim().to_string();
    if nilai.is_empty() {
        None
    } else {
        Some(PathBuf::from(nilai))
    }
}

/// Jadwalkan penghapusan folder saat reboot (untuk berkas terkunci).
#[allow(dead_code)]
fn jadwalkan_hapus(dir: &Path) -> Result<(), String> {
    let dir = dir.to_string_lossy().replace('\'', "''");
    powershell(&format!(
        "New-ItemProperty -Path 'HKLM:\\SYSTEM\\CurrentControlSet\\Control\\Session Manager' \
         -Name 'PendingFileRenameOperations' -PropertyType MultiString -Force \
         -Value @('\\??\\{dir}') -ErrorAction SilentlyContinue | Out-Null",
        dir = dir
    ))
    .map(|_| ())
}

// ---------------------------------------------------------------------------
// Utilitas Windows
// ---------------------------------------------------------------------------

/// Jalankan PowerShell tanpa jendela, kembalikan stdout+stderr.
fn powershell(skrip: &str) -> Result<String, String> {
    let keluaran = Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-ExecutionPolicy", "Bypass", "-Command", skrip])
        .output()
        .map_err(|e| format!("PowerShell tidak dapat dijalankan: {}", e))?;
    let mut gabung = String::from_utf8_lossy(&keluaran.stdout).to_string();
    gabung.push_str(&String::from_utf8_lossy(&keluaran.stderr));
    Ok(gabung)
}

/// `reg add` -- tulis nilai registri.
fn reg_add(kunci: &str, nama: Option<&str>, nilai: &str) -> Result<(), String> {
    let mut cmd = Command::new("reg");
    cmd.args(["add", kunci]);
    match nama {
        Some(n) => {
            cmd.args(["/v", n]);
        }
        None => {
            cmd.args(["/ve"]);
        }
    }
    cmd.args(["/t", "REG_SZ", "/d", nilai, "/f"]);

    let keluaran = cmd
        .output()
        .map_err(|e| format!("reg add gagal dijalankan: {}", e))?;
    if keluaran.status.success() {
        Ok(())
    } else {
        Err(format!(
            "reg add {} ditolak: {}",
            kunci,
            String::from_utf8_lossy(&keluaran.stderr).trim()
        ))
    }
}

/// `reg query` -- baca nilai registri.
fn reg_query(kunci: &str, nama: &str) -> Result<String, String> {
    let keluaran = Command::new("reg")
        .args(["query", kunci, "/v", nama])
        .output()
        .map_err(|e| format!("reg query gagal dijalankan: {}", e))?;
    Ok(String::from_utf8_lossy(&keluaran.stdout).to_string())
}

/// `reg delete` -- hapus kunci/nilai.
fn reg_delete(kunci: &str) -> Result<(), String> {
    let keluaran = Command::new("reg")
        .args(["delete", kunci, "/f"])
        .output()
        .map_err(|e| format!("reg delete gagal dijalankan: {}", e))?;
    let _ = keluaran;
    Ok(())
}

/// Apakah proses berjalan dengan hak administrator.
pub fn adalah_admin() -> bool {
    Command::new("net")
        .arg("session")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Jalankan ulang installer dengan elevasi UAC.
pub fn jalankan_ulang_elevasi(argumen: &[String]) -> Result<(), String> {
    let exe = std::env::current_exe().map_err(|e| format!("Tidak tahu lokasi sendiri: {}", e))?;
    let arg = argumen.join(" ");
    let skrip = format!(
        "Start-Process -FilePath '{}' -ArgumentList '{}' -Verb RunAs",
        exe.to_string_lossy().replace('\'', "''"),
        arg.replace('\'', "''")
    );
    powershell(&skrip).map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tujuan_default_per_user_berada_di_localappdata() {
        let t = Rencana::tujuan_default(false);
        let s = t.to_string_lossy().to_lowercase();
        assert!(s.contains("programs"), "{}", s);
        assert!(s.contains("evernight"), "{}", s);
    }

    #[test]
    fn tujuan_default_per_mesin_di_program_files() {
        let t = Rencana::tujuan_default(true);
        assert_eq!(t, PathBuf::from(r"C:\Program Files\Evernight"));
    }

    #[test]
    fn rencana_default_semua_tugas_aktif() {
        let r = Rencana::default();
        assert!(r.tambah_path);
        assert!(r.asosiasi_eve);
        assert!(r.pasang_ekstensi);
        assert!(r.buat_pintasan);
        assert!(!r.per_mesin);
    }

    #[test]
    fn bin_dan_exe_konsisten() {
        let mut r = Rencana::default();
        r.tujuan = PathBuf::from(r"C:\X\Evernight");
        assert_eq!(r.bin(), PathBuf::from(r"C:\X\Evernight\bin"));
        assert_eq!(r.exe(), PathBuf::from(r"C:\X\Evernight\bin\evernight.exe"));
    }

    #[test]
    fn kunci_uninstall_berbeda_per_mode() {
        assert!(kunci_uninstall(false).starts_with("HKCU"));
        assert!(kunci_uninstall(true).starts_with("HKLM"));
    }
}
