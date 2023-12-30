EXE_NAME=my-reboot.exe
DEST_DIR=~/bin

function install() {
  prepare_executable

  WIN_EXE_PATH=$(cygpath -w "$EXE_PATH")
  create_shortcuts
  set_display_switch_run_on_startup
  echo
}

function create_shortcuts() {
  echo "* Criando atalho para My Reboot na Área de Trabalho..."
  cscript ./scripts/CreateMyRebootShortcut.vbs "$WIN_EXE_PATH"

  echo "* Criando atalho para Trocar de Tela na Área de Trabalho..."
  cscript ./scripts/CreateDisplaySwitchShortcut.vbs "$WIN_EXE_PATH" "$(cygpath -w "$(which DisplaySwitch.exe)")"
}

function set_display_switch_run_on_startup() {
  echo "* Registrando troca de tela ao iniciar o Windows..."
  REG ADD 'HKCU\Software\Microsoft\Windows\CurrentVersion\Run' \
    /v 'My Reboot Display Switch' \
    /d "\"$WIN_EXE_PATH\" switch:saved" \
    /f
}
