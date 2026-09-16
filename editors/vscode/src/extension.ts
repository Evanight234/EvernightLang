import * as vscode from "vscode";

// ── Daftar item autocomplete statis ─────────────────────────────────────────

interface CompletionItem {
  label: string;
  kind: vscode.CompletionItemKind;
  detail: string;
  documentation: string;
  insertText?: string;
}

const KEYWORDS: CompletionItem[] = [
  // Deklarasi
  { label: "fungsi",      kind: vscode.CompletionItemKind.Keyword, detail: "keyword", documentation: "Deklarasi fungsi.\n\n`fungsi nama(param) { }`" },
  { label: "variabel",    kind: vscode.CompletionItemKind.Keyword, detail: "keyword", documentation: "Deklarasi variabel.\n\n`variabel x = 0`" },
  { label: "tetap",       kind: vscode.CompletionItemKind.Keyword, detail: "keyword", documentation: "Deklarasi konstanta.\n\n`tetap PI = 3.14`" },
  { label: "kembali",     kind: vscode.CompletionItemKind.Keyword, detail: "keyword", documentation: "Kembalikan nilai dari fungsi.\n\n`kembali nilai`" },
  // Percabangan
  { label: "jika",        kind: vscode.CompletionItemKind.Keyword, detail: "keyword", documentation: "Percabangan kondisional.\n\n`jika kondisi { }`" },
  { label: "lainnya_jika",kind: vscode.CompletionItemKind.Keyword, detail: "keyword", documentation: "Cabang tambahan.\n\n`lainnya_jika kondisi { }`" },
  { label: "lainnya",     kind: vscode.CompletionItemKind.Keyword, detail: "keyword", documentation: "Cabang default.\n\n`lainnya { }`" },
  { label: "cocok",       kind: vscode.CompletionItemKind.Keyword, detail: "keyword", documentation: "Pattern matching.\n\n`cocok nilai { kasus x { } }`" },
  { label: "kasus",       kind: vscode.CompletionItemKind.Keyword, detail: "keyword", documentation: "Pola dalam `cocok`." },
  { label: "bawaan",      kind: vscode.CompletionItemKind.Keyword, detail: "keyword", documentation: "Pola default dalam `cocok`." },
  // Perulangan
  { label: "selama",      kind: vscode.CompletionItemKind.Keyword, detail: "keyword", documentation: "Perulangan while.\n\n`selama kondisi { }`" },
  { label: "untuk",       kind: vscode.CompletionItemKind.Keyword, detail: "keyword", documentation: "Perulangan for.\n\n`untuk i dari 0 sampai 10 { }`" },
  { label: "dari",        kind: vscode.CompletionItemKind.Keyword, detail: "keyword", documentation: "Batas awal range `untuk`." },
  { label: "sampai",      kind: vscode.CompletionItemKind.Keyword, detail: "keyword", documentation: "Batas akhir range `untuk`." },
  { label: "dalam",       kind: vscode.CompletionItemKind.Keyword, detail: "keyword", documentation: "Iterasi koleksi.\n\n`untuk x dalam daftar { }`" },
  { label: "berhenti",    kind: vscode.CompletionItemKind.Keyword, detail: "keyword", documentation: "Hentikan perulangan (break)." },
  { label: "lanjut",      kind: vscode.CompletionItemKind.Keyword, detail: "keyword", documentation: "Lanjut iterasi berikutnya (continue)." },
  // Error handling
  { label: "coba",        kind: vscode.CompletionItemKind.Keyword, detail: "keyword", documentation: "Blok try.\n\n`coba { } tangkap (e) { }`" },
  { label: "tangkap",     kind: vscode.CompletionItemKind.Keyword, detail: "keyword", documentation: "Blok catch.\n\n`tangkap (e) { }`" },
  { label: "akhirnya",    kind: vscode.CompletionItemKind.Keyword, detail: "keyword", documentation: "Blok finally." },
  { label: "lempar",      kind: vscode.CompletionItemKind.Keyword, detail: "keyword", documentation: "Lempar error.\n\n`lempar(\"pesan\")`" },
  { label: "pastikan",    kind: vscode.CompletionItemKind.Keyword, detail: "keyword", documentation: "Assert kondisi.\n\n`pastikan(kondisi, \"pesan\")`" },
  // Modul
  { label: "impor",       kind: vscode.CompletionItemKind.Keyword, detail: "keyword", documentation: "Impor modul.\n\n`impor \"modul\"`" },
  { label: "sebagai",     kind: vscode.CompletionItemKind.Keyword, detail: "keyword", documentation: "Alias impor.\n\n`impor \"modul\" sebagai alias`" },
  { label: "hapus",       kind: vscode.CompletionItemKind.Keyword, detail: "keyword", documentation: "Hapus elemen.\n\n`hapus daftar[i]`" },
  // Operator kata
  { label: "dan",         kind: vscode.CompletionItemKind.Keyword, detail: "operator", documentation: "Operator logika AND." },
  { label: "atau",        kind: vscode.CompletionItemKind.Keyword, detail: "operator", documentation: "Operator logika OR." },
  { label: "bukan",       kind: vscode.CompletionItemKind.Keyword, detail: "operator", documentation: "Operator logika NOT." },
  // Literal
  { label: "benar",       kind: vscode.CompletionItemKind.Value, detail: "boolean", documentation: "Nilai boolean true." },
  { label: "salah",       kind: vscode.CompletionItemKind.Value, detail: "boolean", documentation: "Nilai boolean false." },
  { label: "kosong",      kind: vscode.CompletionItemKind.Value, detail: "null", documentation: "Nilai null/kosong." },
];

