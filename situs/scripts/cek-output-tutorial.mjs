// Bandingkan blok "Output yang diharapkan" di docs/tutorial dengan evernight.exe asli.
import { readFileSync, writeFileSync, mkdtempSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { execFileSync } from 'node:child_process';

const EXE = 'E:/EvernightLang/paket/evernight-0.1.0-windows-x64/bin/evernight.exe';
const DIR = 'E:/EvernightLang/docs/tutorial';

// input tiap program interaktif (dari teks tutorial)
const INPUT = {
  '02': 'Budi\n20\n',
  '03': '6\n',
  '08': 'Budi\n20\n',
  '10': '1\n10\n4\n3\n6\n7\n4\n10\n0\n5\n9\n6\n',
};
const LEWAT = new Set(['10']); // output di MD sengaja "potongan"

function blok(src, setelah, lang) {
  let s = src;
  const i = s.indexOf(setelah);
  if (i >= 0) s = s.slice(i);
  const re = /```([a-zA-Z]*)\n([\s\S]*?)```/g;
  let m;
  while ((m = re.exec(s))) {
    if (lang === undefined || m[1].toLowerCase() === lang) return m[2].replace(/\n+$/, '');
  }
  return null;
}

const dir = mkdtempSync(join(tmpdir(), 'eve-cek-'));
let beda = 0;
const files = ['01', '02', '03', '04', '05', '06', '07', '08', '09', '10'];
for (const n of files) {
  const f = `${DIR}/${n}-${{ '01': 'halo-dunia', '02': 'variabel-tipe', '03': 'percabangan', '04': 'perulangan', '05': 'fungsi', '06': 'daftar-kamus', '07': 'string-matematika', '08': 'konsol-file', '09': 'kesalahan', '10': 'mini-projek' }[n]}.md`;
  const body = readFileSync(f, 'utf8');
  const kode = blok(body, '## Contoh lengkap', 'eve');
  const harap = blok(body, 'Output yang diharapkan', '');
  if (!kode || !harap || LEWAT.has(n)) { console.log(`- ${n}: dilewati`); continue; }

  const ef = join(dir, `t${n}.eve`);
  writeFileSync(ef, kode, 'utf8');
  let dapat = '';
  try {
    dapat = execFileSync(EXE, [ef], { input: INPUT[n] ?? '', encoding: 'utf8', cwd: dir });
  } catch (e) {
    dapat = (e.stdout ?? '') + (e.stderr ?? '') + `\n[exit ${e.status}]`;
  }
  const norm = (t) =>
    t
      .replace(/\r\n/g, '\n')
      .split('\n')
      .map((l) => {
        // urutan kunci kamus saat dicetak tidak menentu -> urutkan isinya
        if (l.startsWith('Semua skor: {')) return l.replace(/\{(.*)\}/, (_m, isi) => '{' + isi.split(', ').sort().join(', ') + '}');
        return l.replace(/\s+$/, '');
      })
      .join('\n')
      .trim();
  // 08: pesan sistem OS-skipif beda -> bandingkan tanpa baris itu
  const khusus = harap.includes('<pesan sistem>');
  const ambil = (t) =>
    norm(t)
      .split('\n')
      .filter((l) => !(khusus && l.includes('<pesan sistem>')) && !(khusus && l.startsWith('Gagal membaca:')))
      .join('\n');
  if (ambil(dapat) === ambil(harap)) console.log(`v ${n}: cocok`);
  else {
    beda++;
    console.log(`X ${n}: BEDA`);
    console.log('--- exe ---\n' + norm(dapat));
    console.log('--- md ---\n' + norm(harap));
    console.log('-----------');
  }
}
rmSync(dir, { recursive: true, force: true });
console.log(beda === 0 ? 'SEMUA COCOK' : `${beda} beda`);
process.exit(beda === 0 ? 0 : 1);
