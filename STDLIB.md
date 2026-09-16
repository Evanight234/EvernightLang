# STDLIB.md — Standard Library EvernightLanguage

> Sumber kebenaran untuk rancangan dan spesifikasi modul pustaka standar (Standard Library) EvernightLanguage.
> Modul standar diimpor secara eksplisit menggunakan kata kunci `impor <nama_modul>`.

---

## 1. Aturan Penggunaan & Desain

| Aspek | Keputusan |
|-------|-----------|
| **Sistem Impor** | Eksplisit menggunakan `impor <nama_modul>` (contoh: `impor matematika`); untuk berkas `.eve` lain: `impor "path/modul" [sebagai alias]` |
| **Fungsi Bawaan (Built-in)** | `cetak(...)`, `baca(...)`, `baca_angka(...)`, `bersihkan()` dan fungsi konversi tipe dasar (`angka()`, `teks()`, `bolean()`, `daftar()`, `kamus()`) tersedia global tanpa impor |
| **Konvensi Penamaan** | Bahasa Indonesia (huruf kecil, pemisah garis bawah `_`) |
| **Kontrol Berkas** | Kontrol manual terkelola (`buka`, `tutup`) dan fungsi ringkas (`baca_file`, `tulis_file`) |

> **Impor lintas berkas (2026-09-10):** `impor "nama_modul" [sebagai alias]` memuat berkas `.eve` lain relatif terhadap direktori berkas utama (ekstensi `.eve` ditambahkan otomatis). Modul dijalankan **satu kali** (lazy-load + cache), dieksekusi di scope-nya sendiri, dan seluruh `fungsi`/`variabel` top-level-nya diekspos sebagai **kamus** (`Value::Kamus`). Akses ekspor lewat namespace: `impor "helper" sebagai h` → panggil `h.tambah(2, 3)`; tanpa alias nama global = nama dasar berkas (`impor "sub/helper"` → `helper`). Siklus impor (A→B→A) dan berkas hilang → `BAHAYA [FILE]`. Modul standar (`konsol`, `string`, dst.) tetap no-op. Bentuk `impor X dari ...` **belum didukung** (ditunda ke v2).


---

## 2. Daftar Modul Standar Lengkap

### A. Modul Konsol (`impor konsol`)
Menangani interaksi masukan dari pengguna melalui antarmuka terminal/konsol.
> **Catatan (2026-09-10):** fungsi modul konsol diekspos sebagai **global builtin** — `baca`, `baca_angka`, `bersihkan` bisa dipakai **tanpa** `impor`. `impor konsol` diterima compiler sebagai no-op (opsional, tanpa peringatan) agar siapa pun yang sudah menulisnya tetap berjalan.

| Fungsi | Parameter | Tipe Kembalian | Deskripsi | Contoh Penggunaan |
|--------|-----------|----------------|-----------|-------------------|
| `baca(pesan?)` | `teks?` | `teks` | Menampilkan prompt lalu membaca input baris dari pengguna | `variabel nama = baca("Nama: ")` |
| `baca_angka(pesan?)` | `teks?` | `angka` | Membaca input dan langsung mengonversinya menjadi angka | `variabel umur = baca_angka("Umur: ")` |
| `bersihkan()` | *tanpa parameter* | `kosong` | Membersihkan tampilan layar terminal | `bersihkan()` |

---

### B. Modul String (`impor string`)
Fungsi manipulasi dan pemeriksaan teks.

