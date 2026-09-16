# install.ps1 - Pemasang EvernightLanguage (per-user, tanpa hak admin)
#
# Memasang compiler EvernightLanguage ke sistem Windows:
#   - Menyalin berkas ke %LocalAppData%\Programs\Evernight
#   - Menambahkan folder bin ke PATH pengguna (idempoten)
#   - Mengasosiasikan berkas .eve dengan "Evernight files"
#   - Menawarkan pemasangan ekstensi editor (Antigravity/VS Code/Cursor/Windsurf/VSCodium)
#
# Penggunaan:
#   powershell -ExecutionPolicy Bypass -File install.ps1
#   powershell -ExecutionPolicy Bypass -File install.ps1 -Uji             # uji coba (tidak mengubah sistem)
#   powershell -ExecutionPolicy Bypass -File install.ps1 -TanpaAsosiasi
#   powershell -ExecutionPolicy Bypass -File install.ps1 -TanpaEkstensi
#   powershell -ExecutionPolicy Bypass -File install.ps1 -TanpaKonfirmasi # pasang ekstensi tanpa bertanya

[CmdletBinding()]
param(
    [switch]$Uji,
    [switch]$TanpaAsosiasi,
    [switch]$TanpaEkstensi,
    [switch]$TanpaKonfirmasi,
    [string]$LokasiInstal = (Join-Path $env:LOCALAPPDATA "Programs\Evernight")
)

$ErrorActionPreference = "Stop"
$Versi = "0.1.0"
$ProgId = "EvernightFile"
$LabelTipe = "Evernight files"
$NamaApp = "EvernightLanguage"
$NamaVsix = "evernight-language-$Versi.vsix"

function Tulis($pesan, $warna = "Gray") { Write-Host $pesan -ForegroundColor $warna }
function Judul($pesan) { Write-Host ""; Write-Host "== $pesan ==" -ForegroundColor Cyan }

# ---------------------------------------------------------------- Daftar editor yang didukung
$DaftarEditor = @(
    [pscustomobject]@{
        Nama = "Antigravity IDE"
        Cli  = "antigravity-ide"
        Jalur = @(
            (Join-Path $env:LOCALAPPDATA "Programs\Antigravity IDE\bin\antigravity-ide.cmd")
        )
    },
    [pscustomobject]@{
        Nama = "Visual Studio Code"
        Cli  = "code"
        Jalur = @(
            (Join-Path $env:LOCALAPPDATA "Programs\Microsoft VS Code\bin\code.cmd"),
            (Join-Path $env:ProgramFiles "Microsoft VS Code\bin\code.cmd"),
            (Join-Path ${env:ProgramFiles(x86)} "Microsoft VS Code\bin\code.cmd")
        )
    },
    [pscustomobject]@{
        Nama = "VS Code Insiders"
        Cli  = "code-insiders"
        Jalur = @(
            (Join-Path $env:LOCALAPPDATA "Programs\Microsoft VS Code Insiders\bin\code-insiders.cmd"),
            (Join-Path $env:ProgramFiles "Microsoft VS Code Insiders\bin\code-insiders.cmd")
        )
    },
    [pscustomobject]@{
        Nama = "Cursor"
        Cli  = "cursor"
        Jalur = @(
            (Join-Path $env:LOCALAPPDATA "Programs\cursor\resources\app\bin\cursor.cmd")
        )
    },
    [pscustomobject]@{
        Nama = "Windsurf"
        Cli  = "windsurf"
        Jalur = @(
            (Join-Path $env:LOCALAPPDATA "Programs\Windsurf\bin\windsurf.cmd")
        )
    },
    [pscustomobject]@{
        Nama = "VSCodium"
        Cli  = "codium"
        Jalur = @(
            (Join-Path $env:LOCALAPPDATA "Programs\VSCodium\bin\codium.cmd")
        )
    }
)

# Mencari CLI editor: cek PATH dahulu, lalu lokasi instalasi standar.
# Catatan: TIDAK memakai keberadaan folder data (mis. .cursor) karena folder itu
# bisa ada walau aplikasinya tidak terpasang.
function Cari-CliEditor($entri) {
    $perintah = Get-Command $entri.Cli -ErrorAction SilentlyContinue
    if ($perintah) { return $perintah.Source }
    foreach ($j in $entri.Jalur) {
        if ($j -and (Test-Path $j)) { return $j }
    }
    return $null
}

Judul "Pemasang $NamaApp v$Versi"
if ($Uji) { Tulis "[MODE UJI] Tidak akan ada perubahan pada sistem." "Yellow" }
Tulis "Tujuan pemasangan : $LokasiInstal"
Tulis "Asosiasi .eve     : $(if ($TanpaAsosiasi) { 'Dilewati (-TanpaAsosiasi)' } else { 'Ya' })"
Tulis "Ekstensi editor   : $(if ($TanpaEkstensi) { 'Dilewati (-TanpaEkstensi)' } else { 'Ditawarkan otomatis' })"

