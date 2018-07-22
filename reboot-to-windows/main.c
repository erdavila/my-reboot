#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#define QUESTION "Deseja reiniciar no Windows?"

static bool ask() {
	int ret = system(
	  "zenity"
	    " --question"
	    " --text '" QUESTION "'"
	    " --title '" QUESTION "'"
	    " --window-icon=windows10logo.png"
  );
	return (ret == 0);
}

static int get_linux_entry_number() {
	FILE* p = popen("grep ^menuentry /boot/grub/grub.cfg | grep 'Windows Boot Manager' -n | cut -d : -f 1", "r");
	char buffer[10];
	fread(buffer, sizeof(buffer), 1, p);
	pclose(p);
	return atoi(buffer);
}


static void rewrite_grubenv() {
	int n = get_linux_entry_number();
	char cmd[] = "grub-editenv - set saved_entry=\0##########";
	sprintf(cmd + strlen(cmd), "%d", n);
	printf("Executing: %s\n", cmd);
	system(cmd);
}

static void reboot() {
	system("reboot");
}

int main(int argc, char **argv) {
	setuid(0); setgid(0);

	bool yes = ask();
	if (yes) {
		rewrite_grubenv();
		reboot();
	}
	return !yes;
}

