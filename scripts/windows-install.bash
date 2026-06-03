EXE_NAME=my-reboot.exe
CARGO_CFG="--config env.WINDOWS_ICON='true'"
DEST_DIR=~/bin
SWITCH_PROFILE_TITLE="Trocar Perfil"

function install() {
  prepare_executable

  WIN_EXE_PATH=$(cygpath -w "$EXE_PATH")
  create_shortcuts
  set_profile_switch_run_on_startup
  echo
}

function create_shortcuts() {
  echo "* Criando atalho para My Reboot na Área de Trabalho..."
  cscript //NoLogo ./scripts/CreateMyRebootShortcut.vbs "$WIN_EXE_PATH"

  echo "* Criando atalho para $SWITCH_PROFILE_TITLE na Área de Trabalho..."
  cscript //NoLogo ./scripts/CreateProfileSwitchShortcut.vbs "$WIN_EXE_PATH" "$SWITCH_PROFILE_TITLE" "$(cygpath -w "$(which DisplaySwitch.exe)")"
}

function set_profile_switch_run_on_startup() {
  echo "* Registrando troca de perfil ao iniciar o Windows..."
  REG ADD 'HKCU\Software\Microsoft\Windows\CurrentVersion\Run' \
    /v 'My Reboot Profile Switch' \
    /d "\"$WIN_EXE_PATH\" switch:saved" \
    /f
}
