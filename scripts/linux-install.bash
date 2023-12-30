EXE_NAME=my-reboot
DEST_DIR=~/Apps/my-reboot

function install() {
  prepare_executable
  create_link
  install_launching_icon
}

function create_link() {
  DEST_PATH=~/bin
  echo "* Criando link para $EXE_NAME em $DEST_PATH ..."
  ln -sf "$EXE_PATH" "$DEST_PATH"
  echo
}

function install_launching_icon() {
  local ICON_FILE='256x256.png'
  local DESKTOP_FILE_PATH="$DEST_DIR"/my-reboot.desktop
  echo "* Instalando ícone..."
  cp "$ICON_FILE" "$DEST_DIR"
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
