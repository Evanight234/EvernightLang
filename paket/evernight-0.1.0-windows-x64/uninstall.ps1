# uninstall.ps1 - Pencopot EvernightLanguage (per-user)
#
# Menghapus: folder instalasi, entri PATH pengguna, asosiasi berkas .eve,
# dan (opsional) ekstensi editor.
#
# Penggunaan:
#   powershell -ExecutionPolicy Bypass -File uninstall.ps1
#   powershell -ExecutionPolicy Bypass -File uninstall.ps1 -Uji
#   powershell -ExecutionPolicy Bypass -File uninstall.ps1 -SimpanEkstensi
#   powershell -ExecutionPolicy Bypass -File uninstall.ps1 -TanpaKonfirmasi

[CmdletBinding()]
param(
    [switch]$Uji,
    [switch]$SimpanEkstensi,
    [switch]$TanpaKonfirmasi,
    [string]$LokasiInstal = (Join-Path $env:LOCALAPPDATA "Programs\Evernight")
)

$ErrorActionPreference = "Stop"
$ProgId = "EvernightFile"
$IdEkstensi = "satriyo.evernight-language"

function Tulis($pesan, $warna = "Gray") { Write-Host $pesan -ForegroundColor $warna }
function Judul($pesan) { Write-Host ""; Write-Host "== $pesan ==" -ForegroundColor Cyan }

# Daftar editor yang didukung (sama seperti install.ps1)
$DaftarEditor = @(
    [pscustomobject]@{ Nama = "Antigravity IDE";     Cli = "antigravity-ide"; Jalur = @((Join-Path $env:LOCALAPPDATA "Programs\Antigravity IDE\bin\antigravity-ide.cmd")) },
    [pscustomobject]@{ Nama = "Visual Studio Code";  Cli = "code";            Jalur = @((Join-Path $env:LOCALAPPDATA "Programs\Microsoft VS Code\bin\code.cmd"), (Join-Path $env:ProgramFiles "Microsoft VS Code\bin\code.cmd"), (Join-Path ${env:ProgramFiles(x86)} "Microsoft VS Code\bin\code.cmd")) },
    [pscustomobject]@{ Nama = "VS Code Insiders";    Cli = "code-insiders";   Jalur = @((Join-Path $env:LOCALAPPDATA "Programs\Microsoft VS Code Insiders\bin\code-insiders.cmd"), (Join-Path $env:ProgramFiles "Microsoft VS Code Insiders\bin\code-insiders.cmd")) },
    [pscustomobject]@{ Nama = "Cursor";              Cli = "cursor";          Jalur = @((Join-Path $env:LOCALAPPDATA "Programs\cursor\resources\app\bin\cursor.cmd")) },
    [pscustomobject]@{ Nama = "Windsurf";            Cli = "windsurf";        Jalur = @((Join-Path $env:LOCALAPPDATA "Programs\Windsurf\bin\windsurf.cmd")) },
    [pscustomobject]@{ Nama = "VSCodium";            Cli = "codium";          Jalur = @((Join-Path $env:LOCALAPPDATA "Programs\VSCodium\bin\codium.cmd")) }
)

function Cari-CliEditor($entri) {
    $perintah = Get-Command $entri.Cli -ErrorAction SilentlyContinue
    if ($perintah) { return $perintah.Source }
    foreach ($j in $entri.Jalur) {
        if ($j -and (Test-Path $j)) { return $j }
    }
    return $null
}

Judul "Pencopot EvernightLanguage"
if ($Uji) { Tulis "[MODE UJI] Tidak akan ada perubahan pada sistem." "Yellow" }
Tulis "Folder instalasi : $LokasiInstal"

# ---------------------------------------------------------------- 1. Asosiasi registry
Judul "1. Menghapus asosiasi berkas .eve"
$base = "HKCU:\Software\Classes"
$kunci = @(
    "$base\.eve",
    "$base\$ProgId",
    "$base\Applications\evernight.exe"
)
foreach ($k in $kunci) {
    if (Test-Path $k) {
        if ($Uji) {
            Tulis "[UJI] Hapus registry: $k" "Yellow"
        } else {
            Remove-Item -Path $k -Recurse -Force
            Tulis "Dihapus: $k" "Green"
        }
    } else {
        Tulis "Lewati (tidak ada): $k" "DarkGray"
    }
}

