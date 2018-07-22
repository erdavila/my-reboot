EXE=reboot-to-linux.exe
GRUBENV_DIR=/c/grubenv.dir
INSTALL_DIR=$(HOME)/bin
INSTALLED_EXE=$(abspath $(INSTALL_DIR)/$(EXE))
SHORTCUT=Reiniciar no Linux

.PHONY: all
all: $(EXE)

$(EXE): main.c
	$(CC) -Wall -mwindows -DGRUBENV_DIR='"$(GRUBENV_DIR)"' $< -o $@

.PHONY: install
install: $(INSTALLED_EXE) shortcut

$(INSTALLED_EXE): $(EXE)
	mkdir -p $(INSTALL_DIR)
	cp -p $< $@

.PHONY: shortcut
shortcut: $(EXE)
	mkshortcut \
		--desc 'Reiniciar no Linux' \
		--name '$(SHORTCUT)' \
		--desktop \
		--icon Linux.ico \
		--workingdir=/bin \
		$(INSTALLED_EXE)
	@echo
	@echo 'Atalho criado na Área de Trabalho'
	@echo 'Para fixá-lo no menu Iniciar, clique com o'
	@echo 'botão direito e selecione "Fixar em Iniciar"'

.PHONY: uninstall
uninstall:
	rm -rf $(INSTALLED_EXE) "$(shell cygpath -D)/$(SHORTCUT).lnk"

.PHONY: clean
clean:
	rm -rf $(EXE)
