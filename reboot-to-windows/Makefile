NAME=reboot-to-windows

EXE=$(NAME)
DESKTOP=$(NAME).desktop
ICON=windows10logo.png

INSTALL_DIR=$(HOME)/bin
INSTALLED_EXE=$(abspath $(INSTALL_DIR)/$(EXE))
INSTALLED_ICON=$(abspath $(INSTALL_DIR)/$(ICON))


.PHONY: all
all: $(EXE) $(DESKTOP)

$(EXE): main.c
	$(CC) -Wall $< -o $@
	sudo chown root $@
	sudo chmod u+s $@

.PHONY:
install: $(INSTALLED_EXE) $(INSTALLED_ICON) shortcut

$(INSTALLED_EXE): $(EXE)
	cp -p $< $@
	sudo chown root $@
	sudo chmod u+s $@

$(INSTALLED_ICON): $(ICON)
	cp -p $< $@

.PHONY: shortcut
shortcut: $(DESKTOP)
	xdg-desktop-menu install --novendor $<
	@echo
	@echo 'ATENÇÃO: se a entrada do menu não'
	@echo '  aparecer mesmo após reiniciar a sessão,'
	@echo '  ela deverá ser criada manualmente. :-/'

$(DESKTOP): $(DESKTOP).template
	sed \
		-e s:@ICON@:$(INSTALLED_ICON): \
		-e s:@EXEC@:$(INSTALLED_EXE): \
		$< > $@

.PHONY: uninstall
uninstall: $(DESKTOP)
	rm -rf $(INSTALLED_EXE) $(INSTALLED_ICON)
	xdg-desktop-menu uninstall --novendor $<

.PHONY: clean
clean:
	rm -rf $(EXE) $(DESKTOP)
