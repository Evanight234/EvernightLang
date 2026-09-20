# update.ps1 - Perbarui EvernightLanguage dari folder version di GitHub.
#
# Murni perintah terminal (tanpa GUI). Dipanggil oleh update.cmd:
#   update evernight system [--cek]
#
# Alur: baca versi lokal -> unduh version/version -> bandingkan ->
# tidak ada update = pesan zhongk | ada update = unduh file -> timpa ->
# reinstall ekstensi (best-effort) -> pesan zhongk.

# GANTI dengan alamat repo GitHub setelah di-push (mis. "satriyo/EvernightLanguage").
$REPO = "<akun>/<repo>"
$DASAR = "https://raw.githubusercontent.com/$REPO/main/version"

$ErrorActionPreference = "Stop"

function Versi-Lokal {
    $reg = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\EvernightLanguage"
    $v = (Get-ItemProperty -Path $reg -ErrorAction SilentlyContinue).DisplayVersion
    if ([string]::IsNullOrWhiteSpace($v)) { return "0.1.0" }
    return $v.Trim()
}

function Lokasi-Instalasi {
    $reg = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\EvernightLanguage"
    $l = (Get-ItemProperty -Path $reg -ErrorAction SilentlyContinue).InstallLocation
    if ([string]::IsNullOrWhiteSpace($l)) {
        return (Join-Path $env:LOCALAPPDATA "Programs\Evernight")
    }
    return $l.Trim()
}

function Unduh-Teks($url) {
    $wc = New-Object System.Net.WebClient
    $wc.Encoding = [System.Text.Encoding]::UTF8
    return $wc.DownloadString($url).Trim()
}

function Unduh-Berkas($url, $tujuan) {
    $wc = New-Object System.Net.WebClient
    $wc.DownloadFile($url, $tujuan)
}

# --- Mulai ---
$cekSaja = $args -contains "--cek"

$lokal = Versi-Lokal
$tujuan = Lokasi-Instalasi

try {
    $remote = Unduh-Teks "$DASAR/version"
} catch {
    Write-Host ""
    Write-Host "  BAHAYA: Gagal mengambil info versi dari GitHub."
    Write-Host "  Periksa koneksi internet atau alamat repo ($REPO)."
    Write-Host ""
    exit 1
}

if ([string]::IsNullOrWhiteSpace($remote)) {
    Write-Host ""
    Write-Host "  BAHAYA: Berkas version di GitHub kosong."
    Write-Host ""
    exit 1
}

try { $vLokal = [System.Version]$lokal } catch { $vLokal = [System.Version]"0.1.0" }
try { $vRemote = [System.Version]$remote } catch {
    Write-Host ""
    Write-Host "  BAHAYA: Format versi di GitHub tidak valid: $remote"
    Write-Host ""
    exit 1
}

if ($cekSaja) {
    Write-Host ""
    Write-Host "  Versi terpasang : $lokal"
    Write-Host "  Versi di GitHub : $remote"
    Write-Host ""
    exit 0
}

if ($vRemote -le $vLokal) {
    Write-Host ""
    Write-Host "  Istriku lagi sibuk jangan dingagu"
    Write-Host ""
    exit 0
}

# --- Ada update: unduh file-file dari folder version ---
Write-Host ""
Write-Host "  Versi baru tersedia: $remote (terpasang: $lokal)"
Write-Host "  Mengunduh pembaruan..."

$temp = Join-Path ([System.IO.Path]::GetTempPath()) "evernight-update-$remote"
New-Item -ItemType Directory -Force -Path $temp | Out-Null

$berkas = @(
    @{ Nama = "evernight.exe"; Url = "$DASAR/evernight.exe"; Tujuan = (Join-Path $tujuan "bin\evernight.exe") },
    @{ Nama = "evernight-language-$remote.vsix"; Url = "$DASAR/evernight-language-$remote.vsix"; Tujuan = (Join-Path $tujuan "extensions\evernight-language-$remote.vsix") }
)

try {
    foreach ($b in $berkas) {
        $tmp = Join-Path $temp $b.Nama
        Unduh-Berkas $b.Url $tmp
        $dirTujuan = Split-Path $b.Tujuan -Parent
        if (-not (Test-Path $dirTujuan)) { New-Item -ItemType Directory -Force -Path $dirTujuan | Out-Null }
        try {
            Copy-Item -Force $tmp $b.Tujuan
        } catch {
            # Berkas terkunci (mis. evernight.exe sedang berjalan): jadwalkan
            # penimpaan setelah jeda lewat cmd tersembunyi.
            $asal = $tmp; $akhir = $b.Tujuan
            $perintah = "timeout /t 3 /nobreak >nul & move /Y `"$asal`" `"$akhir`" >nul 2>&1"
            Start-Process -WindowStyle Hidden -FilePath "cmd.exe" -ArgumentList "/C", $perintah
            Write-Host "  Catatan: $($b.Nama) terkunci, penimpaan dijadwalkan."
        }
    }
} catch {
    Write-Host ""
    Write-Host "  BAHAYA: Gagal mengunduh pembaruan. Periksa koneksi internet."
    Write-Host ""
    exit 1
}

# --- Reinstall ekstensi ke editor terdeteksi (best-effort) ---
$vsixBaru = Join-Path $tujuan "extensions\evernight-language-$remote.vsix"
$kandidat = @(
    @{ Nama = "Antigravity IDE"; Cli = "Programs\Antigravity IDE\bin\antigravity-ide.cmd" },
    @{ Nama = "Visual Studio Code"; Cli = "Programs\Microsoft VS Code\bin\code.cmd" },
    @{ Nama = "VS Code Insiders"; Cli = "Programs\Microsoft VS Code Insiders\bin\code-insiders.cmd" },
    @{ Nama = "Cursor"; Cli = "Programs\cursor\resources\app\bin\cursor.cmd" },
    @{ Nama = "Windsurf"; Cli = "Programs\Windsurf\bin\windsurf.cmd" },
    @{ Nama = "VSCodium"; Cli = "Programs\VSCodium\bin\codium.cmd" }
)
foreach ($k in $kandidat) {
    $cli = Join-Path $env:LOCALAPPDATA $k.Cli
    if (Test-Path $cli) {
        try {
            $p = Start-Process -FilePath $cli -ArgumentList "--install-extension", $vsixBaru, "--force" -WindowStyle Hidden -Wait -PassThru
            if ($p.ExitCode -ne 0) { Write-Host "  Catatan: gagal memasang ekstensi di $($k.Nama)." }
        } catch {
            Write-Host "  Catatan: gagal memasang ekstensi di $($k.Nama)."
        }
    }
}

# --- Catat versi baru di registri ---
try {
    $kunci = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\EvernightLanguage"
    Set-ItemProperty -Path $kunci -Name "DisplayVersion" -Value $remote -ErrorAction SilentlyContinue
} catch { }

Remove-Item -Recurse -Force $temp -ErrorAction SilentlyContinue

Write-Host ""
Write-Host "  terupdate ke versi $remote, jangan panggil istriku lagi dasar karbit"
Write-Host ""
exit 0
