export interface Soal {
  tanya: string;
  tanyaEn?: string;
  pilihan: [string, string, string, string];
  pilihanEn?: [string, string, string, string];
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
        tanyaEn: 'What symbol starts a comment in EvernightLanguage programs?',
        pilihan: ['#', '//', '/* ... */', '--'],
        pilihanEn: ['#', '//', '/* ... */', '--'],
        benar: 0,
      },
      {
        tanya: 'Berkas `halo.eve` dijalankan lewat perintah apa?',
        tanyaEn: 'How do you run the file halo.eve?',
        pilihan: ['evernight halo.eve', 'eve halo.eve', 'evernight build halo.eve', 'gcc halo.eve'],
        pilihanEn: ['evernight halo.eve', 'eve halo.eve', 'evernight build halo.eve', 'gcc halo.eve'],
        benar: 0,
      },
      {
        tanya: 'Program EvernightLanguage disimpan dengan akhiran berkas apa?',
        tanyaEn: 'What file extension do EvernightLanguage programs use?',
        pilihan: ['.eve', '.ev', '.en', '.evernight'],
        pilihanEn: ['.eve', '.ev', '.en', '.evernight'],
        benar: 0,
      },
      {
        tanya: 'Apa output dari program berikut?\n`variabel pesan = "Halo"\nvariabel nama = "Dunia"\ncetak(pesan, " ", nama)`',
        tanyaEn: 'What is the output of this program?\n`variabel pesan = "Halo"\nvariabel nama = "Dunia"\ncetak(pesan, " ", nama)`',
        pilihan: ['Halo Dunia', 'HaloDunia', 'pesan nama', 'Halo " " Dunia'],
        pilihanEn: ['Halo Dunia', 'HaloDunia', 'pesan nama', 'Halo " " Dunia'],
        benar: 0,
      },
    ],
  },
  '02-variabel-tipe': {
    slug: '02-variabel-tipe',
    soal: [
      {
        tanya: 'Nilai konstanta (tidak boleh diubah) dideklarasikan dengan keyword apa?',
        tanyaEn: 'Which keyword declares a constant (unmodifiable) value?',
        pilihan: ['tetap', 'konstan', 'const', 'kunci'],
        pilihanEn: ['tetap', 'konstan', 'const', 'kunci'],
        benar: 0,
      },
      {
        tanya: 'Nilai bolean bernilai positif ditulis...',
        tanyaEn: 'The positive boolean value is written...',
        pilihan: ['benar', 'true', 'ya', 'positif'],
        pilihanEn: ['benar', 'true', 'ya', 'positif'],
        benar: 0,
      },
      {
        tanya: 'Apa output dari `cetak(angka("42") + 8)`?',
        tanyaEn: 'What does `cetak(angka("42") + 8)` output?',
        pilihan: ['50', '428', '42 + 8', '"50"'],
        pilihanEn: ['50', '428', '42 + 8', '"50"'],
        benar: 0,
      },
      {
        tanya: 'Apa yang terjadi saat menjalankan `cetak(10 + "5")`?',
        tanyaEn: 'What happens when you run `cetak(10 + "5")`?',
        pilihan: [
          'Program gagal dengan BAHAYA tipe',
          'Mencetak 105',
          'Mencetak 15',
          'Mencetak 105 sebagai teks',
        ],
        pilihanEn: [
          'The program fails with a BAHAYA type error',
          'Prints 105',
          'Prints 15',
          'Prints 105 as text',
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
        tanyaEn: 'Which keyword starts the ELSE block in EvernightLanguage branching?',
        pilihan: ['lainnya', 'else', 'kemudian', 'selanjutnya'],
        pilihanEn: ['lainnya', 'else', 'kemudian', 'selanjutnya'],
        benar: 0,
      },
      {
        tanya: 'Operator yang mensyaratkan KEDUA kondisi bernilai `benar` adalah...',
        tanyaEn: 'The operator that requires BOTH conditions to be `benar` is...',
        pilihan: ['dan', 'atau', 'bukan', 'juga'],
        pilihanEn: ['dan', 'atau', 'bukan', 'juga'],
        benar: 0,
      },
      {
        tanya: 'Apa output?\n`variabel n = 6\njika n % 2 == 0 { cetak("genap") } lainnya { cetak("ganjil") }`',
        tanyaEn: 'Output?\n`variabel n = 6\njika n % 2 == 0 { cetak("genap") } lainnya { cetak("ganjil") }`',
        pilihan: ['genap', 'ganjil', '6', 'kelipatan 2'],
        pilihanEn: ['genap', 'ganjil', '6', 'kelipatan 2'],
        benar: 0,
      },
      {
        tanya: 'Manakah struktur `cocok` yang sah?',
        tanyaEn: 'Which `cocok` structure is valid?',
        pilihan: [
          'cocok n { kasus 1 { cetak("Satu") } bawaan { cetak("Lain") } }',
          'cocok n { case 1 { cetak("Satu") } default { } }',
          'cocok n { kasus 1: cetak("Satu") }',
          'switch n { kasus 1 { cetak("Satu") } }',
        ],
        pilihanEn: [
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
        tanyaEn: '`untuk i dari 1 sampai 5 { cetak(i) }` prints how many values?',
        pilihan: [
          'Lima nilai: 1, 2, 3, 4, 5',
          'Empat nilai: 1, 2, 3, 4',
          'Enam nilai: 0, 1, 2, 3, 4, 5',
          'Satu nilai: 5',
        ],
        pilihanEn: [
          'Five values: 1, 2, 3, 4, 5',
          'Four values: 1, 2, 3, 4',
          'Six values: 0, 1, 2, 3, 4, 5',
          'One value: 5',
        ],
        benar: 0,
      },
      {
        tanya: 'Keyword untuk keluar dari perulangan secara paksa:',
        tanyaEn: 'Keyword to exit a loop early:',
        pilihan: ['berhenti', 'hentikan', 'stop', 'break'],
        pilihanEn: ['berhenti', 'hentikan', 'stop', 'break'],
        benar: 0,
      },
      {
        tanya: 'Apa fungsi `lanjut` di dalam perulangan?',
        tanyaEn: 'What does `lanjut` do inside a loop?',
        pilihan: [
          'Langsung ke iterasi berikutnya, lewati sisa blok',
          'Keluar dari perulangan',
          'Menghentikan program',
          'Mengulang dari awal perulangan',
        ],
        pilihanEn: [
          'Go straight to the next iteration, skip the rest of the block',
          'Exit the loop',
          'Stop the program',
          'Restart from the beginning of the loop',
        ],
        benar: 0,
      },
      {
        tanya: 'Manakah deklarasi perulangan `selama` yang sah?',
        tanyaEn: 'Which `selama` loop declaration is valid?',
        pilihan: [
          'selama i < 3 { i = i + 1 }',
          'while i < 3 { i = i + 1 }',
          'selama i < 3 lakukan { i = i + 1 }',
          'ulang i < 3 { i = i + 1 }',
        ],
        pilihanEn: [
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
        tanyaEn: 'Which function declaration is valid?',
        pilihan: [
          'fungsi tambah(a, b) { kembali a + b }',
          'function tambah(a, b) { return a + b }',
          'def tambah(a, b):',
          'fn tambah(a, b) { }',
        ],
        pilihanEn: [
          'fungsi tambah(a, b) { kembali a + b }',
          'function tambah(a, b) { return a + b }',
          'def tambah(a, b):',
          'fn tambah(a, b) { }',
        ],
        benar: 0,
      },
      {
        tanya: 'Keyword untuk mengembalikan nilai dari fungsi:',
        tanyaEn: 'Keyword to return a value from a function:',
        pilihan: ['kembali', 'return', 'hasil', 'keluar'],
        pilihanEn: ['kembali', 'return', 'hasil', 'keluar'],
        benar: 0,
      },
      {
        tanya: 'Apa hasil dari `faktorial(5)` jika fungsi rekursinya benar?',
        tanyaEn: 'What does `faktorial(5)` return if the recursive function is correct?',
        pilihan: ['120', '24', '720', '25'],
        pilihanEn: ['120', '24', '720', '25'],
        benar: 0,
      },
      {
        tanya: 'Fungsi rekursi wajib punya apa supaya tidak berhenti tanpa hasil?',
        tanyaEn: 'What must a recursive function have so it does not stop without a result?',
        pilihan: [
          'Basi - kondisi berhenti',
          'Perulangan `selama` di dalamnya',
          'Variabel global',
          'Minimal dua parameter',
        ],
        pilihanEn: [
          'A base case - the stopping condition',
          'A `selama` loop inside it',
          'A global variable',
          'At least two parameters',
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
        tanyaEn: 'Adding a new element to the END of list `belanja`:',
        pilihan: [
          'tambah(belanja, "minyak")',
          'tambah("minyak", belanja)',
          'sisip(belanja, "minyak")',
          'push(belanja, "minyak")',
        ],
        pilihanEn: [
          'tambah(belanja, "minyak")',
          'tambah("minyak", belanja)',
          'sisip(belanja, "minyak")',
          'push(belanja, "minyak")',
        ],
        benar: 0,
      },
      {
        tanya: 'Jumlah elemen daftar `belanja` dibaca lewat properti...',
        tanyaEn: 'The number of elements in list `belanja` is read via property...',
        pilihan: ['belanja.panjang', 'belanja.length', 'belanja.jumlah', 'belanja.ukuran'],
        pilihanEn: ['belanja.panjang', 'belanja.length', 'belanja.jumlah', 'belanja.ukuran'],
        benar: 0,
      },
      {
        tanya: 'Mengambil nilai kamus yang kuncinya mungkin tidak ada, cara paling aman:',
        tanyaEn: 'Safest way to read a dict value whose key may not exist:',
        pilihan: [
          'dapatkan(skor, "Andi", 0)',
          'ambil(skor, "Andi", 0)',
          'nilai(skor, "Andi", 0)',
          'kunci(skor, "Andi", 0)',
        ],
        pilihanEn: [
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
        tanyaEn:
          'Output?\n`variabel belanja = ["beras", "telur"]\ncetak(belanja[0], " ", belanja.panjang)`',
        pilihan: ['beras 2', 'telur 2', 'beras 3', '0 beras'],
        pilihanEn: ['beras 2', 'telur 2', 'beras 3', '0 beras'],
        benar: 0,
      },
    ],
  },
  '07-string-matematika': {
    slug: '07-string-matematika',
    soal: [
      {
        tanya: 'Apa output dari `cetak(potong("Nusantara", 0, 4))`?',
        tanyaEn: 'What does `cetak(potong("Nusantara", 0, 4))` output?',
        pilihan: ['Nusa', 'Nusan', 'Nusantara', 'usa'],
        pilihanEn: ['Nusa', 'Nusan', 'Nusantara', 'usa'],
        benar: 0,
      },
      {
        tanya: 'Apa output dari `cetak(pecah("apel,mangga,jeruk", ","))`?',
        tanyaEn: 'What does `cetak(pecah("apel,mangga,jeruk", ","))` output?',
        pilihan: ['[apel, mangga, jeruk]', 'apel,mangga,jeruk', '[apel mangga jeruk]', '3'],
        pilihanEn: ['[apel, mangga, jeruk]', 'apel,mangga,jeruk', '[apel mangga jeruk]', '3'],
        benar: 0,
      },
      {
        tanya: 'Apa hasil `pangkat(2, 10)`?',
        tanyaEn: 'What does `pangkat(2, 10)` return?',
        pilihan: ['1024', '20', '100', '210'],
        pilihanEn: ['1024', '20', '100', '210'],
        benar: 0,
      },
      {
        tanya: 'Manakah ekspresi yang menghasilkan `benar`?',
        tanyaEn: 'Which expression yields `benar`?',
        pilihan: [
          'mengandung("Evernight", "night")',
          'besar("night")',
          'potong("Evernight", 0, 4)',
          'ganti("Evernight", "night", "")',
        ],
        pilihanEn: [
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
        tanyaEn: 'Reading a number input from the user (returns `angka`):',
        pilihan: [
          'baca_angka("Umur: ")',
          'baca("Umur: ")',
          'angka_baca("Umur: ")',
          'baca_teks_angka("Umur: ")',
        ],
        pilihanEn: [
          'baca_angka("Umur: ")',
          'baca("Umur: ")',
          'angka_baca("Umur: ")',
          'baca_teks_angka("Umur: ")',
        ],
        benar: 0,
      },
      {
        tanya: 'Menurut aturan sandbox, jalur berkas mana yang benar?',
        tanyaEn: 'According to the sandbox rules, which file path is allowed?',
        pilihan: ['./catatan.txt', '/catatan.txt', 'C:\\catatan.txt', '~/catatan.txt'],
        pilihanEn: ['./catatan.txt', '/catatan.txt', 'C:\\catatan.txt', '~/catatan.txt'],
        benar: 0,
      },
      {
        tanya: 'Membaca SELURUH isi berkas teks:',
        tanyaEn: 'Reading the ENTIRE content of a text file:',
        pilihan: [
          'baca_file("./catatan.txt")',
          'buka("./catatan.txt")',
          'baca("./catatan.txt")',
          'muat_file("./catatan.txt")',
        ],
        pilihanEn: [
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
        tanyaEn:
          'Output?\n`tulis_file("./catatan.txt", "Halo")\ncetak(ada_file("./catatan.txt"))`',
        pilihan: ['benar', 'salah', 'kosong', '1'],
        pilihanEn: ['benar', 'salah', 'kosong', '1'],
        benar: 0,
      },
    ],
  },
  '09-kesalahan': {
    slug: '09-kesalahan',
    soal: [
      {
        tanya: 'Manakah blok `coba` / `tangkap` yang sah?',
        tanyaEn: 'Which `coba` / `tangkap` block is valid?',
        pilihan: [
          'coba { cetak(d[9]) } tangkap (e) { cetak(e) }',
          'coba { cetak(d[9]) } tangkap e { cetak(e) }',
          'coba { cetak(d[9]) } tangkap [e] { cetak(e) }',
          'coba { cetak(d[9]) } kecuali (e) { cetak(e) }',
        ],
        pilihanEn: [
          'coba { cetak(d[9]) } tangkap (e) { cetak(e) }',
          'coba { cetak(d[9]) } tangkap e { cetak(e) }',
          'coba { cetak(d[9]) } tangkap [e] { cetak(e) }',
          'coba { cetak(d[9]) } kecuali (e) { cetak(e) }',
        ],
        benar: 0,
      },
      {
        tanya: 'Keyword untuk memicu kesalahan sendiri:',
        tanyaEn: 'Keyword to throw your own error:',
        pilihan: [
          'lempar("data rusak!")',
          'pancing("data rusak!")',
          'gagal("data rusak!")',
          'error("data rusak!")',
        ],
        pilihanEn: [
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
        tanyaEn:
          'Without `coba`, `pastikan(1 + 1 == 3, "Hitungan rusak!")` makes the program...',
        pilihan: [
          'berhenti dengan BAHAYA [ASSERT] Hitungan rusak!',
          'mencetak PERINGATAN lalu tetap lanjut',
          'berjalan normal karena hanya cek ringan',
          'mencetak nilai salah',
        ],
        pilihanEn: [
          'stop with BAHAYA [ASSERT] Hitungan rusak!',
          'print a PERINGATAN and keep going',
          'run normally because it is only a light check',
          'print the wrong value',
        ],
        benar: 0,
      },
      {
        tanya: 'Blok `akhirnya { }` dijalankan...',
        tanyaEn: 'The `akhirnya { }` block runs...',
        pilihan: [
          'selalu, baik berhasil maupun gagal',
          'hanya jika terjadi kesalahan',
          'hanya jika tidak ada kesalahan',
          'menggantikan blok `tangkap`',
        ],
        pilihanEn: [
          'always, whether it succeeds or fails',
          'only if an error occurs',
          'only if no error occurs',
          'in place of the `tangkap` block',
        ],
        benar: 0,
      },
    ],
  },
};
