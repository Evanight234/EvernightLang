// Penerbit highlight + mini-interpreter Evernight untuk demo browser.
// Dipakai SSR (highlight awal), bundle klien (input + run), dan smoke test node.

const KATA = [
  'lainnya_jika', 'fungsi', 'kembali', 'variabel', 'selama', 'untuk', 'dari', 'sampai',
  'dalam', 'coba', 'tangkap', 'lempar', 'pastikan', 'impor', 'sebagai', 'cocok', 'kasus',
  'berhenti', 'lanjut', 'jika', 'lainnya', 'dan', 'atau', 'bukan', 'benar', 'salah',
  'kosong', 'kelas', 'guna', 'tetap',
];

function esk(t) {
  return t.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
}

function sorotBaris(baris) {
  const esc = esk(baris);
  if (esc.trimStart().startsWith('#')) return `<span class="tok-c">${esc}</span>`;

  // string dan komentar ujung baris -> placeholder (dibungkus, di restore terakhir)
  const bagian = [];
  let out = esc.replace(/"[^"]*"/g, (m) => {
    bagian.push(`<span class="tok-s">${m}</span>`);
    return '\u0000' + 'x'.repeat(bagian.length) + '\u0000';
  });
  out = out.replace(/#.*$/, (m) => {
    bagian.push(`<span class="tok-c">${m}</span>`);
    return '\u0000' + 'x'.repeat(bagian.length) + '\u0000';
  });
  out = out.replace(/(^|[^\s(,])(\d+(?:\.\d+)?)(?![\w.])/g, (_m, p, n) => `${p}<span class="tok-n">${n}</span>`);
  out = out.replace(new RegExp(`\\b(${KATA.join('|')})\\b`, 'g'), '<span class="tok-k">$1</span>');
  out = out.replace(/([A-Za-z_]\w*)\s*\(/g, '<span class="tok-f">$1</span>(');
  out = out.replace(/\u0000(x+)\u0000/g, (_m, x) => bagian[x.length - 1]);
  return out;
}

export function sorot(teks) {
  return teks.split('\n').map(sorotBaris).join('\n');
}

// ---------- mini-interpreter ----------

function fmt(v) {
  if (v === null || v === undefined) return 'kosong';
  if (v === true) return 'benar';
  if (v === false) return 'salah';
  if (v instanceof Error) return v.message;
  if (Array.isArray(v)) return '[' + v.map(fmt).join(', ') + ']';
  if (typeof v === 'object') {
    return '{' + Object.entries(v).map(([k, val]) => `${k}: ${fmt(val)}`).join(', ') + '}';
  }
  return String(v);
}

const LOOP_MAKS = 2_000_000;

export function terjemah(src) {
  // 1. keluarkan string -> placeholder (lindungi dari semua penggantian)
  const strings = [];
  let s = src.replace(/"(?:[^"\\]|\\.)*"|'(?:[^'\\]|\\.)*'/g, (m) => {
    strings.push(m);
    return '\u0001' + strings.length + '\u0001';
  });

  // 2. buang komentar #
  s = s.replace(/#[^\n]*/g, '');

  // 3. buang impor (modul dianggap tersedia global di demo)
  s = s.replace(/^\s*impor\s+\w+\s*$/gm, '');

  // 4. perulangan
  s = s.replace(
    /untuk\s+([A-Za-z_]\w*)\s+dari\s+([\s\S]+?)\s+sampai\s+([\s\S]+?)\s*\{/g,
    (_m, v, a, b) => `for (let ${v} = ${a}; __tk() && ${v} <= ${b}; ${v}++) {`
  );
  s = s.replace(
    /untuk\s+([A-Za-z_]\w*)\s+dalam\s+([\s\S]+?)\s*\{/g,
    (_m, v, k) => `for (const ${v} of ${k}) {`
  );

  // 5. blok percabangan (kondisi dibungkus kurung)
  s = s.replace(/lainnya_jika\s+([\s\S]+?)\s*\{/g, 'else if ($1) {');
  s = s.replace(/\bjika\s+([\s\S]+?)\s*\{/g, 'if ($1) {');
  s = s.replace(/\bselama\s+([\s\S]+?)\s*\{/g, 'while (__tk() && ($1)) {');

  // 6. kata kunci & operator berbasis kata
  const kata = [
    ['lainnya_jika', 'else if'], ['variabel', 'let'], ['tetap', 'const'],
    ['fungsi', 'function'], ['kembali', 'return'], ['berhenti', 'break'],
    ['lanjut', 'continue'], ['coba', 'try'], ['tangkap', 'catch'], ['akhirnya', 'finally'],
    ['lebih_dari_sama_dengan', '>='], ['kurang_dari_sama_dengan', '<='],
    ['sama_dengan', '==='], ['lebih_dari', '>'], ['kurang_dari', '<'],
    ['dan', '&&'], ['atau', '||'], ['bukan', '!'],
    ['berhenti', 'break'], ['lanjut', 'continue'],
    ['jika', 'if'], ['selama', 'while'], ['lainnya', 'else'],
  ];
  for (const [dari, ke] of kata) s = s.replace(new RegExp(`\\b${dari}\\b`, 'g'), ke);

  // 7. properti (pi/e lewat objek H, supaya tidak menabrak identitas seperti tangkap (e))
  s = s.replace(/\.panjang\b/g, '.length');

  // 8. kembalikan string
  s = s.replace(/\u0001(\d+)\u0001/g, (_m, i) => strings[Number(i) - 1]);
  return s;
}

export function jalankan(src) {
  const baris = [];
  const fs = new Map();
  let langkah = 0;
  const __tk = () => {
    if (++langkah > LOOP_MAKS) throw new Error('perulangan terlalu lama, program dihentikan');
    return true;
  };

  const __cetak = (...args) => baris.push(args.map(fmt).join(''));
  const __baca = (pertanyaan = '') => {
    const j = globalThis.prompt(String(pertanyaan)) ?? '';
    baris.push(String(pertanyaan) + j);
    return j;
  };
  const __baca_angka = (pertanyaan = '') => {
    const j = __baca(pertanyaan);
    const n = Number(j.trim());
    if (j.trim() !== '' && Number.isNaN(n)) throw new Error(`'${j}' bukan angka yang valid`);
    return n;
  };

  const H = {
    cetak: __cetak, baca: __baca, baca_angka: __baca_angka, __tk,
    benar: true, salah: false, kosong: null,
    pi: Math.PI, e: Math.E,
    teks: (v) => fmt(v === undefined ? null : v),
    angka: (v) => (typeof v === 'number' ? v : Number(String(v).trim())),
    adalah_angka: (v) => typeof v === 'number' && !Number.isNaN(v),
    besar: (s) => String(s).toUpperCase(),
    kecil: (s) => String(s).toLowerCase(),
    bersih: (s) => String(s).trim(),
    potong: (s, a, b) => String(s).slice(a, b),
    pecah: (s, sep) => String(s).split(sep),
    gabung: (d, sep) => d.join(sep),
    ganti: (s, lama, baru) => String(s).split(lama).join(baru),
    mengandung: (s, t) => String(s).includes(t),
    ulang_teks: (s, n) => String(s).repeat(n),
    format: (pola, ...a) => String(pola).replace(/\{(\d+)\}/g, (_m, i) => fmt(a[Number(i)])),
    akar: Math.sqrt, pangkat: (a, b) => a ** b, mutlak: Math.abs,
    pembulatan: Math.round, bulat_bawah: Math.floor, bulat_atas: Math.ceil,
    min: Math.min, max: Math.max,
    faktorial: (n) => { let r = 1; for (let i = 2; i <= n; i++) r *= i; return r; },
    sin: Math.sin, cos: Math.cos, tan: Math.tan,
    acak_antara: (a, b) => a + Math.floor(Math.random() * (b - a + 1)),
    tambah: (d, x) => { d.push(x); return d; },
    hapus: (d, x) => { const i = d.indexOf(x); if (i >= 0) d.splice(i, 1); return d; },
    urutkan: (d) => (d.every((x) => typeof x === 'number') ? [...d].sort((a, b) => a - b) : [...d].sort()),
    balik: (d) => [...d].reverse(),
    gabung_larik: (a, b) => [...a, ...b],
    jumlah: (d) => d.reduce((a, b) => a + b, 0),
    rata_rata: (d) => d.reduce((a, b) => a + b, 0) / d.length,
    ada: (d, x) => d.includes(x),
    dapatkan: (k, key, bawaan) => (k[key] ?? bawaan),
    setel: (k, key, nilai) => { k[key] = nilai; return k; },
    hapus_kunci: (k, key) => { delete k[key]; return k; },
    kunci: (k) => Object.keys(k),
    nilai: (k) => Object.values(k),
    ada_kunci: (k, key) => Object.prototype.hasOwnProperty.call(k, key),
    tulis_file: (jalur, isi) => { fs.set(String(jalur), String(isi)); },
    baca_file: (jalur) => {
      if (!fs.has(String(jalur))) throw new Error(`Gagal membaca berkas '${jalur}': tidak ditemukan (os error 2)`);
      return fs.get(String(jalur));
    },
    ada_file: (jalur) => fs.has(String(jalur)),
    waktu_sekarang: () => Date.now(),
    tunda: () => {},
    bersihkan: () => {},
    lempar: (pesan) => { throw new Error(String(pesan)); },
    pastikan: (kondisi, pesan) => { if (!kondisi) throw new Error(String(pesan)); },
  };

  if (!src.trim()) return { baris, gagal: false };

  try {
    const badan = terjemah(src);
    const fn = new Function('H', `with (H) {\n${badan}\n}`);
    fn(H);
    return { baris, gagal: false };
  } catch (e) {
    baris.push(`BAHAYA: ${e && e.message ? e.message : String(e)}`);
    return { baris, gagal: true };
  }
}
