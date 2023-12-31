# My Reboot

## Instalação
### No Windows
Executar `./install.sh` no Cygwin para instalar o binário `my-reboot.exe` em `~/bin`.

### No Linux
Executar `./install.sh` para instalar arquivos em `~/Apps/my-reboot` e criar link simbólico em `~/bin`.

## Configuração
Após a instalação em cada sistema operacional, executar `my-reboot configure`.

## Desenvolvimento
[Diagrama](https://docs.google.com/drawings/d/1_oHqDAIFwJa26Q6RdgqlxOEqs1K9uVLITYUuf2e6iwE/edit?usp=sharing)

```bash
cargo run -- PARÂMETROS # Executa
./pre-commit.sh # Roda reformatação, análise estática e testes

# Utilitários no Windows
cargo run -p enum_display_devices
cargo run -p is_windows_11_or_greater
```

## Referências
* [Rust Standard Library](https://doc.rust-lang.org/std/index.html)
* [iced](https://iced.rs/)
* [EnumDisplayDevices (Win32 API)](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumdisplaydevicesa)
* [DisplaySwitch.exe](https://ss64.com/nt/displayswitch.html)
