"""Cutout maskot installer EvernightLanguage (berbasis konektivitas).

Masalah pendekatan ambang-warna murni: area GELAP di dalam karakter yang
kebetulan mirip warna latar ikut menjadi transparan (lubang). Solusi:
hanya piksel latar yang TERHUBUNG ke tepi gambar yang dijadikan transparan.

Tahapan:
1. Deteksi warna latar dari bingkai terluar (median).
2. Tandai kandidat latar: jarak warna < ambang.
3. Flood-fill (BFS) dari piksel kandidat yang menyentuh tepi gambar.
   -> hanya latar yang benar-benar terhubung tepi yang diambil.
4. Alpha = 0 untuk latar, 1 untuk subjek.
5. Haluskan tepi: blur alpha lalu paksa bagian dalam/luar tetap tegas.
6. Skala untuk panel 28% (isi tinggi, crop tengah) + simpan pratinjau.
"""
from collections import deque

import numpy as np
from PIL import Image, ImageDraw, ImageFilter

SUMBER = r"E:\EvernightLang\assets\evernight instaler models.jpeg"
KELUAR = r"E:\EvernightLang\installer\aset"
PANEL_W, PANEL_H = 213, 520
WIN_W, WIN_H = 760, 520

# Palet installer (dari BRAND + gambar referensi user)
C_LATAR = (0x37, 0x31, 0x3D)
C_WELL = (0x2F, 0x24, 0x30)
C_KARTU = (0x44, 0x3C, 0x4A)
C_BORDER = (0x7A, 0x60, 0x74)
C_TEKS = (0xF8, 0xDC, 0xE8)
C_SEKUNDER = (0xB4, 0x9A, 0xAB)
C_AKSEN = (0xD3, 0xA0, 0xB9)

T_KANDIDAT = 42.0  # ambang jarak warna untuk "mungkin latar"


def muat():
    im = Image.open(SUMBER).convert("RGB")
    return np.asarray(im).astype(np.float32)


def warna_latar(arr):
    bingkai = np.concatenate([
        arr[0:4, :, :].reshape(-1, 3),
        arr[-4:, :, :].reshape(-1, 3),
        arr[:, 0:4, :].reshape(-1, 3),
        arr[:, -4:, :].reshape(-1, 3),
    ])
    return np.median(bingkai, axis=0)


def topeng_latar(arr, bg):
    """True = latar (terhubung ke tepi), False = subjek."""
    h, w, _ = arr.shape
    jarak = np.sqrt(((arr - bg[None, None, :]) ** 2).sum(axis=2))
    kandidat = jarak < T_KANDIDAT

    latar = np.zeros((h, w), dtype=bool)
    q = deque()

    # Bibit: semua piksel kandidat di tepi gambar
    for x in range(w):
        for y in (0, h - 1):
            if kandidat[y, x] and not latar[y, x]:
                latar[y, x] = True
                q.append((y, x))
    for y in range(h):
        for x in (0, w - 1):
            if kandidat[y, x] and not latar[y, x]:
                latar[y, x] = True
                q.append((y, x))

    while q:
        y, x = q.popleft()
        for dy, dx in ((1, 0), (-1, 0), (0, 1), (0, -1)):
            ny, nx = y + dy, x + dx
            if 0 <= ny < h and 0 <= nx < w and kandidat[ny, nx] and not latar[ny, nx]:
                latar[ny, nx] = True
                q.append((ny, nx))

    return latar


def haluskan_tepi(latar):
    """Alpha lembut di tepi, tegas di dalam/luar."""
    keras = np.where(latar, 0.0, 1.0).astype(np.float32)
    img = Image.fromarray((keras * 255).astype(np.uint8), mode="L")
    lembut = np.asarray(img.filter(ImageFilter.GaussianBlur(radius=0.9))).astype(np.float32) / 255.0

    dalam = np.asarray(img.filter(ImageFilter.MinFilter(size=3))).astype(np.float32) / 255.0
    luar = np.asarray(img.filter(ImageFilter.MaxFilter(size=3))).astype(np.float32) / 255.0

    hasil = lembut.copy()
    hasil[luar < 0.5] = 0.0     # jauh di luar -> transparan penuh
    hasil[dalam > 0.5] = 1.0    # jauh di dalam -> solid penuh
    return hasil


