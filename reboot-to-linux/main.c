#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <windows.h>

#define GRUBENV_SIZE 1024

#define starts_with(s, p) (strncmp(s, p, strlen(p)) == 0)

static bool ask() {
	const char* question = "Deseja reiniciar no Linux?";
	int ret = MessageBox(NULL, question, question, MB_ICONQUESTION | MB_YESNO);
	return (ret == IDYES);
}

static void rewrite_grubenv() {
	FILE* f = fopen(GRUBENV_DIR "/grubenv", "r+");
	if (!f) {
		perror("Can't read file " GRUBENV_DIR "/grubenv");
		exit(1);
	}

	char output[GRUBENV_SIZE + 1];
	output[0] = '\0';

	char line[GRUBENV_SIZE + 1];
	while(fgets(line, GRUBENV_SIZE + 1, f)) {
		if (!starts_with(line, "saved_entry=")) {
			strcat(output, line);
		}
	}

	size_t len = strlen(output);
	memset(output + len, '#', GRUBENV_SIZE - len);
	rewind(f);
	size_t w = fwrite(output, GRUBENV_SIZE, 1, f);
	if (w != 1) {
		perror("Can't write file " GRUBENV_DIR "/grubenv");
		exit(1);
	}

	fclose(f);
}

static void reboot() {
	system("/c/Windows/system32/shutdown /r /t 0");
}

int main(int argc, char **argv) {
	bool yes = ask();
	if (yes) {
		rewrite_grubenv();
		reboot();
	}
	return !yes;
}
