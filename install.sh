#!/bin/bash
set -e


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


case "$(uname -o)" in
  "Cygwin")
    . ./scripts/windows-install.bash
    ;;
  "GNU/Linux")
    . ./scripts/linux-install.bash
    ;;
  *)
    echo "Sistema operacional não suportado">&2
    exit 1
esac

install

echo "* Instalação concluída"
echo "Execute:"
echo "  my-reboot configure"
echo "para configurar."
