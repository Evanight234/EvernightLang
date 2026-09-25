export interface Soal {
  tanya: string;
  pilihan: [string, string, string, string];
  benar: number;
}

export interface KuisTahap {
  slug: string;
  soal: Soal[];
}

export const kuis: Record<string, KuisTahap> = {
  '01-halo-dunia': {
    slug: '01-halo-dunia',
    soal: [
      {
        tanya: 'Komentar dalam program EvernightLanguage ditulis dengan simbol apa?',
        pilihan: ['#', '//', '/* ... */', '--'],
        benar: 0,
      },
      {
        tanya: 'Berkas `halo.eve` dijalankan lewat perintah apa?',
        pilihan: ['evernight halo.eve', 'eve halo.eve', 'evernight build halo.eve', 'gcc halo.eve'],
        benar: 0,
      },
      {
        tanya: 'Program EvernightLanguage disimpan dengan akhiran berkas apa?',
        pilihan: ['.eve', '.ev', '.en', '.evernight'],
        benar: 0,
      },
      {
        tanya: 'Apa output dari program berikut?\n`variabel pesan = "Halo"\nvariabel nama = "Dunia"\ncetak(pesan, " ", nama)`',
        pilihan: ['Halo Dunia', 'HaloDunia', 'pesan nama', 'Halo " " Dunia'],
        benar: 0,
      },
    ],
  },
  '02-variabel-tipe': {
    slug: '02-variabel-tipe',
    soal: [
      {
        tanya: 'Nilai konstanta (tidak boleh diubah) dideklarasikan dengan keyword apa?',
        pilihan: ['tetap', 'konstan', 'const', 'kunci'],
        benar: 0,
      },
      {
        tanya: 'Nilai bolean bernilai positif ditulis...',
        pilihan: ['benar', 'true', 'ya', 'positif'],
        benar: 0,
      },
      {
        tanya: 'Apa output dari `cetak(angka("42") + 8)`?',
        pilihan: ['50', '428', '42 + 8', '"50"'],
        benar: 0,
      },
      {
        tanya: 'Apa yang terjadi saat menjalankan `cetak(10 + "5")`?',
        pilihan: [
          'Program gagal dengan BAHAYA tipe',
          'Mencetak 105',
          'Mencetak 15',
          'Mencetak 105 sebagai teks',
        ],
        benar: 0,
      },
    ],
  },
  '03-percabangan': {
    slug: '03-percabangan',
    soal: [
      {
        tanya: 'Blok ELSE dalam percabangan EvernightLanguage memakai keyword apa?',
        pilihan: ['lainnya', 'else', 'kemudian', 'selanjutnya'],
        benar: 0,
      },
      {
        tanya: 'Operator yang mensyaratkan KEDUA kondisi bernilai `benar` adalah...',
        pilihan: ['dan', 'atau', 'bukan', 'juga'],
        benar: 0,
      },
      {
        tanya: 'Apa output?\n`variabel n = 6\njika n % 2 == 0 { cetak("genap") } lainnya { cetak("ganjil") }`',
        pilihan: ['genap', 'ganjil', '6', 'kelipatan 2'],
        benar: 0,
      },
      {
        tanya: 'Manakah struktur `cocok` yang sah?',
        pilihan: [
          'cocok n { kasus 1 { cetak("Satu") } bawaan { cetak("Lain") } }',
          'cocok n { case 1 { cetak("Satu") } default { } }',
          'cocok n { kasus 1: cetak("Satu") }',
          'switch n { kasus 1 { cetak("Satu") } }',
        ],
        benar: 0,
      },
    ],
  },
  '04-perulangan': {
    slug: '04-perulangan',
    soal: [
      {
        tanya: '`untuk i dari 1 sampai 5 { cetak(i) }` mencetak berapa nilai?',
        pilihan: [
          'Lima nilai: 1, 2, 3, 4, 5',
          'Empat nilai: 1, 2, 3, 4',
          'Enam nilai: 0, 1, 2, 3, 4, 5',
          'Satu nilai: 5',
        ],
        benar: 0,
      },
      {
        tanya: 'Keyword untuk keluar dari perulangan secara paksa:',
        pilihan: ['berhenti', 'hentikan', 'stop', 'break'],
        benar: 0,
      },
      {
        tanya: 'Apa fungsi `lanjut` di dalam perulangan?',
        pilihan: [
          'Langsung ke iterasi berikutnya, lewati sisa blok',
          'Keluar dari perulangan',
          'Menghentikan program',
          'Mengulang dari awal perulangan',
        ],
        benar: 0,
      },
      {
        tanya: 'Manakah deklarasi perulangan `selama` yang sah?',
        pilihan: [
          'selama i < 3 { i = i + 1 }',
          'while i < 3 { i = i + 1 }',
          'selama i < 3 lakukan { i = i + 1 }',
          'ulang i < 3 { i = i + 1 }',
        ],
        benar: 0,
      },
    ],
  },
  '05-fungsi': {
    slug: '05-fungsi',
    soal: [
      {
        tanya: 'Manakah deklarasi fungsi yang sah?',
        pilihan: [
          'fungsi tambah(a, b) { kembali a + b }',
          'function tambah(a, b) { return a + b }',
          'def tambah(a, b):',
          'fn tambah(a, b) { }',
        ],
        benar: 0,
      },
      {
        tanya: 'Keyword untuk mengembalikan nilai dari fungsi:',
        pilihan: ['kembali', 'return', 'hasil', 'keluar'],
        benar: 0,
      },
      {
        tanya: 'Apa hasil dari `faktorial(5)` jika fungsi rekursinya benar?',
        pilihan: ['120', '24', '720', '25'],
        benar: 0,
      },
      {
        tanya: 'Fungsi rekursi wajib punya apa supaya tidak berhenti tanpa hasil?',
        pilihan: [
          'Basi — kondisi berhenti',
          'Perulangan `selama` di dalamnya',
          'Variabel global',
          'Minimal dua parameter',
        ],
        benar: 0,
      },
    ],
  },
  '06-daftar-kamus': {
    slug: '06-daftar-kamus',
    soal: [
      {
        tanya: 'Menambahkan elemen baru di AKHIR daftar `belanja`:',
        pilihan: [
          'tambah(belanja, "minyak")',
          'tambah("minyak", belanja)',
          'sisip(belanja, "minyak")',
          'push(belanja, "minyak")',
        ],
        benar: 0,
      },
      {
        tanya: 'Jumlah elemen daftar `belanja` dibaca lewat properti...',
        pilihan: ['belanja.panjang', 'belanja.length', 'belanja.jumlah', 'belanja.ukuran'],
        benar: 0,
      },
      {
        tanya: 'Mengambil nilai kamus yang kuncinya mungkin tidak ada, cara paling aman:',
        pilihan: [
          'dapatkan(skor, "Andi", 0)',
          'ambil(skor, "Andi", 0)',
          'nilai(skor, "Andi", 0)',
          'kunci(skor, "Andi", 0)',
        ],
        benar: 0,
      },
      {
        tanya:
          'Apa output?\n`variabel belanja = ["beras", "telur"]\ncetak(belanja[0], " ", belanja.panjang)`',
        pilihan: ['beras 2', 'telur 2', 'beras 3', '0 beras'],
        benar: 0,
      },
    ],
  },
  '07-string-matematika': {
    slug: '07-string-matematika',
    soal: [
      {
        tanya: 'Apa output dari `cetak(potong("Nusantara", 0, 4))`?',
        pilihan: ['Nusa', 'Nusan', 'Nusantara', 'usa'],
        benar: 0,
      },
      {
        tanya: 'Apa output dari `cetak(pecah("apel,mangga,jeruk", ","))`?',
        pilihan: ['[apel, mangga, jeruk]', 'apel,mangga,jeruk', '[apel mangga jeruk]', '3'],
        benar: 0,
      },
      {
        tanya: 'Apa hasil `pangkat(2, 10)`?',
        pilihan: ['1024', '20', '100', '210'],
        benar: 0,
      },
      {
        tanya: 'Manakah ekspresi yang menghasilkan `benar`?',
        pilihan: [
          'mengandung("Evernight", "night")',
          'besar("night")',
          'potong("Evernight", 0, 4)',
          'ganti("Evernight", "night", "")',
        ],
        benar: 0,
      },
    ],
  },
  '08-konsol-file': {
    slug: '08-konsol-file',
    soal: [
      {
        tanya: 'Membaca input angka dari pengguna (mengembalikan `angka`):',
        pilihan: [
          'baca_angka("Umur: ")',
          'baca("Umur: ")',
          'angka_baca("Umur: ")',
          'baca_teks_angka("Umur: ")',
        ],
        benar: 0,
      },
      {
        tanya: 'Menurut aturan sandbox, jalur berkas mana yang benar?',
        pilihan: ['./catatan.txt', '/catatan.txt', 'C:\\catatan.txt', '~/catatan.txt'],
        benar: 0,
      },
      {
        tanya: 'Membaca SELURUH isi berkas teks:',
        pilihan: [
          'baca_file("./catatan.txt")',
          'buka("./catatan.txt")',
          'baca("./catatan.txt")',
          'muat_file("./catatan.txt")',
        ],
        benar: 0,
      },
      {
        tanya:
          'Apa output?\n`tulis_file("./catatan.txt", "Halo")\ncetak(ada_file("./catatan.txt"))`',
        pilihan: ['benar', 'salah', 'kosong', '1'],
        benar: 0,
      },
    ],
  },
  '09-kesalahan': {
    slug: '09-kesalahan',
    soal: [
      {
        tanya: 'Manakah blok `coba` / `tangkap` yang sah?',
        pilihan: [
          'coba { cetak(d[9]) } tangkap (e) { cetak(e) }',
          'coba { cetak(d[9]) } tangkap e { cetak(e) }',
          'coba { cetak(d[9]) } tangkap [e] { cetak(e) }',
          'coba { cetak(d[9]) } kecuali (e) { cetak(e) }',
        ],
        benar: 0,
      },
      {
        tanya: 'Keyword untuk memicu kesalahan sendiri:',
        pilihan: [
          'lempar("data rusak!")',
          'pancing("data rusak!")',
          'gagal("data rusak!")',
          'error("data rusak!")',
        ],
        benar: 0,
      },
      {
        tanya:
          'Tanpa `coba`, `pastikan(1 + 1 == 3, "Hitungan rusak!")` membuat program...',
        pilihan: [
          'berhenti dengan BAHAYA [ASSERT] Hitungan rusak!',
          'mencetak PERINGATAN lalu tetap lanjut',
          'berjalan normal karena hanya cek ringan',
          'mencetak nilai salah',
        ],
        benar: 0,
      },
      {
        tanya: 'Blok `akhirnya { }` dijalankan...',
        pilihan: [
          'selalu, baik berhasil maupun gagal',
          'hanya jika terjadi kesalahan',
          'hanya jika tidak ada kesalahan',
          'menggantikan blok `tangkap`',
        ],
        benar: 0,
      },
    ],
  },
  '10-mini-projek': {
    slug: '10-mini-projek',
    soal: [
      {
        tanya:
          'Perulangan yang membuat menu kalkulator terus ditampilkan sampai pengguna memilih `6`:',
        pilihan: [
          'selama pilihan != "6" { ... }',
          'untuk pilihan dari 1 sampai 6 { ... }',
          'selama pilihan != 6 { ... }',
          'jika pilihan != "6" { ... }',
        ],
        benar: 0,
      },
      {
        tanya:
          'Menyusun teks riwayat `10 + 4 = 14` dari angka `a`, `b`, dan `hasil` — mana yang benar?',
        pilihan: [
          'teks(a) + " + " + teks(b) + " = " + teks(hasil)',
          'a + " + " + b + " = " + hasil',
          'angka(a) + " + " + angka(b)',
          'kamus(a) + " + " + kamus(b)',
        ],
        benar: 0,
      },
      {
        tanya:
          'Saat memilih menu bagi (`4`) dengan `b` = 0, pesan kesalahan dari `pastikan` adalah...',
        pilihan: [
          'Pembagi tidak boleh nol!',
          'Pembagian dengan nol tidak diizinkan!',
          'BAHAYA [DIVISION]',
          'Gagal: ',
        ],
        benar: 0,
      },
      {
        tanya: 'Mengapa kondisi menu ditulis `pilihan != "6"` (dengan tanda petik)?',
        pilihan: [
          'Karena `baca()` mengembalikan teks',
          'Karena angka selalu ditulis sebagai teks',
          'Supaya perulangan lebih cepat',
          'Karena 6 adalah konstanta `tetap`',
        ],
        benar: 0,
      },
    ],
  },
};