# ---------------------------------------------------------------- 1. Validasi sumber
Judul "1. Memeriksa berkas sumber"
$akarSumber = $PSScriptRoot
$sumberExe = Join-Path $akarSumber "bin\evernight.exe"

if (-not (Test-Path $sumberExe)) {
    Tulis "GAGAL: berkas 'bin\evernight.exe' tidak ditemukan di $akarSumber" "Red"
    Tulis "Pastikan skrip ini dijalankan dari dalam folder paket." "Red"
    exit 1
}
Tulis "OK: bin\evernight.exe ditemukan" "Green"

# ---------------------------------------------------------------- 2. Salin berkas
Judul "2. Menyalin berkas aplikasi"
$folderPaket = @("bin", "assets", "extensions", "docs")

foreach ($f in $folderPaket) {
    $asal = Join-Path $akarSumber $f
    $tujuan = Join-Path $LokasiInstal $f
    if (-not (Test-Path $asal)) {
        Tulis "Lewati: folder '$f' tidak ada di paket." "Yellow"
        continue
    }
    if ($Uji) {
        Tulis "[UJI] Salin '$f' -> '$tujuan'" "Yellow"
    } else {
        New-Item -ItemType Directory -Force -Path $tujuan | Out-Null
        Copy-Item -Path (Join-Path $asal "*") -Destination $tujuan -Recurse -Force
        Tulis "Disalin: $f" "Green"
    }
}

$exeTerpasang = Join-Path $LokasiInstal "bin\evernight.exe"

# ---------------------------------------------------------------- 3. PATH pengguna
Judul "3. Menambahkan ke PATH pengguna"
$folderBin = Join-Path $LokasiInstal "bin"
$pathLama = [Environment]::GetEnvironmentVariable("Path", "User")
if ($null -eq $pathLama) { $pathLama = "" }