| Fungsi | Parameter | Tipe Kembalian | Deskripsi | Contoh Penggunaan |
|--------|-----------|----------------|-----------|-------------------|
| `besar(s)` | `teks` | `teks` | Mengubah teks menjadi huruf kapital | `besar("halo")` → `"HALO"` |
| `kecil(s)` | `teks` | `teks` | Mengubah teks menjadi huruf kecil | `kecil("DUNIA")` → `"dunia"` |
| `bersih(s)` | `teks` | `teks` | Menghapus spasi kosong di awal dan akhir teks | `bersih("  teks  ")` → `"teks"` |
| `potong(s, awal, akhir?)` | `teks`, `angka`, `angka?` | `teks` | Mengambil cuplikan teks berdasarkan rentang indeks | `potong("Nusantara", 0, 4)` → `"Nusa"` |
| `pecah(s, pemisah)` | `teks`, `teks` | `daftar` | Memecah teks menjadi daftar teks berdasarkan pemisah | `pecah("a,b,c", ",")` → `["a", "b", "c"]` |
| `gabung(lst, pemisah)` | `daftar`, `teks` | `teks` | Menggabungkan daftar teks menjadi satu teks | `gabung(["a", "b"], "-")` → `"a-b"` |
| `ganti(s, lama, baru)` | `teks`, `teks`, `teks` | `teks` | Mengganti kemunculan teks lama dengan teks baru | `ganti("Budi", "B", "R")` → `"Rudi"` |
| `mengandung(s, target)` | `teks`, `teks` | `bolean` | Memeriksa apakah teks mengandung substring | `mengandung("Evernight", "night")` → `benar` |
| `mulai_dengan(s, awalan)`| `teks`, `teks` | `bolean` | Memeriksa apakah teks diawali dengan awalan tertentu | `mulai_dengan("index.eve", "index")` → `benar` |
| `akhir_dengan(s, akhiran)`| `teks`, `teks` | `bolean` | Memeriksa apakah teks diakhiri dengan akhiran tertentu | `akhir_dengan("main.eve", ".eve")` → `benar` |
| `ulang_teks(s, n)` | `teks`, `angka` | `teks` | Mengulang string sebanyak `n` kali | `ulang_teks("ha", 3)` → `"hahaha"` |
| `format(pola, args...)` | `teks`, `variadic` | `teks` | Memformat string dengan variabel interpolasi | `format("Halo {0}, umur {1}", "Budi", 20)` |

---

### C. Modul Daftar (`impor daftar`)
Operasi dan manipulasi koleksi data larik/daftar.

> **Implementasi (2026-09-10):** fungsi global tanpa impor (`impor daftar` diizinkan, no-op). `tambah/sisip/hapus`(fungsi) memutasi daftar asli in-place dan mengembalikan `kosong`; `urutkan/balik/unik/gabung_larik/iris` mengembalikan daftar baru (tidak mengubah asal). `urutkan` hanya untuk daftar **homogen** angka atau teks — campuran → `BAHAYA [TYPE]`. `jumlah`/`rata_rata` hanya elemen angka → `[TYPE]`; `rata_rata([])` → `[JUMLAH]`. `sisip` indeks out-of-range → `[INDEX]`; `iris` clamp otomatis seperti `potong`. `cari` → indeks pertama atau `-1`; `ada` → `benar/salah`. Higher-order `peta/saring/lipat/setiap` butuh callback 1 argumen (`lipat` callback 2 argumen: akumulator + elemen — ORDER itu, bukan elemen dulu), arity salah → `[FUNGSI]`; `lipat([], awal)` → `awal`; `setiap` → `kosong`; error di dalam callback merambat ke `coba/tangkap` pemanggil. Method syntax `a.tambah(x)` **belum didukung** — pakai fungsi global (`ARRAY.md`). Statement `hapus a[i]` menghapus elemen (wrap negatif, out-of-range `[INDEX]`); lihat keyword `hapus`.

