<#
    ATTENTION: Ensure that this file is saved as UTF-8 with BOM to avoid encoding issues in PowerShell.
#>

$ErrorActionPreference = "Stop"
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8

$ScriptDir = $PSScriptRoot
if (-not $ScriptDir) { $ScriptDir = Get-Location }

$ExeName = "my-reboot.exe"
$DestDir = Join-Path $env:LOCALAPPDATA "Programs\my-reboot"
$SwitchProfileTitle = "Trocar Perfil"

function Initialize-Executable {
    Write-Host "* Compilando..."
    cargo build --release --config "env.WINDOWS_ICON='true'"
    Write-Host ""

    Write-Host "* Copiando $ExeName para $DestDir ..."
    if (-not (Test-Path $DestDir)) {
        New-Item -ItemType Directory -Force -Path $DestDir | Out-Null
    }
    Copy-Item (Join-Path $ScriptDir "target\release\$ExeName") "$DestDir"
    Write-Host ""

    return Join-Path $DestDir $ExeName
}

function New-Shortcut {
    param(
        [Parameter(Mandatory = $true)][string]$title,
        [Parameter(Mandatory = $true)][string]$exePath,
        [string]$arguments,
        [string]$iconLocation,
        [string]$hotkey
    )

    $WshShell = New-Object -ComObject WScript.Shell
    $DesktopPath = [System.Environment]::GetFolderPath("Desktop")

    $Shortcut = $WshShell.CreateShortcut((Join-Path $DesktopPath "$title.lnk"))
    $Shortcut.TargetPath = $exePath
    if ($PSBoundParameters.ContainsKey("arguments")) {
        $Shortcut.Arguments = $arguments
    }
    $Shortcut.Description = $title
    $Shortcut.WindowStyle = 7 # Minimized
    if ($PSBoundParameters.ContainsKey("iconLocation")) {
        $Shortcut.IconLocation = $iconLocation
    }
    if ($PSBoundParameters.ContainsKey("hotkey")) {
        $Shortcut.Hotkey = $hotkey
    }

    $Shortcut.Save()
}

function Install-Shortcuts($exePath) {
    Write-Host "* Criando atalho para My Reboot na Área de Trabalho..."
    $mainShortcutParams = @{
        title = "My Reboot"
        exePath = $exePath
    }
    New-Shortcut @mainShortcutParams

    $displaySwitchPath = (Get-Command DisplaySwitch.exe -ErrorAction SilentlyContinue).Source
    if (-not $displaySwitchPath) {
        $displaySwitchPath = "C:\Windows\System32\DisplaySwitch.exe"
    }

    Write-Host "* Criando atalho para $SwitchProfileTitle na Área de Trabalho..."
    $profileSwitchShortcutParams = @{
        title = $SwitchProfileTitle
        exePath = $exePath
        arguments = "switch:other"
        iconLocation = "$displaySwitchPath,2"
        hotkey = "ALT+CTRL+X"
    }
    New-Shortcut @profileSwitchShortcutParams
}

function Set-ProfileSwitchRunOnStartup($exePath) {
    Write-Host "* Registrando troca de perfil ao iniciar o Windows..."
    $registryPath = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Run"
    $name = "My Reboot Profile Switch"
    $value = "`"$exePath`" switch:saved"

    if (-not (Test-Path $registryPath)) {
        New-Item -Path $registryPath -Force | Out-Null
    }
    Set-ItemProperty -Path $registryPath -Name $name -Value $value
}

function Add-ToPath($dir) {
    Write-Host "* Verificando PATH..."
    $path = [Environment]::GetEnvironmentVariable("PATH", "User")
    $dirs = $path -split ";"
    if ($dirs -notcontains $dir) {
        Write-Host "* Adicionando $dir ao PATH do usuário..."
        $newPath = "$path;$dir"
        [Environment]::SetEnvironmentVariable("PATH", $newPath, "User")
        $env:PATH += ";$dir"
    }
}

$exePath = Initialize-Executable
Install-Shortcuts $exePath
Set-ProfileSwitchRunOnStartup $exePath
Add-ToPath $DestDir

Write-Host ""
Write-Host "* Instalação concluída"
Write-Host "Execute:"
Write-Host "  my-reboot configure"
Write-Host "para configurar."
