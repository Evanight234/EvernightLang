# KEYWORD.md — Daftar Kata Kunci (Keywords) EvernightLanguage

> Sumber kebenaran untuk seluruh kata kunci resmi dalam EvernightLanguage.
> Kata kunci di bawah ini dicadangkan (reserved words) dan tidak boleh digunakan sebagai identifier (nama variabel/fungsi).

---

## Aturan Kata Kunci

- Kata kunci ditulis dalam **huruf kecil semua** (case-sensitive).
- Tidak dapat digunakan sebagai nama pengenal (identifier) variabel, fungsi, maupun kelas.
- Kata kunci dikelompokkan menjadi **Fase 1 (Inti / Aktif)** dan **Fase 2 (Cadangan Masa Depan)**.

---

## 1. Kata Kunci Fase 1 (Inti & Aktif)

### A. Tipe & Literal
| Kata Kunci | Padanan (EN) | Deskripsi & Fungsi | Contoh Sintaksis |
|------------|--------------|-------------------|------------------|
| `benar` | `true` | Nilai kebenaran boolean positif | `variabel status = benar` |
| `salah` | `false` | Nilai kebenaran boolean negatif | `variabel aktif = salah` |
| `kosong` | `null` / `nil` | Menandakan ketiadaan nilai | `variabel data = kosong` |
| `angka` | `number` | Tipe data numerik (bilangan bulat & desimal) | `angka("123")` |
| `teks` | `string` | Tipe data untaian karakter UTF-8 | `teks(42)` |
| `bolean` | `boolean` | Tipe data nilai kebenaran | `bolean(1)` |

### B. Deklarasi Variabel & Konstanta
| Kata Kunci | Padanan (EN) | Deskripsi & Fungsi | Contoh Sintaksis |
|------------|--------------|-------------------|------------------|
| `variabel` | `let` / `var` | Mendeklarasikan variabel yang nilainya dapat diubah | `variabel skor = 100` |
| `tetap` | `const` | Mendeklarasikan konstanta mutlak | `tetap PI = 3.14159` |

### C. Percabangan (Branching)
| Kata Kunci | Padanan (EN) | Deskripsi & Fungsi | Contoh Sintaksis |
|------------|--------------|-------------------|------------------|
| `jika` | `if` | Mengevaluasi blok kondisi pertama | `jika nilai > 80 { ... }` |
| `lainnya_jika` | `else if` | Mengevaluasi kondisi lanjutan jika sebelumnya `salah` | `lainnya_jika nilai > 60 { ... }` |
| `lainnya` | `else` | Blok penampung jika semua kondisi `salah` | `lainnya { ... }` |
| `cocok` | `switch` / `match` | Struktur pencocokan nilai terhadap pola/kasus | `cocok opsi { ... }` |
| `kasus` | `case` | Cabang kasus spesifik dalam struktur `cocok` | `kasus 1 { cetak("Satu") }` |
| `bawaan` | `default` | Cabang default dalam struktur `cocok` | `bawaan { cetak("Lainnya") }` |
| `_` | `_` (wildcard) | Pola penangkap semua (wildcard) dalam struktur `cocok` | `kasus _ { cetak("Lainnya") }` |

### D. Perulangan (Looping)
| Kata Kunci | Padanan (EN) | Deskripsi & Fungsi | Contoh Sintaksis |
|------------|--------------|-------------------|------------------|
| `selama` | `while` | Mengulang blok selama kondisi bernilai `benar` | `selama x < 10 { x = x + 1 }` |
| `untuk` | `for` | Mengulang rentang angka atau iterasi elemen | `untuk item dalam daftar { ... }` |
| `dalam` | `in` | Operator keanggotaan koleksi / iterasi `untuk` | `untuk i dari 0 sampai 5` / `x dalam list` |
| `berhenti` | `break` | Menghentikan dan keluar dari perulangan secara paksa | `jika i == 5 { berhenti }` |
| `lanjut` | `continue` | Melompati sisa blok dan lanjut ke iterasi berikutnya | `jika i == 2 { lanjut }` |

### E. Fungsi (Function)
| Kata Kunci | Padanan (EN) | Deskripsi & Fungsi | Contoh Sintaksis |
|------------|--------------|-------------------|------------------|
| `fungsi` | `fn` / `func` | Mendeklarasikan sebuah fungsi atau lambda | `fungsi tambah(a, b) { ... }` |
| `kembali` | `return` | Mengembalikan nilai dari dalam fungsi | `kembali a + b` |

