//! Server HTTP mandiri (zero-dependency) untuk Web Notebook March.
//!
//! Melayani berkas antarmuka SPA Colab Dark Mode yang di-embed langsung ke biner,
//! serta endpoint REST API untuk eksekusi sel di kernel VM, penjelajah berkas lokal,
//! dan autosave berkas `.emlbn`.

use super::kernel::MarchKernel;
use super::notebook::MarchNotebook;
use crate::json::{self, Json};
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

const INDEX_HTML: &str = include_str!("aset/index.html");

pub struct MarchServerState {
    pub notebook: MarchNotebook,
    pub notebook_path: PathBuf,
    pub working_dir: PathBuf,
    pub kernel: MarchKernel,
    pub port: u16,
}

pub fn mulai_server(notebook_path: PathBuf, working_dir: PathBuf, port_awal: u16, buka_browser: bool) -> Result<(), String> {
    // 1. Inisialisasi notebook (buka yang ada atau buat baru)
    let notebook = if notebook_path.exists() {
        MarchNotebook::muat_dari_berkas(&notebook_path)?
    } else {
        let nama = notebook_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Notebook_Baru.emlbn");
        let nb = MarchNotebook::baru(nama, working_dir.to_str().unwrap_or("."));
        nb.simpan_ke_berkas(&notebook_path)?;
        nb
    };

    // 2. Cari port yang tersedia
    let mut port = port_awal;
    let listener = loop {
        match TcpListener::bind(("127.0.0.1", port)) {
            Ok(l) => break l,
            Err(_) => {
                port += 1;
                if port > port_awal + 20 {
                    return Err(format!("Gagal mengikat port antara {} dan {}", port_awal, port));
                }
            }
        }
    };

    let url = format!("http://127.0.0.1:{}", port);
    let state = Arc::new(Mutex::new(MarchServerState {
        kernel: MarchKernel::new(working_dir.clone()),
        notebook,
        notebook_path: notebook_path.clone(),
        working_dir: working_dir.clone(),
        port,
    }));

    // 3. Cetak banner Colab style ke terminal
    println!("======================================================================");
    println!("     MARCH NOTEBOOK SERVER (EvernightLanguage ML Workspace 9H)        ");
    println!("======================================================================");
    println!("  [OK] Server Web berjalan di  : {}", url);
    println!("  [OK] Direktori kerja laptop  : {}", working_dir.display());
    println!("  [OK] Berkas notebook aktif   : {}", notebook_path.display());
    println!("  [OK] Penyimpanan lokal 100%  : Aktif (tanpa ketergantungan cloud)");
    println!("  Tekan Ctrl + C pada terminal ini untuk menghentikan server.");
    println!("======================================================================\n");

    // 4. Buka browser default secara otomatis
    if buka_browser {
        buka_peramban_web(&url);
    }

    // 5. Loop penerimaan koneksi HTTP
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let state_clone = Arc::clone(&state);
                tangani_koneksi(stream, state_clone);
            }
            Err(e) => {
                eprintln!("[March Server] Kesalahan koneksi TCP: {}", e);
            }
        }
    }

    Ok(())
}