| Fungsi | Parameter | Tipe Kembalian | Deskripsi | Contoh Penggunaan |
|--------|-----------|----------------|-----------|-------------------|
| `tambah(lst, item)` | `daftar`, `apapun` | `kosong` | Menambahkan elemen ke akhir daftar | `tambah(a, 10)` |
| `sisip(lst, idx, item)` | `daftar`, `angka`, `apapun`| `kosong` | Menyisipkan elemen pada posisi indeks tertentu | `sisip(a, 0, "awal")` |
| `hapus(lst, item)` | `daftar`, `apapun` | `kosong` | Menghapus elemen pertama yang cocok | `hapus(a, "awal")` |
| `urutkan(lst)` | `daftar` | `daftar` | Mengurutkan elemen daftar | `urutkan([3, 1, 2])` → `[1, 2, 3]` |
| `balik(lst)` | `daftar` | `daftar` | Membalikkan urutan elemen | `balik([1, 2, 3])` → `[3, 2, 1]` |
| `unik(lst)` | `daftar` | `daftar` | Mengambil elemen-elemen unik (tanpa duplikasi) | `unik([1, 2, 2, 3])` → `[1, 2, 3]` |
| `jumlah(lst)` | `daftar` | `angka` | Menjumlahkan seluruh elemen angka | `jumlah([10, 20, 30])` → `60` |
| `rata_rata(lst)` | `daftar` | `angka` | Menghitung rata-rata nilai elemen | `rata_rata([10, 20, 30])` → `20` |
| `gabung_larik(a, b)` | `daftar`, `daftar` | `daftar` | Menggabungkan dua daftar menjadi satu | `gabung_larik([1], [2])` → `[1, 2]` |
| `iris(lst, awal, akhir)` | `daftar`, `angka`, `angka` | `daftar` | Mengambil sub-bagian daftar (slicing) | `iris([10, 20, 30, 40], 1, 3)` → `[20, 30]` |
| `cari(lst, target)` | `daftar`, `apapun` | `angka` | Mencari indeks pertama kemunculan elemen (`-1` jika tidak ada) | `cari([10, 20], 20)` → `1` |
| `ada(lst, target)` | `daftar`, `apapun` | `bolean` | Memeriksa ketersediaan elemen dalam daftar | `ada([1, 2], 2)` → `benar` |
| `peta(lst, fn)` | `daftar`, `fungsi` | `daftar` | Menerapkan fungsi transformasi per elemen (`map`) | `peta([1, 2], fungsi(x) { kembali x * 2 })` → `[2, 4]` |
| `saring(lst, fn)` | `daftar`, `fungsi` | `daftar` | Menyaring elemen berdasarkan predikat (`filter`) | `saring([1, 6], fungsi(x) { kembali x > 5 })` → `[6]` |
| `lipat(lst, awal, fn)` | `daftar`, `apapun`, `fungsi` | `apapun` | Mengakumulasi seluruh isi daftar (`reduce`) | `lipat([1, 2], 0, fungsi(acc, x) { kembali acc + x })` → `3` |
| `setiap(lst, fn)` | `daftar`, `fungsi` | `kosong` | Menjalankan iterasi per elemen (`forEach`) | `setiap(data, fungsi(x) { cetak(x) })` |

---

### D. Modul Matematika
> **Implementasi (2026-09-10):** fungsi global tanpa impor (`impor matematika` diizinkan, no-op). `max` dipakai (bukan `maks`); alias `pembulatan`/`bundar` dan `mutlak`/`abs` dua-duanya berlaku; konstanta `pi` & `e` tersedia langsung. Error domain: `akar(-4)`/`log(0)`/`log(-1)` → `BAHAYA [MATH]`; `faktorial` negatif/fraksi/`> 170` → `BAHAYA [JUMLAH]`. `acak_antara(min, max)` inklusif, otomatis swap bila `min > max`.

