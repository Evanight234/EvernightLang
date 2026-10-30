@echo off
REM kill.cmd - Copot EvernightLanguage dari sistem.
REM   CMD:    kill evernight system
REM   PowerShell:  .\kill.ps1
REM
REM Mencari uninstall.exe dari registri, lalu menjalankannya.

setlocal

REM Cari lokasi instalasi dari registri (InstallLocation).
for /f "tokens=2*" %%a in (
    'reg query "HKCU\Software\Microsoft\Windows\CurrentVersion\Uninstall\EvernightLanguage" /v InstallLocation 2^>nul'
) do set "LOKASI=%%b"

REM Fallback: default per-user.
if not defined LOKASI (
    set "LOKASI=%LOCALAPPDATA%\Programs\Evernight"
)

set "UNINSTALL=%LOKASI%\uninstall.exe"

if not exist "%UNINSTALL%" (
    echo.
    echo  BAHAYA: EvernightLanguage tidak terpasang atau uninstall.exe tidak ditemukan.
    echo  Lokasi dicari: %UNINSTALL%
    echo.
    exit /b 1
)

"%UNINSTALL%" --uninstall %*

set HASIL=%ERRORLEVEL%
exit /b %HASIL%
