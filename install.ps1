<#
    ATTENTION: Ensure that this file is saved as UTF-8 with BOM to avoid encoding issues in PowerShell.
#>

$ErrorActionPreference = "Stop"
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8

$ExeName = "my-reboot.exe"
$AppName = "My Reboot"
$SwitchProfileTitle = "Trocar Perfil"

function Initialize-Executable {
    Write-Host "* Compilando..."
    cargo install --path . --config "env.WINDOWS_ICON='true'"
    Write-Host ""

    return Join-Path $env:USERPROFILE ".cargo\bin\$ExeName"
}

function New-Shortcut {
    param(
        [Parameter(Mandatory = $true)][string]$title,
        [string]$arguments,
        [string]$iconLocation,
        [string]$hotkey
    )

    Write-Host "* Criando atalho para $title na Área de Trabalho..."

    $WshShell = New-Object -ComObject WScript.Shell
    $DesktopPath = [System.Environment]::GetFolderPath("Desktop")

    $Shortcut = $WshShell.CreateShortcut((Join-Path $DesktopPath "$title.lnk"))
    $Shortcut.TargetPath = $ExeName
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

function Install-Shortcuts {
    $mainShortcutParams = @{
        title = $AppName
    }
    New-Shortcut @mainShortcutParams

    $displaySwitchPath = (Get-Command DisplaySwitch.exe -ErrorAction SilentlyContinue).Source
    if (-not $displaySwitchPath) {
        $displaySwitchPath = "C:\Windows\System32\DisplaySwitch.exe"
    }
    $profileSwitchShortcutParams = @{
        title = $SwitchProfileTitle
        arguments = "switch:other"
        iconLocation = "$displaySwitchPath,2"
        hotkey = "ALT+CTRL+X"
    }
    New-Shortcut @profileSwitchShortcutParams
}

function Set-ProfileSwitchRunOnStartup($exePath) {
    Write-Host "* Registrando troca de perfil ao iniciar o Windows..."
    $registryPath = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Run"
    $name = "$AppName Profile Switch"
    $value = "`"$exePath`" switch:saved"

    if (-not (Test-Path $registryPath)) {
        New-Item -Path $registryPath -Force | Out-Null
    }
    Set-ItemProperty -Path $registryPath -Name $name -Value $value
}

$exePath = Initialize-Executable
Install-Shortcuts
Set-ProfileSwitchRunOnStartup $exePath

Write-Host ""
Write-Host "* Instalação concluída"
Write-Host ""
Write-Host "Execute:"
Write-Host "  my-reboot configure"
Write-Host "para configurar."
