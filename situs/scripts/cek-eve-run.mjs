import { jalankan, sorot } from '../src/lib/eve-run.mjs';

let gagal = 0;
function cek(nama, kode, harap, harapGagal) {
  const r = jalankan(kode);
  const ok =
    harapGagal !== undefined
      ? r.gagal === harapGagal
      : !r.gagal && r.baris.join('\n') === harap;
  if (!ok) {
    gagal++;
    console.log('X ' + nama);
    console.log('  dapat :', JSON.stringify(r.baris), 'gagal=' + r.gagal);
    console.log('  harap :', JSON.stringify(harap));
  } else console.log('v ' + nama);
}

cek(
  't01 contoh lengkap',
  `# Program pertama EvernightLanguage
variabel pesan = "Halo Dunia dari EvernightLanguage!"
cetak(pesan)

variabel nama = "Evernight"
cetak("Selamat belajar bersama, ", nama, "!")`,
  'Halo Dunia dari EvernightLanguage!\nSelamat belajar bersama, Evernight!'
);

cek(
  't05 faktorial+fungsi',
  `fungsi faktorial(n) {
    jika n <= 1 {
        kembali 1
    } lainnya {
        kembali n * faktorial(n - 1)
    }
}
variabel angka = 5
cetak("Faktorial ", angka, " = ", faktorial(angka))
cetak("Faktorial 0 = ", faktorial(0))`,
  'Faktorial 5 = 120\nFaktorial 0 = 1'
);

cek(
  't04 untuk dari-sampai + jumlah',
  `cetak("Deret 1 sampai 5:")
untuk i dari 1 sampai 5 {
    cetak(i, " ")
}
cetak("")
variabel jumlah = 0
untuk i dari 1 sampai 5 {
    jumlah = jumlah + i
}
cetak("Jumlah 1..5 = ", jumlah)`,
  'Deret 1 sampai 5:\n1 \n2 \n3 \n4 \n5 \n\nJumlah 1..5 = 15'
);

cek(
  't04 untuk dalam + selama + lanjut/berhenti',
  `variabel langkah = 0
variabel total = 0
selama langkah < 20 {
    langkah = langkah + 1
    jika langkah % 2 != 0 {
        lanjut
    }
    jika langkah > 12 {
        berhenti
    }
    total = total + langkah
}
cetak("Jumlah bilangan genap sampai 12 = ", total)
variabel buah = ["apel", "mangga"]
untuk b dalam buah {
    cetak("Buah: ", b)
}`,
  'Jumlah bilangan genap sampai 12 = 42\nBuah: apel\nBuah: mangga'
);

cek(
  't03 percabangan + bukan + atau',
  `variabel n = 6
jika n % 2 == 0 {
    cetak(n, " adalah bilangan genap")
} lainnya {
    cetak(n, " adalah bilangan ganjil")
}
jika bukan (n > 100) {
    cetak("Nilainya di bawah 100 (bukan)")
}
jika n == 0 atau n == 6 {
    cetak("nol atau enam")
}`,
  '6 adalah bilangan genap\nNilainya di bawah 100 (bukan)\nnol atau enam'
);

cek(
  't06 daftar+kamus',
  `variabel belanja = ["beras", "telur", "gula"]
tambah(belanja, "minyak")
cetak("Belanja: ", belanja, " -> ", belanja.panjang, " item")
variabel skor = { "Andi": 90, "Budi": 85 }
setel(skor, "Dewi", 88)
hapus_kunci(skor, "Budi")
cetak("Skor Andi: ", dapatkan(skor, "Andi", 0))
cetak("Skor Ani (bawaan 0): ", dapatkan(skor, "Ani", 0))
cetak("Ada kunci Andi: ", ada_kunci(skor, "Andi"))`,
  'Belanja: [beras, telur, gula, minyak] -> 4 item\nSkor Andi: 90\nSkor Ani (bawaan 0): 0\nAda kunci Andi: benar'
);

cek(
  't07 string+matematika',
  `cetak("besar  : ", besar("  Evernight  "))
cetak("potong : ", potong("Nusantara", 0, 4))
cetak("akar(144) = ", akar(144))
cetak("2 * pi = ", 2 * pi)`,
  'besar  :   EVERNIGHT  \npotong : Nusa\nakar(144) = 12\n2 * pi = 6.283185307179586'
);

cek(
  't09 coba/tangkap/lempar/pastikan',
  `coba {
    pastikan(1 + 1 == 3, "Hitungan rusak!")
} tangkap (e) {
    cetak("Tertangkap: ", e)
}
coba {
    lempar("Usia tidak boleh negatif!")
} tangkap (e) {
    cetak("Error: ", e)
}
coba {
    variabel d = [1, 2]
    cetak(d[9])
} tangkap (e) {
    cetak("Error: ", e)
} akhirnya {
    cetak("Blok akhirnya dijalankan")
}
cetak("Semua aman!")`,
  'Tertangkap: Hitungan rusak!\nError: Usia tidak boleh negatif!\nkosong\nBlok akhirnya dijalankan\nSemua aman!'
);

cek(
  't08 fake file',
  `tulis_file("./catatan.txt", "Isi berkas")
cetak(baca_file("./catatan.txt"))
cetak(ada_file("./catatan.txt"))
coba {
    baca_file("./tidak_ada.txt")
} tangkap (e) {
    cetak("Gagal membaca: ", e)
}`,
  'Isi berkas\nbenar\nGagal membaca: Gagal membaca berkas \'./tidak_ada.txt\': tidak ditemukan (os error 2)'
);

cek(
  't05 fungsi anonim',
  `variabel ganda = fungsi(x) {
    kembali x * 2
}
cetak("Dua kali 21 = ", ganda(21))`,
  'Dua kali 21 = 42'
);

cek(
  't02 teks/angka + hubung + tetap',
  `tetap NEGARA = "Indonesia"
cetak("Tahun: " + teks(1945))
cetak("Teks jadi angka: ", angka("42") + 8)
cetak("Negara: ", NEGARA, " (panjang teks: ", NEGARA.panjang, ")")`,
  'Tahun: 1945\nTeks jadi angka: 50\nNegara: Indonesia (panjang teks: 9)'
);

cek('error sintaks -> BAHAYA', 'cetak("halo', undefined, true);
cek('pastikan gagal top-level -> BAHAYA', 'pastikan(1 + 1 == 3, "Hitungan rusak!")', undefined, true);
cek('loop tak berhenti -> guard', 'selama benar {\n}', undefined, true);
cek('kode kosong -> tidak gagal', '', undefined, false);

const s1 = sorot('fungsi a() {');
const s2 = sorot('cetak("x") # komentar');
if (!(s1.includes('tok-k') && s2.includes('tok-s') && s2.includes('tok-c'))) {
  gagal++;
  console.log('X sorot ada kelas');
} else console.log('v sorot ada kelas');

console.log(gagal === 0 ? 'SEMUA LULUS' : gagal + ' GAGAL');
process.exit(gagal === 0 ? 0 : 1);
