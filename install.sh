#!/bin/bash
set -e


function windows() {
  BASH_EXE_PATH=~/bin/my-reboot.exe
  WIN_EXE_PATH=$(cygpath -w $BASH_EXE_PATH)

  windows_prepare_executable
  windows_create_shortcuts
  windows_set_display_switch_run_on_startup
}


function windows_prepare_executable() {
  echo "* Compilando..."
  cargo build --release

  echo "* Copiando my-reboot.exe para $(dirname $BASH_EXE_PATH)"
  cp ./target/release/my-reboot.exe $BASH_EXE_PATH
  echo
}


function windows_create_shortcuts() {
  echo "* Criando atalho para My Reboot na Área de Trabalho..."
  cscript ./scripts/CreateMyRebootShortcut.vbs "$WIN_EXE_PATH"

  echo "* Criando atalho para Trocar de Tela na Área de Trabalho..."
  cscript ./scripts/CreateDisplaySwitchShortcut.vbs "$WIN_EXE_PATH" "$(cygpath -w $(which DisplaySwitch.exe))"
}


function windows_set_display_switch_run_on_startup() {
  echo "* Registrando troca de tela ao iniciar o Windows..."
  REG ADD 'HKCU\Software\Microsoft\Windows\CurrentVersion\Run' \
    /v 'My Reboot Display Switch' \
    /d "\"$WIN_EXE_PATH\" switch:saved" \
    /f
}


case "$(uname -o)" in
  "Cygwin")
    windows
    ;;
esac

echo
echo "* Instalação concluída"
echo "Execute:"
echo "  my-reboot configure"
echo "para configurar."