const BUILTINS: CompletionItem[] = [
  // Konsol (global)
  { label: "cetak",        kind: vscode.CompletionItemKind.Function, detail: "builtin", documentation: "Cetak ke konsol.\n\n`cetak(nilai, ...)`", insertText: "cetak(${1})" },
  { label: "baca",         kind: vscode.CompletionItemKind.Function, detail: "builtin", documentation: "Baca input dari pengguna.\n\n`baca(pesan?)`", insertText: "baca(${1})" },
  { label: "baca_angka",   kind: vscode.CompletionItemKind.Function, detail: "builtin", documentation: "Baca input angka.\n\n`baca_angka(pesan?)`", insertText: "baca_angka(${1})" },
  { label: "bersihkan",    kind: vscode.CompletionItemKind.Function, detail: "builtin", documentation: "Bersihkan layar konsol.", insertText: "bersihkan()" },
  { label: "argumen",      kind: vscode.CompletionItemKind.Function, detail: "builtin", documentation: "Ambil argumen CLI program.\n\n`argumen()`", insertText: "argumen()" },
  // String
  { label: "besar",        kind: vscode.CompletionItemKind.Function, detail: "string", documentation: "Ubah teks ke huruf besar.", insertText: "besar(${1:teks})" },
  { label: "kecil",        kind: vscode.CompletionItemKind.Function, detail: "string", documentation: "Ubah teks ke huruf kecil.", insertText: "kecil(${1:teks})" },
  { label: "bersih",       kind: vscode.CompletionItemKind.Function, detail: "string", documentation: "Hapus spasi di ujung teks.", insertText: "bersih(${1:teks})" },
  { label: "potong",       kind: vscode.CompletionItemKind.Function, detail: "string", documentation: "Ambil substring.", insertText: "potong(${1:teks}, ${2:awal}, ${3:akhir})" },
  { label: "pecah",        kind: vscode.CompletionItemKind.Function, detail: "string", documentation: "Pecah teks jadi daftar.", insertText: "pecah(${1:teks}, ${2:pemisah})" },
  { label: "gabung",       kind: vscode.CompletionItemKind.Function, detail: "string", documentation: "Gabung daftar jadi teks.", insertText: "gabung(${1:daftar}, ${2:pemisah})" },
  { label: "ganti",        kind: vscode.CompletionItemKind.Function, detail: "string", documentation: "Ganti substring.", insertText: "ganti(${1:teks}, ${2:dari}, ${3:ke})" },
  { label: "mengandung",   kind: vscode.CompletionItemKind.Function, detail: "string", documentation: "Cek apakah teks mengandung substring.", insertText: "mengandung(${1:teks}, ${2:sub})" },
  { label: "format",       kind: vscode.CompletionItemKind.Function, detail: "string", documentation: "Format teks dengan placeholder `{}`.", insertText: "format(${1:pola}, ${2:nilai})" },
  // Matematika
  { label: "akar",         kind: vscode.CompletionItemKind.Function, detail: "matematika", documentation: "Akar kuadrat.", insertText: "akar(${1:n})" },
  { label: "pangkat",      kind: vscode.CompletionItemKind.Function, detail: "matematika", documentation: "Pangkat.", insertText: "pangkat(${1:basis}, ${2:eksponen})" },
  { label: "mutlak",       kind: vscode.CompletionItemKind.Function, detail: "matematika", documentation: "Nilai mutlak (absolut).", insertText: "mutlak(${1:n})" },
  { label: "acak_antara",  kind: vscode.CompletionItemKind.Function, detail: "matematika", documentation: "Angka acak antara min dan max.", insertText: "acak_antara(${1:min}, ${2:max})" },
  { label: "bulat_bawah",  kind: vscode.CompletionItemKind.Function, detail: "matematika", documentation: "Pembulatan ke bawah (floor).", insertText: "bulat_bawah(${1:n})" },
  { label: "bulat_atas",   kind: vscode.CompletionItemKind.Function, detail: "matematika", documentation: "Pembulatan ke atas (ceil).", insertText: "bulat_atas(${1:n})" },
  { label: "pembulatan",   kind: vscode.CompletionItemKind.Function, detail: "matematika", documentation: "Pembulatan biasa (round).", insertText: "pembulatan(${1:n})" },
  // Daftar
  { label: "tambah",       kind: vscode.CompletionItemKind.Function, detail: "daftar", documentation: "Tambah elemen ke daftar.", insertText: "tambah(${1:daftar}, ${2:nilai})" },
  { label: "sisip",        kind: vscode.CompletionItemKind.Function, detail: "daftar", documentation: "Sisip elemen di posisi.", insertText: "sisip(${1:daftar}, ${2:indeks}, ${3:nilai})" },
  { label: "urutkan",      kind: vscode.CompletionItemKind.Function, detail: "daftar", documentation: "Urutkan daftar (in-place).", insertText: "urutkan(${1:daftar})" },
  { label: "balik",        kind: vscode.CompletionItemKind.Function, detail: "daftar", documentation: "Balik urutan daftar.", insertText: "balik(${1:daftar})" },
  { label: "panjang",      kind: vscode.CompletionItemKind.Property,  detail: "properti", documentation: "Panjang daftar/teks/kamus." },
  { label: "peta",         kind: vscode.CompletionItemKind.Function, detail: "daftar", documentation: "Map setiap elemen.", insertText: "peta(${1:daftar}, ${2:fungsi})" },
  { label: "saring",       kind: vscode.CompletionItemKind.Function, detail: "daftar", documentation: "Filter elemen.", insertText: "saring(${1:daftar}, ${2:fungsi})" },
  { label: "lipat",        kind: vscode.CompletionItemKind.Function, detail: "daftar", documentation: "Reduce daftar.", insertText: "lipat(${1:daftar}, ${2:fungsi}, ${3:awal})" },
  // Kamus
  { label: "kunci",        kind: vscode.CompletionItemKind.Function, detail: "kamus", documentation: "Ambil semua kunci kamus.", insertText: "kunci(${1:kamus})" },
  { label: "nilai",        kind: vscode.CompletionItemKind.Function, detail: "kamus", documentation: "Ambil semua nilai kamus.", insertText: "nilai(${1:kamus})" },
  { label: "ada_kunci",    kind: vscode.CompletionItemKind.Function, detail: "kamus", documentation: "Cek apakah kunci ada.", insertText: "ada_kunci(${1:kamus}, ${2:kunci})" },
  { label: "dapatkan",     kind: vscode.CompletionItemKind.Function, detail: "kamus", documentation: "Dapatkan nilai dengan default.", insertText: "dapatkan(${1:kamus}, ${2:kunci}, ${3:bawaan})" },
  // Sistem
  { label: "baca_file",    kind: vscode.CompletionItemKind.Function, detail: "sistem", documentation: "Baca isi file.", insertText: "baca_file(${1:path})" },
  { label: "tulis_file",   kind: vscode.CompletionItemKind.Function, detail: "sistem", documentation: "Tulis ke file.", insertText: "tulis_file(${1:path}, ${2:isi})" },
  { label: "ada_file",     kind: vscode.CompletionItemKind.Function, detail: "sistem", documentation: "Cek apakah file ada.", insertText: "ada_file(${1:path})" },
  { label: "waktu_sekarang", kind: vscode.CompletionItemKind.Function, detail: "sistem", documentation: "Unix timestamp sekarang (detik).", insertText: "waktu_sekarang()" },
  { label: "tunda",        kind: vscode.CompletionItemKind.Function, detail: "sistem", documentation: "Tunda eksekusi (milidetik).", insertText: "tunda(${1:ms})" },
  // Utilitas
  { label: "adalah_angka", kind: vscode.CompletionItemKind.Function, detail: "utilitas", documentation: "Cek apakah nilai adalah angka.", insertText: "adalah_angka(${1:nilai})" },
  { label: "adalah_teks",  kind: vscode.CompletionItemKind.Function, detail: "utilitas", documentation: "Cek apakah nilai adalah teks.", insertText: "adalah_teks(${1:nilai})" },
  { label: "adalah_daftar",kind: vscode.CompletionItemKind.Function, detail: "utilitas", documentation: "Cek apakah nilai adalah daftar.", insertText: "adalah_daftar(${1:nilai})" },
  { label: "adalah_kamus", kind: vscode.CompletionItemKind.Function, detail: "utilitas", documentation: "Cek apakah nilai adalah kamus.", insertText: "adalah_kamus(${1:nilai})" },
  { label: "salin",        kind: vscode.CompletionItemKind.Function, detail: "utilitas", documentation: "Deep copy nilai.", insertText: "salin(${1:nilai})" },
  { label: "angka",        kind: vscode.CompletionItemKind.Function, detail: "utilitas", documentation: "Konversi ke angka.", insertText: "angka(${1:nilai})" },
  { label: "teks",         kind: vscode.CompletionItemKind.Function, detail: "utilitas", documentation: "Konversi ke teks.", insertText: "teks(${1:nilai})" },
  { label: "ke_boolean",   kind: vscode.CompletionItemKind.Function, detail: "utilitas", documentation: "Konversi ke boolean.", insertText: "ke_boolean(${1:nilai})" },
];