$sudahAda = ($pathLama -split ";" | Where-Object { $_.Trim().TrimEnd("\") -ieq $folderBin.TrimEnd("\") }).Count -gt 0

if ($sudahAda) {
    Tulis "Sudah terdaftar di PATH (tidak ada perubahan)." "Green"
} elseif ($Uji) {
    Tulis "[UJI] Tambah ke PATH: $folderBin" "Yellow"
} else {
    $pathBaru = if ([string]::IsNullOrWhiteSpace($pathLama)) { $folderBin } else { $pathLama.TrimEnd(";") + ";" + $folderBin }
    [Environment]::SetEnvironmentVariable("Path", $pathBaru, "User")
    Tulis "Ditambahkan ke PATH pengguna." "Green"
}

# ---------------------------------------------------------------- 4. Asosiasi .eve
if (-not $TanpaAsosiasi) {
    Judul "4. Mengasosiasikan berkas .eve"

    if ($Uji) {
        Tulis "[UJI] HKCU\Software\Classes\.eve -> $ProgId" "Yellow"
        Tulis "[UJI] HKCU\Software\Classes\$ProgId -> '$LabelTipe'" "Yellow"
        Tulis "[UJI] Ikon -> $LokasiInstal\assets\icon.ico" "Yellow"
        Tulis "[UJI] Perintah -> `"$exeTerpasang`" run `"%1`"" "Yellow"
    } else {
        $base = "HKCU:\Software\Classes"

        # .eve -> ProgID
        New-Item -Path "$base\.eve" -Force | Out-Null
        Set-ItemProperty -Path "$base\.eve" -Name "(Default)" -Value $ProgId

        # ProgID utama + label tipe
        New-Item -Path "$base\$ProgId" -Force | Out-Null
        Set-ItemProperty -Path "$base\$ProgId" -Name "(Default)" -Value $LabelTipe
        New-Item -Path "$base\$ProgId\DefaultIcon" -Force | Out-Null
        Set-ItemProperty -Path "$base\$ProgId\DefaultIcon" -Name "(Default)" -Value ("{0},0" -f (Join-Path $LokasiInstal "assets\icon.ico"))
        New-Item -Path "$base\$ProgId\shell\open\command" -Force | Out-Null
        Set-ItemProperty -Path "$base\$ProgId\shell\open\command" -Name "(Default)" -Value ("`"{0}`" run `"%1`"" -f $exeTerpasang)

        # Daftarkan aplikasi agar muncul di "Open with"
        $appKey = Join-Path $base "Applications\evernight.exe"
        New-Item -Path "$appKey\shell\open\command" -Force | Out-Null
        Set-ItemProperty -Path "$appKey" -Name "FriendlyAppName" -Value $NamaApp
        Set-ItemProperty -Path "$appKey" -Name "(Default)" -Value $NamaApp
        Set-ItemProperty -Path "$appKey\shell\open\command" -Name "(Default)" -Value ("`"{0}`" run `"%1`"" -f $exeTerpasang)

        # Beri tahu Windows agar ikon/tipi file langsung diperbarui
        try {
            Add-Type -Namespace Win32 -Name Shell -MemberDefinition @"
[DllImport("shell32.dll")]
public static extern void SHChangeNotify(int wEventId, int uFlags, IntPtr dwItem1, IntPtr dwItem2);
"@ -ErrorAction SilentlyContinue
            [Win32.Shell]::SHChangeNotify(0x08000000, 0x0000, [IntPtr]::Zero, [IntPtr]::Zero)
        } catch { }

        Tulis "Asosiasi .eve selesai." "Green"
    }
} else {
    Judul "4. Asosiasi .eve dilewati"
}

# ---------------------------------------------------------------- 5. Ekstensi editor
$vsixSumber = Join-Path $akarSumber "extensions\$NamaVsix"

if ($TanpaEkstensi) {
    Judul "5. Ekstensi editor dilewati"
    Tulis "Dilewati karena opsi -TanpaEkstensi." "DarkGray"
} elseif (-not (Test-Path $vsixSumber)) {
    Judul "5. Ekstensi editor"
    Tulis "Berkas ekstensi tidak ditemukan: extensions\$NamaVsix" "Yellow"
    Tulis "Lewati langkah ini." "DarkGray"
} else {
    Judul "5. Ekstensi editor (warna sintaksis & autocomplete)"

    # Deteksi editor yang benar-benar terpasang (lewat CLI)
    $terdeteksi = @()
    foreach ($e in $DaftarEditor) {
        $cli = Cari-CliEditor $e
        if ($cli) {
            $terdeteksi += [pscustomobject]@{ Nama = $e.Nama; Cli = $cli }
        }
    }

    if ($terdeteksi.Count -eq 0) {
        Tulis "Tidak ada editor keluarga VS Code yang terdeteksi." "Yellow"
        Tulis "Anda dapat memasang ekstensi secara manual nanti (lihat docs/PANDUAN.txt)." "DarkGray"
    } else {
        Tulis "Editor terdeteksi:" "White"
        foreach ($t in $terdeteksi) { Tulis "  - $($t.Nama)" "Green" }

        $lanjut = $true
        if (-not $TanpaKonfirmasi) {
            if ($Uji) {
                $lanjut = $true
            } else {
                $jawab = Read-Host "Pasang ekstensi EvernightLanguage ke editor di atas? (y/t)"
                $lanjut = ($jawab -match '^(y|ya|t|true|1)$')
            }
        }

        if (-not $lanjut) {
            Tulis "Pemasangan ekstensi dilewati oleh pengguna." "DarkGray"
            Tulis "Anda bisa memasangnya nanti lewat docs/PANDUAN.txt." "DarkGray"
        } else {
            foreach ($t in $terdeteksi) {
                if ($Uji) {
                    Tulis "[UJI] `"$($t.Cli)`" --install-extension `"$vsixSumber`" --force" "Yellow"
                    continue
                }
                # Beberapa CLI editor menulis peringatan ke stderr walau berhasil.
                # Longgarkan ErrorActionPreference agar peringatan itu tidak dianggap fatal.
                $eapLama = $ErrorActionPreference
                $ErrorActionPreference = "Continue"
                try {
                    $keluaran = (& $t.Cli --install-extension $vsixSumber --force 2>&1 | Out-String)
                    $kode = $LASTEXITCODE
                    $berhasil = ($keluaran -match "successfully installed") -or ($keluaran -match "already installed") -or ($kode -eq 0)
                    if ($berhasil) {
                        Tulis "Ekstensi terpasang: $($t.Nama)" "Green"
                    } else {
                        Tulis "Gagal memasang ke $($t.Nama). Coba manual (lihat docs/PANDUAN.txt)." "Yellow"
                    }
                } catch {
                    Tulis "Gagal memasang ke $($t.Nama): $($_.Exception.Message)" "Yellow"
                } finally {
                    $ErrorActionPreference = $eapLama
                }
            }
            Tulis ""
            Tulis "Aktifkan tema (opsional): Ctrl+Shift+P -> Preferences: Color Theme -> 'Evernight Nusantara Gelap'" "White"
        }
    }
}

# ---------------------------------------------------------------- 6. Verifikasi
Judul "6. Verifikasi"
if ($Uji) {
    Tulis "[UJI] Selesai uji coba. Tidak ada perubahan yang dilakukan." "Yellow"
} else {
    if (Test-Path $exeTerpasang) {
        try {
            $keluaran = & $exeTerpasang --versi 2>&1
            Tulis "Biner terpasang: $keluaran" "Green"
        } catch {
            Tulis "Peringatan: biner belum dapat dijalankan (mungkin sedang dipindai antivirus). Coba lagi." "Yellow"
        }
    }
    Tulis ""
    Tulis "Pemasangan SELESAI." "Green"
    Tulis ""
    Tulis "Langkah selanjutnya:" "White"
    Tulis "  1. TUTUP lalu BUKA terminal baru (agar PATH terbaru terpakai)."
    Tulis "  2. Ketik: evernight --versi"
    Tulis "  3. Jalankan program: evernight run berkas.eve"
    Tulis "  4. Atau klik dua kali berkas .eve di Explorer."
    Tulis ""
    Tulis "Dokumentasi lengkap: docs/PANDUAN.txt"
}
Tulis ""
