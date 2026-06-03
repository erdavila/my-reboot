#!/bin/bash
set -e

EXE_NAME=my-reboot
DEST_DIR=~/Apps/my-reboot

function prepare_executable() {
  echo "* Compilando..."
  cargo build --release
  echo

  echo "* Copiando $EXE_NAME para $DEST_DIR ..."
  mkdir -p "$DEST_DIR"
  cp ./target/release/"$EXE_NAME" "$DEST_DIR"
  echo

  EXE_PATH="$DEST_DIR/$EXE_NAME"
}

function create_link() {
  DEST_PATH=~/bin
  echo "* Criando link para $EXE_NAME em $DEST_PATH ..."
  ln -sf "$EXE_PATH" "$DEST_PATH"
  echo
}

function install_launching_icon() {
  local ICON_FILE='icon-256x256.png'
  local DESKTOP_FILE_PATH="$DEST_DIR"/my-reboot.desktop
  echo "* Instalando ícone..."
  cp assets/"$ICON_FILE" "$DEST_DIR"
  echo "[Desktop Entry]
Version=3.0.0
Type=Application
Name=My Reboot
Icon=$DEST_DIR/$ICON_FILE
Exec=$EXE_PATH
Comment=Reboot options
Categories=System;Utility;
Terminal=false
" >"$DESKTOP_FILE_PATH"

  xdg-desktop-menu install --novendor --mode user $DESKTOP_FILE_PATH
  xdg-desktop-menu forceupdate --mode user
  echo
}

if [ "$(uname -o)" != "GNU/Linux" ]; then
  echo 'Sistema operacional não suportado.'
  echo 'No Windows, execute .\install.ps1.'
  exit 1
fi >&2

prepare_executable
create_link
install_launching_icon

echo "* Instalação concluída"
echo "Execute:"
echo "  my-reboot configure"
echo "para configurar."
