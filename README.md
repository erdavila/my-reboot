# My Reboot

## No Windows
```bash
yarn
yarn build
make -C get_active_display_device_id
```
Garantir que o Windows consegue executar `get_active_display_device_id.exe` normalmente. Se for compilado no Cygwin, o resultado de `cygpath -w /usr/bin` deve ser incluído no `PATH` do Windows. Testar no Command Prompt do Windows.

```bash
# Empacotamento
yarn dist

# Instalação
dist/my-reboot\ Setup\ 2.0.0.exe

# Configuração
/d/Users/erdavila/AppData/Local/Programs/my-reboot/my-reboot.exe -- configure
```

## No Linux
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