| Item | Tipe | Deskripsi | Contoh Penggunaan |
|------|------|-----------|-------------------|
| `akar(x)` | Fungsi | Menghitung akar kuadrat | `akar(25)` → `5` |
| `pangkat(a, b)` | Fungsi | Menghitung `a` pangkat `b` | `pangkat(2, 3)` → `8` |
| `bulat_bawah(x)` | Fungsi | Membulatkan desimal ke bawah (floor) | `bulat_bawah(4.9)` → `4` |
| `bulat_atas(x)` | Fungsi | Membulatkan desimal ke atas (ceil) | `bulat_atas(4.1)` → `5` |
| `pembulatan(x)` / `bundar(x)` | Fungsi | Membulatkan ke bilangan bulat terdekat (round) | `pembulatan(4.5)` → `5` |
| `mutlak(x)` / `abs(x)` | Fungsi | Menghitung nilai absolut | `mutlak(-10)` → `10` |
| `acak_antara(min, max)` | Fungsi | Menghasilkan bilangan bulat acak inklusif di antara min dan max | `acak_antara(1, 10)` |
| `log(x)` | Fungsi | Menghitung nilai logaritma natural | `log(10)` |
| `faktorial(x)` | Fungsi | Menghitung nilai faktorial secara matematis (integer ≤ 170) | `faktorial(5)` → `120` |
| `min(a, b)` | Fungsi | Mengembalikan nilai terkecil | `min(10, 5)` → `5` |
| `max(a, b)` | Fungsi | Mengembalikan nilai terbesar | `max(10, 5)` → `10` |
| `sin(x)` / `cos(x)` / `tan(x)` | Fungsi | Fungsi trigonometri dasar (dalam radian) | `sin(0)` → `0` |
| `pi` | Konstanta | Nilai Pi ($\approx 3.141592653589793$) | `variabel k = 2 * pi * r` |
| `e` | Konstanta | Bilangan Euler ($\approx 2.718281828459045$) | `variabel log_e = e` |

---

### E. Modul Kamus (`impor kamus`)
Utilitas pemrosesan struktur data asosiatif pasangan kunci-nilai.

> **Implementasi (2026-09-10):** fungsi global tanpa impor (`impor kamus` diizinkan, no-op). `setel/hapus_kunci` memutasi kamus in-place → `kosong`; `kunci/nilai/pasangan/gabung_objek` mengembalikan baru. Hasil `kunci/nilai/pasangan` **diurutkan alfabetis berdasarkan kunci** (konsisten, bukan urutan acak). `dapatkan(k, kunci, bawaan?)` — argc 2 atau 3; kunci hilang tanpa `bawaan` → `BAHAYA [KEY]`. `gabung_objek(a, b)` — kunci yang sama dimenangkan `b`. Kunci non-teks dalam perbandingan dikonversi ke teks. Statement `hapus k["x"]` dan `hapus k.nama` menghapus entri; kunci tidak ada → diam-diam `kosong` (lihat keyword `hapus`).

| Fungsi | Parameter | Tipe Kembalian | Deskripsi | Contoh Penggunaan |
|--------|-----------|----------------|-----------|-------------------|
| `kunci(k)` | `kamus` | `daftar` | Mengambil seluruh daftar kunci | `kunci({a: 1, b: 2})` → `["a", "b"]` |
| `nilai(k)` | `kamus` | `daftar` | Mengambil seluruh daftar nilai | `nilai({a: 1, b: 2})` → `[1, 2]` |
| `pasangan(k)` | `kamus` | `daftar` | Mengambil daftar pasangan `[kunci, nilai]` | `pasangan({a: 1})` → `[["a", 1]]` |
| `ada_kunci(k, target)` | `kamus`, `apapun` | `bolean` | Memeriksa apakah kunci ada di dalam kamus | `ada_kunci(k, "nama")` → `benar` |
| `hapus_kunci(k, target)` | `kamus`, `apapun` | `kosong` | Menghapus entri kunci dari kamus | `hapus_kunci(k, "umur")` |
| `dapatkan(k, kunci, bawaan?)` | `kamus`, `apapun`, `apapun?` | `apapun` | Mengambil nilai dengan nilai cadangan | `dapatkan(k, "skor", 0)` |
| `setel(k, kunci, nilai)` | `kamus`, `apapun`, `apapun` | `kosong` | Memperbarui atau menambah nilai kunci | `setel(k, "skor", 100)` |
| `gabung_objek(a, b)` | `kamus`, `kamus` | `kamus` | Menggabungkan dua kamus menjadi kamus baru | `gabung_objek(d1, d2)` |

