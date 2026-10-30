---
judul: "Data Science & ML Library (Matra)"
grup: "Rujukan"
urutan: 10
---

# Data Science & ML Library (Matra)

The `matra` library covers the modern data science workflow. Import the library with:

```eve
impor "matra" sebagai mt
```

---

## 1. `matra.larik` (Tensors & Linear Algebra)

| Function / Method | Argument Types | Return | Description |
|---|---|---|---|
| `mt.larik(data)` | 1D or 2D `Daftar` | `Larik` | Constructs a new n-dimensional tensor |
| `mt.nol(baris, kolom)` | `Angka, Angka` | `Larik` | Initializes a matrix of zeros |
| `mt.satu(baris, kolom)` | `Angka, Angka` | `Larik` | Initializes a matrix of ones |
| `mt.rentang(mulai, stop, langkah)` | `Angka, Angka, Angka` | `Larik` | Generates a range array of numbers |
| `.bentuk` | Property | `Daftar` | Returns tensor shape dimensions `[rows, cols]` |
| `.transpos()` | - | `Larik` | Transposes the 2D matrix |
| `.titik(b)` | `Larik` | `Larik` | Computes matrix multiplication or vector dot product |
| `.rata_rata()` | - | `Angka` | Computes mean of elements |
| `.varians()` | - | `Angka` | Computes variance of elements |
| `.deviasi_standar()` | - | `Angka` | Computes standard deviation |
| `.min()` / `.max()` | - | `Angka` | Minimum / maximum element |

---

## 2. `matra.tabel` (DataFrame & CSV Processing)

| Function / Method | Argument Types | Return | Description |
|---|---|---|---|
| `mt.tabel(kamus)` | Column `Kamus` | `Tabel` | Constructs a DataFrame from a dictionary |
| `mt.baca_csv(path)` | `Teks` | `Tabel` | Reads a local CSV file into a DataFrame |
| `.simpan_csv(path)` | `Teks` | `Kosong` | Saves the DataFrame to a CSV file |
| `.kepala(n?)` | Optional `Angka` | `Kosong` | Prints top `n` rows (default 5) |
| `.ekor(n?)` | Optional `Angka` | `Kosong` | Prints bottom `n` rows |
| `.ringkasan()` | - | `Kosong` | Prints descriptive summary statistics |
| `.kolom(nama)` | `Teks` | `Daftar` | Extracts a single column as a list |
| `.pilih_kolom(daftar)` | `Daftar<Teks>` | `Tabel` | Selects a subset of columns |
| `.tambah_kolom(nama, data)` | `Teks, Daftar` | `Kosong` | Appends a new column to the table |
| `.buang_kosong()` | - | `Tabel` | Drops rows containing missing values |
| `.isi_kosong(kolom, nilai)` | `Teks, Nilai` | `Tabel` | Imputes missing values in a column |
| `.kelompokkan(kolom)` | `Teks` | `Grup` | Groups rows for aggregate operations |

---

## 3. `matra.grafik` (SVG Vector Visualization)

| Chaining Method | Argument Types | Return | Description |
|---|---|---|---|
| `mt.grafik()` | - | `Grafik` | Creates a new plot canvas |
| `.judul(teks)` | `Teks` | `Grafik` | Sets main chart title |
| `.label_x(teks)` / `.label_y(teks)` | `Teks` | `Grafik` | Sets X and Y axis labels |
| `.garis(x, y, opsi?)` | `Daftar, Daftar, Teks?` | `Grafik` | Adds a series line plot |
| `.sebar(x, y, opsi?)` | `Daftar, Daftar, Teks?` | `Grafik` | Adds a scatter correlation plot |
| `.batang(kategori, nilai)` | `Daftar, Daftar` | `Grafik` | Adds a categorical bar chart |
| `.histogram(data, bin)` | `Daftar, Angka` | `Grafik` | Adds a frequency distribution histogram |
| `.simpan_svg(path)` | `Teks` | `Kosong` | Renders and saves vector graphic to SVG |

---

## 4. `matra.latih`, `matra.model`, `matra.syaraf`

| Component | Invocation Syntax | Primary Role |
|---|---|---|
| `pisah_data` | `mt.latih.pisah_data(X, y, rasio)` | Splits datasets into train and test sets |
| `PipaKerja` | `mt.alur.PipaKerja([tahap1, tahap2])` | Chains transforms and models preventing leakage |
| `RegresiLinear` | `mt.model.RegresiLinear()` | Continuous numeric target prediction |
| `KNNKlasifikasi` | `mt.model.KNNKlasifikasi(k)` | Spatial K-Nearest Neighbors classifier |
| `HutanAcak` | `mt.model.HutanAcak(n_pohon)` | Random Forest ensemble classifier |
| `akurasi` | `mt.metrik.akurasi(y_asli, y_pred)` | Calculates prediction accuracy score |
| `laporan_klasifikasi` | `mt.metrik.laporan_klasifikasi(y, pred)` | Produces precision, recall, and F1 reports |
| `ModelSyaraf` | `mt.syaraf.ModelSyaraf([lapisan...])` | Multi-layer neural network model |
| `Pelatih` | `mt.pelatih.Pelatih(model, optim, loss)` | Deep learning training loop orchestrator |
