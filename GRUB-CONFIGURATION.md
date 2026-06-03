# GRUB Configuration

The GRUB configuration includes setting a disk partition for the [GRUB environment block] to be
visible by Windows.

## GRUB configuration to remember the last operating system

- Edit the file `/etc/default/grub`, definining the values (insert a new line if needed):
    - `GRUB_DEFAULT=saved`
    - `GRUB_SAVEDEFAULT=true`
    - `GRUB_TIMEOUT=2` (optional)

- Execute:

    ```bash
    sudo update-grub
    ```

- Test it:
    1. Reboot; select Windows on the GRUB menu
    2. Reboot; check if Windows is pre-selected on the GRUB menu.


## The shared `grubenv` partition

Define a partition for the [GRUB environment block]. It can be as small as 100MiB and formatted with the FAT file system.

Use some Linux disk management tool to identify the partion UUID and take note of it.

Also, identify the GRUB-ID for the new partition. This can be done by pressing `c` on the GRUB menu and then
running the command `ls -l -h`.

On the instructions below, replace `<UUID>` and `<GRUB-ID>` with the values you took note of.


## `grubenv` mount point

- Add the line to `/etc/fstab`:

    `UUID=<UUID>  /boot/grub/grubenv.dir  vfat  defaults,umask=0000  0  1`

- Execute:

    ```bash
    sudo mkdir /boot/grub/grubenv.dir
    sudo mount /boot/grub/grubenv.dir
    sudo mv /boot/grub/grubenv /boot/grub/grubenv.dir
    sudo ln -s grubenv.dir/grubenv /boot/grub
    ```

- Test it:

    ```bash
    grub-editenv list
    # The value for `saved_entry` must be displayed without any error
    ```


## Configure the shared `grubenv` on GRUB

### File `/etc/grub.d/00_header`

- Back it up:

    ```bash
    sudo cp /etc/grub.d/00_header{,.bkp}
    sudo chmod a-x /etc/grub.d/00_header.bkp
    ```

- Replace:

    `quick_boot="1"`

    with:

    `quick_boot="0"`

- Replace the block:

    ```bash
    if [ -s \$prefix/grubenv ]; then
      set have_grubenv=true
      load_env
    fi
    ```

    with:

    ```bash
    set have_grubenv=true
    load_env --file (<GRUB-ID>)/grubenv
    ```

- Replace all occurrences of `save_env` with `save_env --file (<GRUB-ID>)/grubenv`


### File `/etc/grub.d/10_linux`

- Back it up:

    ```bash
    sudo cp /etc/grub.d/10_linux{,.bkp}
    sudo chmod a-x /etc/grub.d/10_linux.bkp
    ```

- Replace:

    `quick_boot="1"`

    with:

    `quick_boot="0"`


### Apply the configurations and test it

- Execute:

    ```bash
    sudo update-grub
    ```

- Test it:
    1. Reboot; select Windows on the GRUB menu
    2. Reboot; check if Windows is pre-selected on the GRUB menu.


## Configure the shared `grubenv` on Windows

On the Disk Management tool, map the `grubenv` partitition to the folder `C:\grubenv.dir`.


## Using different paths for the shared `grubenv`

To use a different paths for the shared `grubenv`, use the `STATE_DIR_PATH` environment variable during installation.

On Linux:
```bash
STATE_DIR_PATH=/boot/grub/other-grubenv.dir ./install.sh
```

On Windows (PowerShell):
```powershell
$env:STATE_DIR_PATH = "C:\other-grubenv.dir"; .\install.ps1
```

Also, use the desired path on all the instructions below.


[GRUB environment block]: https://www.gnu.org/software/grub/manual/grub/html_node/Environment-block.html