---

### F. Modul Tipe & Utilitas (`impor utilitas` / global)
Pemeriksaan tipe, refleksi data, dan penyalinan memori.

> **Implementasi (2026-09-10):** fungsi global tanpa impor (`impor utilitas` diizinkan, no-op). `adalah_*` → `benar/salah` (argumen hilang → `salah`). `ke_boolean`/`e_boolean`/`bolean` → `is_truthy` (angka ≠ 0, teks tak kosong, daftar/kamus tak kosong). `ke_larik`/`daftar` → teks→daftar karakter, daftar→salinan dangkal, kamus→daftar kunci diurut alfabetis; lain → `BAHAYA [TYPE]`. `salin(x)` **deep copy** (daftar/kamus rekursif; struktur siklik aman; fungsi → dibagikan, bukan disalin). Konversi dasar: `angka()` (Angka→identitas, Bolean→1/0, Teks→parse gagal → `[NaN]`), `teks()` (semua tipe → `to_string`), `bolean()` (≡ `ke_boolean`), `daftar()` (≡ `ke_larik`), `kamus()` (Kamus→klon; Daftar pasangan `[kunci, nilai]` → kamus, elemen salah ukuran/tipe → `[TYPE]`). Alias `e_boolean`≡`bolean`, `daftar`≡`ke_larik` (STDLIB `bolean()`/`daftar()`/`kamus()` konstruktor digabung).

| Fungsi | Parameter | Tipe Kembalian | Deskripsi | Contoh Penggunaan |
|--------|-----------|----------------|-----------|-------------------|
| `adalah_angka(x)` | `apapun` | `bolean` | Memeriksa apakah nilai bertipe `angka` | `adalah_angka(42)` → `benar` |
| `adalah_teks(x)` | `apapun` | `bolean` | Memeriksa apakah nilai bertipe `teks` | `adalah_teks("a")` → `benar` |
| `adalah_daftar(x)`| `apapun` | `bolean` | Memeriksa apakah nilai bertipe `daftar` | `adalah_daftar([])` → `benar` |
| `adalah_kamus(x)` | `apapun` | `bolean` | Memeriksa apakah nilai bertipe `kamus` | `adalah_kamus({})` → `benar` |
| `ke_boolean(x)` / `e_boolean(x)` | `apapun` | `bolean` | Mengonversi nilai secara ketat ke `bolean` | `e_boolean(1)` → `benar` |
| `ke_larik(x)` / `daftar(x)` | `apapun` | `daftar` | Mengonversi teks menjadi daftar karakter / daftar menjadi salinan / kamus menjadi daftar kunci | `ke_larik("abc")` → `["a", "b", "c"]` |
| `salin(x)` | `apapun` | `apapun` | Membuat salinan mendalam (deep copy) suatu objek/daftar | `variabel klon = salin(objek_asli)` |
| `angka(x)` | `apapun` | `angka` | Konversi dasar ke angka (teks, bolean) | `angka("12.5")` → `12.5` |
| `teks(x)` | `apapun` | `teks` | Konversi dasar ke teks | `teks(42)` → `"42"` |

---

### G. Modul Sistem & Waktu (`impor sistem`)
Operasi berkas, waktu, dan variabel lingkungan.