def main():
    arr = muat()
    h, w, _ = arr.shape
    bg = warna_latar(arr)
    print(f"Sumber : {w}x{h}")
    print(f"Latar  : #{int(bg[0]):02X}{int(bg[1]):02X}{int(bg[2]):02X}")

    latar = topeng_latar(arr, bg)
    alpha = haluskan_tepi(latar)
    print(f"Latar  : {100 * latar.mean():.1f}%  |  Subjek: {100 * (1 - latar.mean()):.1f}%")

    rgba = np.dstack([arr, alpha * 255.0]).astype(np.uint8)
    penuh = Image.fromarray(rgba, mode="RGBA")
    penuh.save(f"{KELUAR}\\maskot-penuh.png")

    # Skala isi-tinggi lalu crop tengah ke ukuran panel
    gw, gh = penuh.size
    skala = PANEL_H / gh
    tw = max(1, round(gw * skala))
    panel = penuh.resize((tw, PANEL_H), Image.LANCZOS)
    if tw > PANEL_W:
        x0 = (tw - PANEL_W) // 2
        panel = panel.crop((x0, 0, x0 + PANEL_W, PANEL_H))
    panel.save(f"{KELUAR}\\maskot.png")
    print(f"Panel  : {panel.size} (skala {skala:.3f}, lebar penuh {tw}, crop {max(0, tw - PANEL_W)}px)")

    # Pratinjau komposit
    k = Image.new("RGB", (WIN_W, WIN_H), C_LATAR)
    k.paste(panel, (0, 0), panel)
    d = ImageDraw.Draw(k)

    d.rectangle((0, 0, WIN_W, 36), fill=C_WELL)
    d.rounded_rectangle((12, 11, 26, 25), radius=3, fill=C_AKSEN)
    d.text((34, 13), "EvernightLanguage Setup", fill=C_TEKS)
    for i, glif in enumerate(("-", "[]", "x")):
        d.text((WIN_W - 66 + i * 20, 13), glif, fill=C_SEKUNDER)

    kiri = PANEL_W + 28
    kanan = WIN_W - 28
    d.text((kiri, 52), "Memasang", fill=C_TEKS)
    d.text((kiri, 70), "Menyalin berkas, mohon tunggu...", fill=C_SEKUNDER)

    kotak = (kiri, 96, kanan, WIN_H - 76)
    d.rounded_rectangle(kotak, radius=8, fill=C_KARTU, outline=C_BORDER, width=1)
    track = (kiri + 20, kotak[1] + 24, kanan - 20, kotak[1] + 38)
    d.rounded_rectangle(track, radius=7, fill=C_WELL)
    isi = (track[0], track[1], track[0] + int((track[2] - track[0]) * 0.6), track[3])
    d.rounded_rectangle(isi, radius=7, fill=C_AKSEN)
    d.text((kiri + 20, kotak[1] + 52), "C:\\Users\\...\\Evernight\\bin\\evernight.exe", fill=C_SEKUNDER)

    d.rounded_rectangle((kanan - 196, WIN_H - 56, kanan - 120, WIN_H - 26), radius=8,
                        fill=C_KARTU, outline=C_BORDER, width=1)
    d.text((kanan - 178, WIN_H - 48), "Kembali", fill=C_SEKUNDER)
    d.rounded_rectangle((kanan - 108, WIN_H - 56, kanan, WIN_H - 26), radius=8, fill=C_AKSEN)
    d.text((kanan - 78, WIN_H - 48), "Lanjut", fill=C_WELL)

    k.save(f"{KELUAR}\\pratinjau-panel.png")
    print(f"Simpan : {KELUAR}\\maskot.png, maskot-penuh.png, pratinjau-panel.png")


if __name__ == "__main__":
    main()