fn tangani_koneksi(mut stream: TcpStream, state_arc: Arc<Mutex<MarchServerState>>) {
    let _ = stream.set_read_timeout(Some(std::time::Duration::from_millis(2500)));
    let _ = stream.set_write_timeout(Some(std::time::Duration::from_millis(2500)));
    let mut buffer = [0u8; 8192];
    let bytes_read = match stream.read(&mut buffer) {
        Ok(n) if n > 0 => n,
        _ => return,
    };

    let request_str = String::from_utf8_lossy(&buffer[..bytes_read]);
    let mut lines = request_str.lines();
    let request_line = match lines.next() {
        Some(l) => l,
        None => return,
    };

    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() < 2 {
        return;
    }

    let method = parts[0];
    let path = parts[1];

    // Ekstraksi body jika ada (untuk POST)
    let body = if method == "POST" {
        if let Some(pos) = request_str.find("\r\n\r\n") {
            &request_str[pos + 4..]
        } else {
            ""
        }
    } else {
        ""
    };

    // Routing
    if method == "OPTIONS" {
        kirim_respon(&mut stream, 204, "No Content", "text/plain", "");
        return;
    }

    match (method, path) {
        ("GET", "/") | ("GET", "/index.html") => {
            kirim_respon(&mut stream, 200, "OK", "text/html; charset=utf-8", INDEX_HTML);
        }
        ("GET", "/api/status") => {
            let state = state_arc.lock().unwrap();
            let json = Json::Objek(vec![
                ("status".to_string(), Json::Teks("ok".to_string())),
                ("port".to_string(), Json::Angka(state.port as f64)),
                ("notebook".to_string(), Json::Teks(state.notebook.nama_notebook.clone())),
                ("folder".to_string(), Json::Teks(state.working_dir.to_string_lossy().to_string())),
            ]);
            kirim_respon(&mut stream, 200, "OK", "application/json; charset=utf-8", &json.tulis_kompak());
        }
        ("GET", "/api/berkas") => {
            let state = state_arc.lock().unwrap();
            let files_json = pindai_berkas_lokal(&state.working_dir);
            kirim_respon(&mut stream, 200, "OK", "application/json; charset=utf-8", &files_json.tulis_kompak());
        }
        ("GET", "/api/notebook") => {
            let state = state_arc.lock().unwrap();
            let nb_json = state.notebook.ke_json();
            kirim_respon(&mut stream, 200, "OK", "application/json; charset=utf-8", &nb_json.tulis_kompak());
        }
        ("POST", "/api/folder/ganti") => {
            let parsed_req = json::parse(body);
            let folder_baru = match parsed_req {
                Ok(Json::Objek(p)) => {
                    let mut f = String::new();
                    for (k, v) in p {
                        if k == "folder" {
                            f = v.sebagai_teks().unwrap_or("").to_string();
                        }
                    }
                    f
                }
                _ => String::new(),
            };

            if folder_baru.trim().is_empty() {
                kirim_respon(&mut stream, 400, "Bad Request", "application/json; charset=utf-8", "{\"sukses\":false,\"pesan\":\"Path folder tidak boleh kosong\"}");
                return;
            }

            let path = std::path::PathBuf::from(folder_baru.trim());
            if !path.exists() {
                if let Err(e) = fs::create_dir_all(&path) {
                    let err_json = format!("{{\"sukses\":false,\"pesan\":\"Folder tidak ada dan gagal dibuat: {}\"}}", e);
                    kirim_respon(&mut stream, 400, "Bad Request", "application/json; charset=utf-8", &err_json);
                    return;
                }
            } else if !path.is_dir() {
                kirim_respon(&mut stream, 400, "Bad Request", "application/json; charset=utf-8", "{\"sukses\":false,\"pesan\":\"Path yang diberikan bukan direktori/folder\"}");
                return;
            }

            let mut state = state_arc.lock().unwrap();
            state.working_dir = path;
            let files_json = pindai_berkas_lokal(&state.working_dir);
            let res = Json::Objek(vec![
                ("sukses".to_string(), Json::Bolean(true)),
                ("folder".to_string(), Json::Teks(state.working_dir.to_string_lossy().to_string())),
                ("berkas_data".to_string(), files_json),
            ]);
            kirim_respon(&mut stream, 200, "OK", "application/json; charset=utf-8", &res.tulis_kompak());
        }
        ("POST", "/api/folder/telusuri") => {
            #[cfg(windows)]
            let folder_opt = {
                let script = "[System.Reflection.Assembly]::LoadWithPartialName('System.Windows.Forms') | Out-Null; $d = New-Object System.Windows.Forms.FolderBrowserDialog; $d.Description = 'Pilih Folder Penyimpanan Notebook March (.emlbn)'; $d.ShowNewFolderButton = $true; if ($d.ShowDialog() -eq [System.Windows.Forms.DialogResult]::OK) { Write-Output $d.SelectedPath }";
                std::process::Command::new("powershell")
                    .args(["-NoProfile", "-Command", script])
                    .output()
                    .ok()
                    .and_then(|output| {
                        if output.status.success() {
                            let s = String::from_utf8_lossy(&output.stdout).trim().to_string();
                            if !s.is_empty() {
                                Some(s)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
            };

            #[cfg(not(windows))]
            let folder_opt: Option<String> = None;

            if let Some(folder_terpilih) = folder_opt {
                let path = std::path::PathBuf::from(&folder_terpilih);
                let mut state = state_arc.lock().unwrap();
                state.working_dir = path;
                let files_json = pindai_berkas_lokal(&state.working_dir);
                let res = Json::Objek(vec![
                    ("sukses".to_string(), Json::Bolean(true)),
                    ("folder".to_string(), Json::Teks(state.working_dir.to_string_lossy().to_string())),
                    ("berkas_data".to_string(), files_json),
                ]);
                kirim_respon(&mut stream, 200, "OK", "application/json; charset=utf-8", &res.tulis_kompak());
            } else {
                kirim_respon(&mut stream, 200, "OK", "application/json; charset=utf-8", "{\"sukses\":false,\"dibatalkan\":true}");
            }
        }
        ("POST", "/api/notebook/buka") => {
            let parsed_req = json::parse(body);
            let target_nama = match parsed_req {
                Ok(Json::Objek(p)) => {
                    let mut n = String::new();
                    for (k, v) in p {
                        if k == "nama" {
                            n = v.sebagai_teks().unwrap_or("").to_string();
                        }
                    }
                    n
                }
                _ => String::new(),
            };

            if target_nama.is_empty() {
                kirim_respon(&mut stream, 400, "Bad Request", "application/json; charset=utf-8", "{\"sukses\":false,\"pesan\":\"Nama berkas tidak boleh kosong\"}");
                return;
            }

            let mut state = state_arc.lock().unwrap();
            let file_path = state.working_dir.join(&target_nama);
            match MarchNotebook::muat_dari_berkas(&file_path) {
                Ok(nb) => {
                    state.kernel.reset();
                    state.notebook = nb;
                    state.notebook_path = file_path;
                    let nb_json = state.notebook.ke_json();
                    kirim_respon(&mut stream, 200, "OK", "application/json; charset=utf-8", &nb_json.tulis_kompak());
                }
                Err(e) => {
                    let err_json = format!("{{\"sukses\":false,\"pesan\":\"{}\"}}", e);
                    kirim_respon(&mut stream, 500, "Internal Server Error", "application/json; charset=utf-8", &err_json);
                }
            }
        }
        ("POST", "/api/notebook/baru") => {
            let parsed_req = json::parse(body);
            let nama = match parsed_req {
                Ok(Json::Objek(p)) => {
                    let mut n = String::new();
                    for (k, v) in p {
                        if k == "nama" {
                            n = v.sebagai_teks().unwrap_or("").to_string();
                        }
                    }
                    n
                }
                _ => String::new(),
            };

            let final_name = if nama.trim().is_empty() {
                format!("Notebook_{}.emlbn", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0))
            } else if !nama.ends_with(".emlbn") {
                format!("{}.emlbn", nama.trim())
            } else {
                nama.trim().to_string()
            };

            let mut state = state_arc.lock().unwrap();
            let file_path = state.working_dir.join(&final_name);
            let folder_str = state.working_dir.to_string_lossy().to_string();
            let nb = MarchNotebook::baru(&final_name, &folder_str);
            let _ = nb.simpan_ke_berkas(&file_path);

            state.kernel.reset();
            state.notebook = nb;
            state.notebook_path = file_path;
            let nb_json = state.notebook.ke_json();
            kirim_respon(&mut stream, 200, "OK", "application/json; charset=utf-8", &nb_json.tulis_kompak());
        }
        ("POST", "/api/sel/jalankan") => {
            let parsed_req = json::parse(body);
            let (id, kode) = match parsed_req {
                Ok(Json::Objek(p)) => {
                    let mut id_val = String::new();
                    let mut kode_val = String::new();
                    for (k, v) in p {
                        if k == "id" {
                            id_val = v.sebagai_teks().unwrap_or("").to_string();
                        } else if k == "kode" {
                            kode_val = v.sebagai_teks().unwrap_or("").to_string();
                        }
                    }
                    (id_val, kode_val)
                }
                _ => {
                    kirim_respon(&mut stream, 400, "Bad Request", "application/json; charset=utf-8", "{\"sukses\":false,\"pesan\":\"Payload JSON tidak valid\"}");
                    return;
                }
            };

            let mut state = state_arc.lock().unwrap();
            let (urutan, keluaran) = state.kernel.jalankan_sel(&kode);

            // Perbarui sel di notebook dan simpan ke disk otomatis
            for cell in &mut state.notebook.sel {
                if cell.id == id {
                    cell.urutan_eksekusi = Some(urutan);
                    cell.sumber = kode.clone();
                    cell.keluaran = Some(keluaran.clone());
                    break;
                }
            }
            let _ = state.notebook.simpan_ke_berkas(&state.notebook_path);

            let res_json = Json::Objek(vec![
                ("sukses".to_string(), Json::Bolean(true)),
                ("urutan".to_string(), Json::Angka(urutan as f64)),
                ("keluaran".to_string(), keluaran.ke_json()),
            ]);
            kirim_respon(&mut stream, 200, "OK", "application/json; charset=utf-8", &res_json.tulis_kompak());
        }
        ("POST", "/api/simpan") => {
            let parsed_req = json::parse(body);
            match parsed_req {
                Ok(json_nb) => match MarchNotebook::dari_json(&json_nb) {
                    Ok(nb) => {
                        let mut state = state_arc.lock().unwrap();
                        state.notebook = nb;
                        let _ = state.notebook.simpan_ke_berkas(&state.notebook_path);
                        kirim_respon(&mut stream, 200, "OK", "application/json; charset=utf-8", "{\"sukses\":true}");
                    }
                    Err(e) => {
                        let err_json = format!("{{\"sukses\":false,\"pesan\":\"{}\"}}", e);
                        kirim_respon(&mut stream, 400, "Bad Request", "application/json; charset=utf-8", &err_json);
                    }
                },
                Err(e) => {
                    let err_json = format!("{{\"sukses\":false,\"pesan\":\"{}\"}}", e);
                    kirim_respon(&mut stream, 400, "Bad Request", "application/json; charset=utf-8", &err_json);
                }
            }
        }
        ("POST", "/api/kernel/reset") => {
            let mut state = state_arc.lock().unwrap();
            state.kernel.reset();
            kirim_respon(&mut stream, 200, "OK", "application/json; charset=utf-8", "{\"sukses\":true}");
        }
        ("GET", "/api/ekspor") => {
            let state = state_arc.lock().unwrap();
            let skrip_eve = state.notebook.ekspor_ke_eve();
            kirim_respon(&mut stream, 200, "OK", "text/plain; charset=utf-8", &skrip_eve);
        }
        _ => {
            kirim_respon(&mut stream, 404, "Not Found", "text/plain", "Halaman tidak ditemukan (404)");
        }
    }
}

fn kirim_respon(stream: &mut TcpStream, status_code: u16, status_text: &str, content_type: &str, body: &str) {
    let header = format!(
        "HTTP/1.1 {} {}\r\n\
Content-Type: {}\r\n\
Content-Length: {}\r\n\
Access-Control-Allow-Origin: *\r\n\
Access-Control-Allow-Methods: GET, POST, OPTIONS\r\n\
Access-Control-Allow-Headers: Content-Type\r\n\
Connection: close\r\n\
\r\n",
        status_code,
        status_text,
        content_type,
        body.len()
    );
    let _ = stream.write_all(header.as_bytes());
    let _ = stream.write_all(body.as_bytes());
    let _ = stream.flush();
}

fn pindai_berkas_lokal(dir: &std::path::Path) -> Json {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                // HANYA menyaring berkas notebook March (.emlbn)
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if ext.eq_ignore_ascii_case("emlbn") {
                        if let Some(nama) = path.file_name().and_then(|n| n.to_str()) {
                            let metadata = entry.metadata().ok();
                            let ukuran = metadata.map(|m| m.len()).unwrap_or(0);
                            files.push(Json::Objek(vec![
                                ("nama".to_string(), Json::Teks(nama.to_string())),
                                ("ukuran".to_string(), Json::Angka(ukuran as f64)),
                            ]));
                        }
                    }
                }
            }
        }
    }

    files.sort_by(|a, b| {
        let na = a.ambil_teks("nama").unwrap_or("");
        let nb = b.ambil_teks("nama").unwrap_or("");
        na.cmp(nb)
    });

    Json::Objek(vec![
        ("folder".to_string(), Json::Teks(dir.to_string_lossy().to_string())),
        ("berkas".to_string(), Json::Larik(files)),
    ])
}

fn buka_peramban_web(url: &str) {
    #[cfg(windows)]
    {
        let _ = std::process::Command::new("cmd")
            .args(["/C", "start", url])
            .spawn();
    }
    #[cfg(not(windows))]
    {
        let _ = std::process::Command::new("xdg-open")
            .arg(url)
            .spawn();
    }
}
