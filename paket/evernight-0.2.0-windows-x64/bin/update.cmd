@echo off
REM update.cmd - Perbarui EvernightLanguage dari folder version di GitHub.
REM   CMD:        update evernight system
REM   PowerShell: update evernight system
REM
REM Murni perintah terminal (tanpa GUI). Memanggil update.ps1 yang berisi
REM seluruh logika (cek versi -> unduh -> timpa -> pesan).

setlocal

if /i not "%~1"=="evernight" goto :pakai
if /i not "%~2"=="system" goto :pakai

REM Cari lokasi instalasi dari registri (InstallLocation).
for /f "tokens=2*" %%a in (
    'reg query "HKCU\Software\Microsoft\Windows\CurrentVersion\Uninstall\EvernightLanguage" /v InstallLocation 2^>nul'
) do set "LOKASI=%%b"

REM Fallback: default per-user.
if not defined LOKASI (
    set "LOKASI=%LOCALAPPDATA%\Programs\Evernight"
)

set "BIN=%LOKASI%\bin"
set "SKRIP=%BIN%\update.ps1"

REM Bila dipanggil dari folder proyek (belum terpasang), pakai skrip lokal.
if not exist "%SKRIP%" (
    set "SKRIP=%~dp0update.ps1"
)

if not exist "%SKRIP%" (
    echo.
    echo  BAHAYA: update.ps1 tidak ditemukan.
    echo.
    exit /b 1
)

powershell -NoProfile -ExecutionPolicy Bypass -File "%SKRIP%" %3 %4 %5 %6 %7 %8 %9
exit /b %ERRORLEVEL%

:pakai
echo.
echo  Penggunaan: update evernight system [--cek]
echo.
exit /b 1
