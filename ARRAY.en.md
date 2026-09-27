# ARRAY.md - List (Array) EvernightLanguage

> Specification of the `daftar` (list) data type for EvernightLanguage.
> Semantics adopt Python 1:1 with Indonesian keywords & method names.
>
> **Implementation (2026-09-10):** method syntax `a.tambah(x)` is **not yet supported**: use the `daftar` module's global functions (`tambah(a, x)`, `sisip(a, i, x)`, `hapus(a, nil)`, etc; implementation & full table in `STDLIB.md §C`). The `hapus a[i]` statement removes an element by index (negative wrap, out-of-range `BAHAYA [INDEX]`). Slicing `a[awal:akhir]`, `a * n`, `nilai dalam a`, and the `ambil`/`bersihkan` methods are also still deferred (table rows below = design targets).

## 1. Decisions Already Made

| Aspect | Decision |
|--------|----------|
| Type name | **daftar** |
| Nature | Dynamic: length changes, mixed data types allowed |
| Type system | Dynamic |
| Stdlib | `daftar` module (Phase 3 in `RENCANA.md`) |

## 2. List Operations (Python Semantics)

| Operation | Syntax / Method | Python Equivalent | Example |
|-----------|-----------------|-------------------|---------|
| Create list | `[1, 2, 3]` / `daftar()` | `[1, 2, 3]` / `list()` | `variabel a = [1, "dua", 3.0]` |
| Index access | `a[indeks]` | `a[index]` | `a[0]`, `a[-1]` (negative indices supported) |
| Set element | `a[indeks] = nilai` | `a[index] = val` | `a[0] = 99` |
| Append | `a.tambah(nilai)` | `a.append(val)` | `a.tambah(4)` |
| Insert at index | `a.sisip(indeks, nilai)` | `a.insert(i, val)` | `a.sisip(0, "awal")` |
| Remove first value | `a.hapus(nilai)` | `a.remove(val)` | `a.hapus("dua")` |
| Pop | `a.ambil()` / `a.ambil(indeks)` | `a.pop()` / `a.pop(i)` | `variabel x = a.ambil()` |
| List length | `panjang(a)` | `len(a)` | `panjang(a)` |
| Slice | `a[awal:akhir:langkah]` | `a[start:end:step]` | `a[1:3]`, `a[:2]`, `a[::-1]` |
| Membership check | `nilai dalam a` | `val in a` | `jika 2 dalam a { ... }` |
| Concatenate | `a + b` | `a + b` | `[1, 2] + [3, 4]` |
| Repeat list | `a * n` | `a * n` | `[0] * 3` -> `[0, 0, 0]` |
| Clear | `a.bersihkan()` | `a.clear()` | `a.bersihkan()` |

## 3. Literal Syntax

- Format: `[elemen1, elemen2, ...]` (square brackets)
- Empty list: `[]` or `daftar()`

Example:
```eve
variabel angka = [10, 20, 30]
angka.tambah(40)
cetak(angka[0])        # 10
cetak(angka[-1])       # 40 (elemen terakhir)
cetak(angka[1:3])      # [20, 30]
```

## 4. Design Decisions

1. **Index**: Starts at **0**, supports negative indices (`-1` = the last element).
2. **Out-of-range access**: Produces the error **`BAHAYA: Indeks di luar batas jangkauan daftar!`** (IndexError).
3. **Nested lists**: **Allowed** (lists inside lists / multi-dimensional are fully supported, e.g. `matrix = [[1, 2], [3, 4]]`).

---

## Finalization Status

- [x] List operations completed (1:1 Python semantics)
- [x] Literal syntax chosen (`[...]`)
- [x] Design questions answered
