# ARRAY.md — Daftar (Array) EvernightLanguage

> Spesifikasi tipe data `daftar` (list) untuk EvernightLanguage.
> Semantik mengadopsi Python 1:1 dengan keyword & nama method Bahasa Indonesia.
>
> **Implementasi (2026-09-10):** method syntax `a.tambah(x)` **belum didukung** — gunakan fungsi global modul `daftar` (`tambah(a, x)`, `sisip(a, i, x)`, `hapus(a, nil)`, dst; impl & tabel lengkap di `STDLIB.md §C`). Statement `hapus a[i]` menghapus elemen per indeks (wrap negatif, out-of-range `BAHAYA [INDEX]`). Slicing `a[awal:akhir]`, `a * n`, `nilai dalam a`, dan method `ambil`/`bersihkan` juga masih ditunda (baris tabel di bawah = target desain).

## 1. Keputusan yang Sudah Ada

| Aspek | Keputusan |
|-------|-----------|
| Nama tipe | **daftar** |
| Sifat | Dinamis — panjang berubah, bisa campur tipe data |
| Sistem tipe | Dinamis |
| Stdlib | Modul `daftar` (Fase 3 di `RENCANA.md`) |

## 2. Operasi Daftar (Semantik Python)

| Operasi | Sintaksis / Method | Python Equivalent | Contoh |
|---------|-------------------|-------------------|--------|
| Buat daftar | `[1, 2, 3]` / `daftar()` | `[1, 2, 3]` / `list()` | `variabel a = [1, "dua", 3.0]` |
| Akses indeks | `a[indeks]` | `a[index]` | `a[0]`, `a[-1]` (indeks negatif didukung) |
| Ubah elemen | `a[indeks] = nilai` | `a[index] = val` | `a[0] = 99` |
| Tambah akhir | `a.tambah(nilai)` | `a.append(val)` | `a.tambah(4)` |
| Sisip di indeks | `a.sisip(indeks, nilai)` | `a.insert(i, val)` | `a.sisip(0, "awal")` |
| Hapus nilai pertama | `a.hapus(nilai)` | `a.remove(val)` | `a.hapus("dua")` |
| Ambil & hapus | `a.ambil()` / `a.ambil(indeks)` | `a.pop()` / `a.pop(i)` | `variabel x = a.ambil()` |
| Panjang daftar | `panjang(a)` | `len(a)` | `panjang(a)` |
| Potong (slice) | `a[awal:akhir:langkah]` | `a[start:end:step]` | `a[1:3]`, `a[:2]`, `a[::-1]` |
| Cek keberadaan | `nilai dalam a` | `val in a` | `jika 2 dalam a { ... }` |
| Gabung daftar | `a + b` | `a + b` | `[1, 2] + [3, 4]` |
| Gandakan daftar | `a * n` | `a * n` | `[0] * 3` -> `[0, 0, 0]` |
| Bersihkan | `a.bersihkan()` | `a.clear()` | `a.bersihkan()` |

## 3. Sintaksis Literal

- Format: `[elemen1, elemen2, ...]` (kurung siku)
- Daftar kosong: `[]` atau `daftar()`

Contoh:
```eve
variabel angka = [10, 20, 30]
angka.tambah(40)
cetak(angka[0])        # 10
cetak(angka[-1])       # 40 (elemen terakhir)
cetak(angka[1:3])      # [20, 30]
```

## 4. Keputusan Desain

1. **Indeks**: Mulai dari **0**, mendukung indeks negatif (`-1` = elemen terakhir).
2. **Akses di luar batas**: Menghasilkan error **`BAHAYA: Indeks di luar batas jangkauan daftar!`** (IndexError).
3. **Nested daftar**: **Boleh** (daftar di dalam daftar / multi-dimensi didukung penuh, mis. `matrix = [[1, 2], [3, 4]]`).

---

## Status Finalisasi

- [x] Operasi daftar sudah dilengkapi (1:1 semantik Python)
- [x] Sintaksis literal sudah dipilih (`[...]`)
- [x] Pertanyaan desain sudah dijawab