# ---------------------------------------------------------------- 2. PATH pengguna
Judul "2. Menghapus entri PATH pengguna"
$folderBin = Join-Path $LokasiInstal "bin"
$pathLama = [Environment]::GetEnvironmentVariable("Path", "User")
if ([string]::IsNullOrWhiteSpace($pathLama)) {
    Tulis "PATH pengguna kosong (tidak ada yang dihapus)." "DarkGray"
} else {
    $bagian = $pathLama -split ";" | Where-Object { $_.Trim() -ne "" }
    $sisa = $bagian | Where-Object { $_.Trim().TrimEnd("\") -ine $folderBin.TrimEnd("\") }
    if ($sisa.Count -eq $bagian.Count) {
        Tulis "Entri PATH tidak ditemukan (tidak ada perubahan)." "DarkGray"
    } elseif ($Uji) {
        Tulis "[UJI] Hapus dari PATH: $folderBin" "Yellow"
    } else {
        $pathBaru = [string]::Join(";", $sisa)
        [Environment]::SetEnvironmentVariable("Path", $pathBaru, "User")
        Tulis "Entri PATH dihapus." "Green"
    }
}

# ---------------------------------------------------------------- 3. Ekstensi editor (opsional)
if ($SimpanEkstensi) {
    Judul "3. Ekstensi editor dibiarkan terpasang"
    Tulis "Dilewati karena opsi -SimpanEkstensi." "DarkGray"
} else {
    Judul "3. Ekstensi editor"
    $terdeteksi = @()
    foreach ($e in $DaftarEditor) {
        $cli = Cari-CliEditor $e
        if ($cli) { $terdeteksi += [pscustomobject]@{ Nama = $e.Nama; Cli = $cli } }
    }

    if ($terdeteksi.Count -eq 0) {
        Tulis "Tidak ada editor keluarga VS Code yang terdeteksi." "DarkGray"
    } else {
        $copoT = $false
        if ($TanpaKonfirmasi) {
            $copoT = $true
        } elseif ($Uji) {
            $copoT = $true
        } else {
            $jawab = Read-Host "Copot juga ekstensi EvernightLanguage dari editor? (y/t)"
            $copoT = ($jawab -match '^(y|ya|t|true|1)$')
        }

        if (-not $copoT) {
            Tulis "Ekstensi dibiarkan terpasang." "DarkGray"
        } else {
            foreach ($t in $terdeteksi) {
                if ($Uji) {
                    Tulis "[UJI] `"$($t.Cli)`" --uninstall-extension $IdEkstensi" "Yellow"
                    continue
                }
                # Beberapa CLI editor menulis peringatan ke stderr walau berhasil.
                $eapLama = $ErrorActionPreference
                $ErrorActionPreference = "Continue"
                try {
                    $keluaran = (& $t.Cli --uninstall-extension $IdEkstensi 2>&1 | Out-String)
                    if ($keluaran -match "not installed|not found|not present") {
                        Tulis "Tidak terpasang di $($t.Nama) (dilewati)." "DarkGray"
                    } else {
                        Tulis "Ekstensi dicopot dari $($t.Nama)." "Green"
                    }
                } catch {
                    Tulis "Gagal mencopot dari $($t.Nama): $($_.Exception.Message)" "Yellow"
                } finally {
                    $ErrorActionPreference = $eapLama
                }
            }
        }
    }
}

# ---------------------------------------------------------------- 4. Folder instalasi
Judul "4. Menghapus folder instalasi"
if (Test-Path $LokasiInstal) {
    if ($Uji) {
        Tulis "[UJI] Hapus folder: $LokasiInstal" "Yellow"
    } else {
        # Bila berkas terkunci (mis. antivirus), jadwalkan hapus lewat proses terpisah.
        try {
            Remove-Item -Path $LokasiInstal -Recurse -Force -ErrorAction Stop
            Tulis "Folder dihapus." "Green"
        } catch {
            Tulis "Sebagian berkas terkunci. Menjadwalkan penghapusan tertunda..." "Yellow"
            $sementara = Join-Path $env:TEMP ("eve_bersih_" + [Guid]::NewGuid().ToString("N") + ".cmd")
            $isi = "@echo off`r`ntimeout /t 2 /nobreak >nul`r`nrmdir /s /q `"$LokasiInstal`"`r`ndel `"%~f0`""
            Set-Content -Path $sementara -Value $isi -Encoding ASCII
            Start-Process -FilePath "cmd.exe" -ArgumentList "/c", "`"$sementara`"" -WindowStyle Hidden
        }
    }
} else {
    Tulis "Folder tidak ditemukan (sudah bersih)." "DarkGray"
}

# ---------------------------------------------------------------- 5. Refresh shell
if (-not $Uji) {
    try {
        Add-Type -Namespace Win32 -Name ShellU -MemberDefinition @"
[DllImport("shell32.dll")]
public static extern void SHChangeNotify(int wEventId, int uFlags, IntPtr dwItem1, IntPtr dwItem2);
"@ -ErrorAction SilentlyContinue
        [Win32.ShellU]::SHChangeNotify(0x08000000, 0x0000, [IntPtr]::Zero, [IntPtr]::Zero)
    } catch { }
}

Judul "Selesai"
if ($Uji) {
    Tulis "[UJI] Tidak ada perubahan yang dilakukan." "Yellow"
} else {
    Tulis "EvernightLanguage telah dicopot." "Green"
    Tulis "Tutup terminal lama dan buka yang baru agar perubahan PATH berlaku."
}
Tulis ""
