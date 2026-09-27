// Peta slug + struktur sidebar dokumentasi (WEB.md §4, disesuaikan konten docs/).

const petaSlug: Record<string, string> = {
  keyword: 'kata-kunci',
  tipe: 'tipe-data',
  function: 'fungsi',
  error: 'kesalahan',
  stdlib: 'stdlib',
  grammar: 'tata-bahasa',
  array: 'daftar',
};

export function slugDari(id: string): string {
  const idb = id.toLowerCase();
  if (idb.startsWith('docs/')) return idb.slice(5);
  return petaSlug[idb] ?? idb;
}

export function judulDariBody(body: string, id: string): string {
  const m = body.match(/^#\s+(.+)$/m);
  if (!m) return id;
  return m[1].trim().replace(/^\S+\.md\s*[-–—]\s*/, '');
}

export interface ItemSidebar {
  label: string;
  labelEn: string;
  href: string;
}
export interface GrupSidebar {
  key: string;
  label: string;
  item: ItemSidebar[];
}

export const sidebar: GrupSidebar[] = [
  {
    key: 'mulai',
    label: 'grp.mulai',
    item: [
      { label: 'Beranda Evernight', labelEn: 'Evernight Home', href: '/' },
      { label: 'Pengenalan', labelEn: 'Introduction', href: '/docs/pengenalan' },
      { label: 'Instalasi', labelEn: 'Installation', href: '/docs/instalasi' },
      { label: 'Mulai dalam 5 Menit', labelEn: 'Get Started in 5 Minutes', href: '/docs/quickstart' },
    ],
  },
  {
    key: 'tutorial',
    label: 'grp.tutorial',
    item: [
      { label: 'Tahap 1: Halo Dunia', labelEn: 'Stage 1: Hello World', href: '/docs/tutorial/01-halo-dunia' },
      { label: 'Tahap 2: Variabel & Tipe', labelEn: 'Stage 2: Variables & Types', href: '/docs/tutorial/02-variabel-tipe' },
      { label: 'Tahap 3: Percabangan', labelEn: 'Stage 3: Branching', href: '/docs/tutorial/03-percabangan' },
      { label: 'Tahap 4: Perulangan', labelEn: 'Stage 4: Loops', href: '/docs/tutorial/04-perulangan' },
      { label: 'Tahap 5: Fungsi', labelEn: 'Stage 5: Functions', href: '/docs/tutorial/05-fungsi' },
      { label: 'Tahap 6: Daftar & Kamus', labelEn: 'Stage 6: Lists & Dicts', href: '/docs/tutorial/06-daftar-kamus' },
      { label: 'Tahap 7: String & Matematika', labelEn: 'Stage 7: Strings & Math', href: '/docs/tutorial/07-string-matematika' },
      { label: 'Tahap 8: Konsol & Berkas', labelEn: 'Stage 8: Console & Files', href: '/docs/tutorial/08-konsol-file' },
      { label: 'Tahap 9: Penanganan Kesalahan', labelEn: 'Stage 9: Error Handling', href: '/docs/tutorial/09-kesalahan' },
    ],
  },
  {
    key: 'inti',
    label: 'grp.inti',
    item: [
      { label: 'Struktur Sintaks', labelEn: 'Syntax Structure', href: '/docs/panduan/01-sintaks-dasar' },
      { label: 'Variabel & Tipe Data', labelEn: 'Variables & Data Types', href: '/docs/panduan/02-variabel-tipe' },
      { label: 'Fungsi', labelEn: 'Functions', href: '/docs/panduan/03-fungsi' },
      { label: 'Percabangan', labelEn: 'Branching', href: '/docs/panduan/04-percabangan' },
      { label: 'Perulangan', labelEn: 'Loops', href: '/docs/panduan/05-perulangan' },
      { label: 'Penanganan Kesalahan', labelEn: 'Error Handling', href: '/docs/panduan/06-kesalahan' },
      { label: 'Impor & Modul', labelEn: 'Imports & Modules', href: '/docs/panduan/07-impor' },
    ],
  },
  {
    key: 'rujukan',
    label: 'grp.rujukan',
    item: [
      { label: 'Kata Kunci Lengkap', labelEn: 'All Keywords', href: '/docs/kata-kunci' },
      { label: 'Standard Library', labelEn: 'Standard Library', href: '/docs/stdlib' },
      { label: 'Referensi CLI & REPL', labelEn: 'CLI & REPL Reference', href: '/docs/cli' },
      { label: 'Galeri Contoh Program', labelEn: 'Example Programs', href: '/docs/contoh' },
      { label: 'Tata Bahasa (EBNF)', labelEn: 'Grammar (EBNF)', href: '/docs/tata-bahasa' },
      { label: 'Spesifikasi Tipe Data', labelEn: 'Data Type Spec', href: '/docs/tipe-data' },
      { label: 'Spesifikasi Fungsi', labelEn: 'Function Spec', href: '/docs/fungsi' },
      { label: 'Tabel Kode Error', labelEn: 'Error Code Table', href: '/docs/kesalahan' },
      { label: 'Operasi Daftar', labelEn: 'List Operations', href: '/docs/daftar' },
    ],
  },
];

// Urutan datar (hanya halaman /docs/) untuk tombol ‹ prev / next ›
export const rataSidebar: ItemSidebar[] = sidebar.flatMap((g) => g.item).filter((i) => i.href.startsWith('/docs/'));

export function tetangga(slug: string): { sebelum?: ItemSidebar; sesudah?: ItemSidebar } {
  const i = rataSidebar.findIndex((x) => x.href === `/docs/${slug}`);
  if (i < 0) return {};
  return { sebelum: rataSidebar[i - 1], sesudah: rataSidebar[i + 1] };
}
