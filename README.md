# My Reboot

## Instalação
### No Windows
```bash
yarn
yarn build
make -C get_active_display_device_id
```
Garantir que o Windows consegue executar `get_active_display_device_id/get_active_display_device_id.exe` normalmente. Se for compilado no Cygwin, o resultado de `cygpath -w /usr/bin` deve ser incluído no `PATH` do Windows. Testar no Command Prompt do Windows.

```bash
# Empacotamento
yarn dist

# Instalação
dist/my-reboot\ Setup\ 2.0.0.exe

# Configuração
/d/Users/erdavila/AppData/Local/Programs/my-reboot/my-reboot.exe -- configure
```

### No Linux
```bash
yarn
yarn build

# Empacotamento
yarn dist

# Instalação
sudo dpkg -i dist/my-reboot_2.0.0_amd64.deb

# Configuração
my-reboot -- configure
```

## Desenvolvimento
[Diagrama](https://docs.google.com/drawings/d/1_oHqDAIFwJa26Q6RdgqlxOEqs1K9uVLITYUuf2e6iwE/edit?usp=sharing)

```bash
yarn  # Baixa dependências
yarn build  # Compila (gera *.js)
yarn start PARÂMETROS  # Executa
yarn lint [--fix]  # Analisa código
```

## Referências
* [Node.js](https://nodejs.org/dist/latest-v16.x/docs/api/)
* [Electron](https://www.electronjs.org/docs/latest/)
* [Electron Builder](https://www.electron.build/)
* [EnumDisplayDevices (Win32 API)](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumdisplaydevicesa)
* [DisplaySwitch.exe](https://ss64.com/nt/displayswitch.html)
