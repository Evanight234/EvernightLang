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
      { label: 'Beranda Evernight', href: '/' },
      { label: 'Pengenalan', href: '/docs/pengenalan' },
      { label: 'Instalasi', href: '/docs/instalasi' },
      { label: 'Mulai dalam 5 Menit', href: '/docs/quickstart' },
    ],
  },
  {
    key: 'tutorial',
    label: 'grp.tutorial',
    item: [
      { label: 'Tahap 1: Halo Dunia', href: '/docs/tutorial/01-halo-dunia' },
      { label: 'Tahap 2: Variabel & Tipe', href: '/docs/tutorial/02-variabel-tipe' },
      { label: 'Tahap 3: Percabangan', href: '/docs/tutorial/03-percabangan' },
      { label: 'Tahap 4: Perulangan', href: '/docs/tutorial/04-perulangan' },
      { label: 'Tahap 5: Fungsi', href: '/docs/tutorial/05-fungsi' },
      { label: 'Tahap 6: Daftar & Kamus', href: '/docs/tutorial/06-daftar-kamus' },
      { label: 'Tahap 7: String & Matematika', href: '/docs/tutorial/07-string-matematika' },
      { label: 'Tahap 8: Konsol & Berkas', href: '/docs/tutorial/08-konsol-file' },
      { label: 'Tahap 9: Penanganan Kesalahan', href: '/docs/tutorial/09-kesalahan' },
      { label: 'Tahap 10: Mini Proyek', href: '/docs/tutorial/10-mini-projek' },
    ],
  },
  {
    key: 'inti',
    label: 'grp.inti',
    item: [
      { label: 'Struktur Sintaks', href: '/docs/panduan/01-sintaks-dasar' },
      { label: 'Variabel & Tipe Data', href: '/docs/panduan/02-variabel-tipe' },
      { label: 'Fungsi', href: '/docs/panduan/03-fungsi' },
      { label: 'Percabangan', href: '/docs/panduan/04-percabangan' },
      { label: 'Perulangan', href: '/docs/panduan/05-perulangan' },
      { label: 'Penanganan Kesalahan', href: '/docs/panduan/06-kesalahan' },
      { label: 'Impor & Modul', href: '/docs/panduan/07-impor' },
    ],
  },
  {
    key: 'rujukan',
    label: 'grp.rujukan',
    item: [
      { label: 'Kata Kunci Lengkap', href: '/docs/kata-kunci' },
      { label: 'Standard Library', href: '/docs/stdlib' },
      { label: 'Referensi CLI & REPL', href: '/docs/cli' },
      { label: 'Galeri Contoh Program', href: '/docs/contoh' },
      { label: 'Tata Bahasa (EBNF)', href: '/docs/tata-bahasa' },
      { label: 'Spesifikasi Tipe Data', href: '/docs/tipe-data' },
      { label: 'Spesifikasi Fungsi', href: '/docs/fungsi' },
      { label: 'Tabel Kode Error', href: '/docs/kesalahan' },
      { label: 'Operasi Daftar', href: '/docs/daftar' },
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
