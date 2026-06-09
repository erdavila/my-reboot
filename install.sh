#!/bin/bash
set -e

EXE_NAME=my-reboot
ICON_NAME=$EXE_NAME

function prepare_executable() {
  echo "* Compilando..."
  cargo install --path .
  echo
}

function install_launching_icon() {
  echo "* Instalando ícone no menu..."

  xdg-icon-resource install --novendor --mode user --size 256 assets/icon-256x256.png $ICON_NAME

  local DESKTOP_FILE_PATH=$(mktemp --directory)/my-reboot.desktop
  echo "[Desktop Entry]
Version=3.0.0
Type=Application
Name=My Reboot
Icon=$ICON_NAME
Exec=$EXE_NAME
Comment=Reboot options
Categories=System;Utility;
Terminal=false
" >$DESKTOP_FILE_PATH
  xdg-desktop-menu install --novendor --mode user $DESKTOP_FILE_PATH

  echo
}

if [ "$(uname -o)" != "GNU/Linux" ]; then
  echo 'Sistema operacional não suportado.'
  echo 'No Windows, execute .\install.ps1.'
  exit 1
fi >&2

prepare_executable
install_launching_icon

echo "* Instalação concluída"
echo
echo "Execute:"
echo "  my-reboot configure"
echo "para configurar."