> **Implementasi (2026-09-10):** fungsi global tanpa impor (`impor sistem` diizinkan, no-op). `baca_file`/`tulis_file`/`ada_file` memakai operasi instan (`std::fs`); kegagalan IO → `BAHAYA [FILE]`. `env(nama, bawaan?)` — argc 1–2; tanpa bawaan dan variabel hilang → `BAHAYA [ENV]`. `tanggal_sekarang(ts?)` & `format_tanggal(ts, pola)` — ts timestamp milidetik, tampilan **UTC**; `tanggal_sekarang()` tanpa arg = waktu sekarang. Pola `format_tanggal`: token `YYYY MM DD HH mm ss` (sisanya disalin apa adanya). `tunda(detik)` terima desimal, negatif → `BAHAYA [WAKTU]`. `selisih_waktu(t1, t2)` → selisih mutlak. Kode error baru: `[ENV]`, `[WAKTU]` (lihat `ERROR.md`). **`argumen()` diimplementasikan di Fase 4A** (id 75: mengembalikan `daftar` argumen program yang diberikan setelah `--` di CLI). **Handle berkas (`buka/tutup/baca_baris/tulis`) DITUNDA** — tidak diimplementasikan; segera pakai `baca_file`/`tulis_file` instan.

| Fungsi | Parameter | Tipe Kembalian | Deskripsi | Contoh Penggunaan |
|--------|-----------|----------------|-----------|-------------------|
| `argumen()` | *tanpa parameter* | `daftar` | Mengembalikan daftar argumen baris perintah program (setelah `--`) | `variabel args = argumen()` |
| `waktu_sekarang()` | *tanpa parameter* | `angka` | Mengembalikan timestamp epoch dalam milidetik | `variabel t = waktu_sekarang()` |
| `tanggal_sekarang(ts?)` | `angka?` | `teks` | Tanggal/jam saat ini ISO (UTC); ts opsional | `tanggal_sekarang()` / `tanggal_sekarang(0)` |
| `format_tanggal(ts, format)`| `angka`, `teks` | `teks` | Memformat timestamp sesuai pola (`YYYY MM DD HH mm ss`) | `format_tanggal(t, "YYYY-MM-DD")` |
| `tunda(detik)` | `angka` | `kosong` | Menjeda eksekusi selama n detik | `tunda(1.5)` |
| `selisih_waktu(t1, t2)` | `angka`, `angka` | `angka` | Menghitung selisih durasi antara dua timestamp | `selisih_waktu(t2, t1)` |
| `baca_file(path)` | `teks` | `teks` | Membaca seluruh isi berkas secara instan | `variabel isi = baca_file("data.txt")` |
| `tulis_file(path, teks)` | `teks`, `teks` | `kosong` | Menulis/menimpa teks ke berkas secara instan | `tulis_file("data.txt", "Halo")` |
| `buka(path, mode)` | `teks`, `teks` | `objek` | DITUNDA — belum diimplementasikan | `~` |
| `tutup(f)` | `objek` | `kosong` | DITUNDA — belum diimplementasikan | `~` |
| `baca_baris(f)` | `objek` | `teks` | DITUNDA — belum diimplementasikan | `~` |
| `tulis(f, teks)` | `objek`, `teks` | `kosong` | DITUNDA — belum diimplementasikan | `~` |
| `ada_file(path)` | `teks` | `bolean` | Memeriksa ketersediaan berkas di filesystem | `ada_file("config.eve")` → `benar` |
| `env(nama, bawaan?)` | `teks`, `apapun?` | `teks` | Mengambil variabel lingkungan OS (bawaan jika tidak ada) | `env("DB_HOST", "localhost")` |
| `atur_env(nama, nilai)` | `teks`, `teks` | `kosong` | Mengatur variabel lingkungan OS | `atur_env("APP_ENV", "dev")` |

---

## 3. Status Finalisasi

- [x] 7 Kategori modul pustaka standar terdefinisi lengkap
- [x] Sinkronisasi nama fungsi dengan keputusan bahasa (`besar`/`kecil`, `bersih`, `tambah`, `ada_kunci`)
- [x] Penambahan fitur waktu, deep copy, pemeriksaan tipe, dan utilitas berkas cepat
