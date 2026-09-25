import { dirname, posix, resolve } from 'node:path';

const peta = {
  keyword: 'kata-kunci',
  tipe: 'tipe-data',
  function: 'fungsi',
  error: 'kesalahan',
  stdlib: 'stdlib',
  grammar: 'tata-bahasa',
  array: 'daftar',
};

// Ubah link relatif *.md menjadi URL docs (/docs/slug/) agar tidak 404 di situs statis.
export default function remarkMdLink() {
  return (tree, file) => {
    const abs = String(file.path || '').replace(/\\/g, '/');
    const repo = resolve(process.cwd(), '..').replace(/\\/g, '/');
    const relDir =
      abs.toLowerCase().startsWith(repo.toLowerCase()) && abs.length > repo.length
        ? dirname(abs.slice(repo.length + 1)) || '.'
        : null;
    if (relDir === null) return;

    const ubah = (n) => {
      if (n.type !== 'link' && n.type !== 'definition') return;
      const u = n.url;
      if (typeof u !== 'string' || u.startsWith('http') || u.startsWith('mailto:') || u.startsWith('#')) return;
      const potong = u.split('#');
      const p = potong[0];
      if (!p.endsWith('.md')) return;
      const hash = potong.length > 1 ? '#' + potong.slice(1).join('#') : '';
      const target = posix.normalize(posix.join(relDir, p));
      let slug;
      if (target.startsWith('docs/')) slug = target.slice(5).replace(/\.md$/i, '');
      else slug = peta[posix.basename(target).toLowerCase().replace(/\.md$/i, '')];
      if (!slug) return;
      n.url = `/docs/${slug}/${hash}`;
    };

    const jalan = (n) => {
      ubah(n);
      if (Array.isArray(n.children)) n.children.forEach(jalan);
    };
    jalan(tree);
  };
}
