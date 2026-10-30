---
judul: "Pustaka Sains Data & ML (Matra)"
grup: "Rujukan"
urutan: 10
---

# Pustaka Sains Data & ML (Matra)

Pustaka `matra` mencakup seluruh alur kerja sains data modern. Pustaka ini diakses dengan mengimpor modul:

```eve
impor "matra" sebagai mt
```

---

## 1. `matra.larik` (Tensor & Aljabar Linier)

| Fungsi / Metode | Tipe Argumen | Kembalian | Deskripsi Singkat |
|---|---|---|---|
| `mt.larik(data)` | `Daftar` 1D atau 2D | `Larik` | Membuat tensor n-dimensi baru |
| `mt.nol(baris, kolom)` | `Angka, Angka` | `Larik` | Inisialisasi matriks berisi angka 0 |
| `mt.satu(baris, kolom)` | `Angka, Angka` | `Larik` | Inisialisasi matriks berisi angka 1 |
| `mt.rentang(mulai, stop, langkah)` | `Angka, Angka, Angka` | `Larik` | Membuat larik interval angka |
| `.bentuk` | Properti | `Daftar` | Mengembalikan dimensi bentuk `[baris, kolom]` |
| `.transpos()` | - | `Larik` | Menghasilkan transposisi matriks |
| `.titik(b)` | `Larik` | `Larik` | Perkalian matriks atau dot product vektor |
| `.rata_rata()` | - | `Angka` | Nilai rata-rata seluruh elemen |
| `.varians()` | - | `Angka` | Nilai varians elemen |
| `.deviasi_standar()` | - | `Angka` | Nilai deviasi standar elemen |
| `.min()` / `.max()` | - | `Angka` | Nilai minimum / maksimum |

---

## 2. `matra.tabel` (DataFrame & Olah CSV)

| Fungsi / Metode | Tipe Argumen | Kembalian | Deskripsi Singkat |
|---|---|---|---|
| `mt.tabel(kamus)` | `Kamus` kolom | `Tabel` | Membuat DataFrame baru dari kamus |
| `mt.baca_csv(path)` | `Teks` | `Tabel` | Membaca berkas CSV lokal ke DataFrame |
| `.simpan_csv(path)` | `Teks` | `Kosong` | Menyimpan DataFrame ke berkas CSV |
| `.kepala(n?)` | `Angka` opsional | `Kosong` | Menampilkan `n` baris teratas (default 5) |
| `.ekor(n?)` | `Angka` opsional | `Kosong` | Menampilkan `n` baris terbawah |
| `.ringkasan()` | - | `Kosong` | Menampilkan ringkasan statistik deskriptif |
| `.kolom(nama)` | `Teks` | `Daftar` | Mengambil data satu kolom |
| `.pilih_kolom(daftar)` | `Daftar<Teks>` | `Tabel` | Memilih subset kolom tertentu |
| `.tambah_kolom(nama, data)` | `Teks, Daftar` | `Kosong` | Menambahkan kolom baru ke tabel |
| `.buang_kosong()` | - | `Tabel` | Membuang baris yang memiliki nilai null |
| `.isi_kosong(kolom, nilai)` | `Teks, Nilai` | `Tabel` | Mengisi nilai null dengan nilai pengganti |
| `.kelompokkan(kolom)` | `Teks` | `Grup` | Mengelompokkan tabel untuk agregasi |

---

## 3. `matra.grafik` (Visualisasi Vektor SVG)

| Metode Chaining | Tipe Argumen | Kembalian | Deskripsi Singkat |
|---|---|---|---|
| `mt.grafik()` | - | `Grafik` | Membuat kanvas grafik baru |
| `.judul(teks)` | `Teks` | `Grafik` | Menetapkan judul utama grafik |
| `.label_x(teks)` / `.label_y(teks)` | `Teks` | `Grafik` | Menetapkan label sumbu X dan Y |
| `.garis(x, y, opsi?)` | `Daftar, Daftar, Teks?` | `Grafik` | Menambahkan grafik garis deret |
| `.sebar(x, y, opsi?)` | `Daftar, Daftar, Teks?` | `Grafik` | Menambahkan titik sebar korelasi (*scatter*) |
| `.batang(kategori, nilai)` | `Daftar, Daftar` | `Grafik` | Menambahkan grafik diagram batang |
| `.histogram(data, bin)` | `Daftar, Angka` | `Grafik` | Menambahkan grafik histogram distribusi |
| `.simpan_svg(path)` | `Teks` | `Kosong` | Merender dan menyimpan grafik ke berkas SVG |

---

## 4. `matra.latih`, `matra.model`, `matra.syaraf`

| Komponen | Sintaksis Pemanggilan | Peran Utama |
|---|---|---|
| `pisah_data` | `mt.latih.pisah_data(X, y, rasio)` | Memisahkan data menjadi subset latih & uji |
| `PipaKerja` | `mt.alur.PipaKerja([tahap1, tahap2])` | Rantai pra-proses dan pemodelan anti-bocor |
| `RegresiLinear` | `mt.model.RegresiLinear()` | Model prediksi numerik kontinu |
| `KNNKlasifikasi` | `mt.model.KNNKlasifikasi(k)` | Model klasifikasi tetangga spasial terdekat |
| `HutanAcak` | `mt.model.HutanAcak(n_pohon)` | Model ensemble pohon keputusan |
| `akurasi` | `mt.metrik.akurasi(y_asli, y_pred)` | Menghitung persentase kebenaran prediksi |
| `laporan_klasifikasi` | `mt.metrik.laporan_klasifikasi(y, pred)` | Menghasilkan tabel presisi, recall, F1 |
| `ModelSyaraf` | `mt.syaraf.ModelSyaraf([lapisan...])` | Jaringan syaraf tiruan multi-lapisan |
| `Pelatih` | `mt.pelatih.Pelatih(model, optim, loss)` | Orkestrator siklus pelatihan deep learning |
