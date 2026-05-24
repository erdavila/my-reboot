# My Reboot

## Instalação
### No Windows
Executar `./install.sh` no Cygwin para instalar o binário `my-reboot.exe` em `~/bin`.

### No Linux
Executar `./install.sh` para instalar arquivos em `~/Apps/my-reboot` e criar link simbólico em `~/bin`.

## Configuração
Após a instalação em cada sistema operacional, executar `my-reboot configure`.

## Desenvolvimento
Dependência: [`just`](https://just.systems/man/en/installation.html)

```bash
just --list
```

## Referências
* [Rust Standard Library](https://doc.rust-lang.org/std/index.html)
* [iced](https://iced.rs/)