// Modul stdlib (dipakai di autocomplete setelah `impor`)
const MODUL_STDLIB = ["string", "matematika", "daftar", "kamus", "sistem", "utilitas", "konsol"];

// ── Aktivasi ekstensi ────────────────────────────────────────────────────────

export function activate(context: vscode.ExtensionContext): void {
  // Autocomplete statis keyword + builtin
  const completionProvider = vscode.languages.registerCompletionItemProvider(
    { language: "evernight", scheme: "file" },
    {
      provideCompletionItems(
        document: vscode.TextDocument,
        position: vscode.Position
      ): vscode.CompletionItem[] {
        const linePrefix = document.lineAt(position).text.slice(0, position.character);

        // Setelah `impor "` → sarankan nama modul stdlib
        if (/\bimpor\s+"[^"]*$/.test(linePrefix)) {
          return MODUL_STDLIB.map((m) => {
            const item = new vscode.CompletionItem(m, vscode.CompletionItemKind.Module);
            item.detail = "modul stdlib";
            item.documentation = new vscode.MarkdownString(`Modul pustaka standar \`${m}\`.`);
            return item;
          });
        }

        // Autocomplete umum — keyword + builtin
        const all = [...KEYWORDS, ...BUILTINS];
        return all.map((c) => {
          const item = new vscode.CompletionItem(c.label, c.kind);
          item.detail = c.detail;
          item.documentation = new vscode.MarkdownString(c.documentation);
          if (c.insertText) {
            item.insertText = new vscode.SnippetString(c.insertText);
          }
          return item;
        });
      },
    },
    // Trigger characters
    ".", '"', " "
  );

  // Perintah debug sederhana
  const haloCmd = vscode.commands.registerCommand("evernight.halo", () => {
    vscode.window.showInformationMessage("EvernightLanguage aktif!");
  });

  context.subscriptions.push(completionProvider, haloCmd);
}

export function deactivate(): void {}
