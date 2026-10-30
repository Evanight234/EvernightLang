---
judul: "Tahap 8: Masukan Konsol & Berkas"
grup: "Tutorial"
urutan: 8
---

# Tahap 8: Masukan Konsol & Berkas

## Tujuan

Menerima input pengguna dari terminal (`baca`, `baca_angka`) dan menyimpan/membaca
isi berkas lewat modul `sistem`.

## Konsep

### Masukan dari pengguna

```eve
variabel nama = baca("Siapa namamu? ")
variabel umur = baca_angka("Umur: ")
```

- `baca(...)` → mengembalikan `teks`
- `baca_angka(...)` → mengembalikan `angka` (input bukan angka akan gagal)
- `bersihkan()` → menghapus isi layar

### Berkas (`impor sistem`)

```eve
impor sistem

tulis_file("./catatan.txt", "Isi berkas")
cetak(baca_file("./catatan.txt"))
cetak(ada_file("./catatan.txt"))   # benar
```

| Fungsi | Kegunaan |
|--------|----------|
| `tulis_file(jalur, teks)` | menulis / menimpa berkas |
| `baca_file(jalur)` | membaca seluruh isi berkas |
| `ada_file(jalur)` | mengecek apakah berkas ada |

> **Aturan penting (sandbox):** jalur berkas hanya boleh berada di dalam folder program
> dan **diselesaikan dari folder tempat Anda menjalankan perintah**. Karena itu jalankan
> `evernight` dari folder yang sama tempat berkas `.eve` Anda berada, dan tulis jalur
> relatif dengan awalan `./` — bukan `data.txt` polos.

### Waktu

`waktu_sekarang()` mengembalikan timestamp milidetik; `tunda(0.5)` menahan program
setengah detik.

## Contoh lengkap

```eve
# Masukan pengguna & berkas
impor sistem

variabel nama = baca("Siapa namamu? ")
variabel umur = baca_angka("Umurmu berapa? ")
cetak("Halo ", nama, ", umurmu ", umur, " tahun")

variabel jalur = "./catatan.txt"
tulis_file(jalur, "Halo dari EvernightLanguage!\nNama: " + nama)
cetak("Berkas dibuat: ", ada_file(jalur))
cetak("Isi berkas:")
cetak(baca_file(jalur))

coba {
    baca_file("./tidak_ada.txt")
} tangkap (e) {
    cetak("Gagal membaca: ", e)
}
```

`\n` di dalam teks menghasilkan baris baru.

## Jalankan

Simpan sebagai `catat.eve` **di dalam satu folder**, buka terminal di folder itu, lalu:

```powershell
evernight catat.eve
```

Ketik `Budi` lalu `20`:

```
Siapa namamu? Umurmu berapa? Halo Budi, umurmu 20 tahun
Berkas dibuat: benar
Isi berkas:
Halo dari EvernightLanguage!
Nama: Budi
Gagal membaca: Gagal membaca berkas './tidak_ada.txt': <pesan sistem> (os error 2)
```

Berkas `catatan.txt` kini tersimpan di folder yang sama.

Contoh bawaan: `evernight examples/sistem.eve` (dijalankan dari folder repo).

## Latihan

1. Minta pengguna menulis 3 baris pesan, simpan ke `./pesan.txt`, lalu cetak ulang
   isinya.
2. Buat program yang mengecek `ada_file("./pesan.txt")` dan mencetak isi bila ada,
   atau `"Berkas belum dibuat"` bila tidak.
3. Tambahkan `cetak("Selesai dalam 1 detik")` + `tunda(1)` di akhir program.