### F. Operator Logika & Perbandingan Berbasis Kata
| Kata Kunci | Padanan (EN) | Simbol Ekivalen | Contoh Sintaksis |
|------------|--------------|-----------------|------------------|
| `dan` | `and` | `&&` | `jika umur >= 18 dan punya_ktp { ... }` |
| `atau` | `or` | `\|\|` | `jika status == "admin" atau status == "root" { ... }` |
| `bukan` | `not` | `!` | `jika bukan aktif { ... }` |
| `sama_dengan` | `equal` | `==` | `jika x sama_dengan 10 { ... }` |
| `lebih_dari` | `greater than` | `>` | `jika skor lebih_dari 50 { ... }` |
| `kurang_dari` | `less than` | `<` | `jika suhu kurang_dari 0 { ... }` |

### G. Penanganan Kesalahan (Error Handling)
| Kata Kunci | Padanan (EN) | Deskripsi & Fungsi | Contoh Sintaksis |
|------------|--------------|-------------------|------------------|
| `coba` | `try` | Membuka blok pemantauan potensi kesalahan | `coba { ... }` |
| `tangkap` | `catch` | Menangkap dan mengisolasi pesan kesalahan | `tangkap(err) { ... }` |
| `akhirnya` | `finally` | Blok yang selalu dieksekusi di akhir `coba` | `akhirnya { tutup(f) }` |
| `lempar` | `throw` | Memicu kesalahan fatal secara eksplisit | `lempar("Data rusak!")` |
| `pastikan` | `assert` | Memvalidasi kondisi mutlak bernilai benar | `pastikan(x > 0, "x harus positif")` |

### H. Sistem Modul & Impor
| Kata Kunci | Padanan (EN) | Deskripsi & Fungsi | Contoh Sintaksis |
|------------|--------------|-------------------|------------------|
| `impor` | `import` | Memuat modul pustaka eksternal / file `.eve` lain | `impor matematika` |
| `dari` | `from` | Mengimpor item tertentu secara selektif | `impor akar, pi dari "matematika"` |
| `sebagai` | `as` | Memberikan nama alias pada modul/item impor | `impor matematika sebagai mtk` |

### I. Masukan & Keluaran Dasar (I/O)
| Kata Kunci | Padanan (EN) | Deskripsi & Fungsi | Contoh Sintaksis |
|------------|--------------|-------------------|------------------|
| `cetak` | `print` | Mencetak output ke terminal standar | `cetak("Halo Dunia")` |
| `baca` | `read / input` | Membaca input baris teks dari terminal | `variabel input = baca("Prompt: ")` |

---

## 2. Kata Kunci Fase 2 (Cadangan OOP & Lanjutan)

Kata kunci ini dicadangkan untuk tahap pengembangan berorientasi objek, asinkron, dan manajemen memori lanjutan:

| Kelompok | Kata Kunci yang Dicadangkan |
|----------|-----------------------------|
| **OOP & Struktur** | `objek`, `larik`, `ini`, `warisi`, `induk`, `baru`, `privat`, `publik`, `statis`, `abstrak`, `konstruktor`, `destruktor`, `antarmuka`, `implementasi`, `struktur`, `enum`, `operator` |
| **Asinkron & Konkurensi** | `async`, `tunggu`, `paralel`, `kunci`, `sinkron` |
| **Generator & Memori** | `hasil`, `panggil_ulang`, `lakukan`, `dengan`, `hapus`, `jenis`, `global`, `lokal`, `debug`, `peringatan` |

---

## 3. Status Finalisasi

- [x] Seluruh kata kunci inti Fase 1 terdefinisi lengkap
- [x] Sinkronisasi nama kata kunci: `variabel`, `fungsi`, `kembali`, `impor`
- [x] Kata kunci perulangan `berhenti`, `lanjut`, `dalam` diakomodasi
- [x] Wildcard `_` ditambahkan ke pattern matching `cocok`
- [x] Kata kunci error `coba`, `tangkap`, `akhirnya`, `lempar`, `pastikan` disepakati
- [x] Pencadangan kata kunci masa depan (Fase 2) didokumentasikan
