#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <windows.h>

#define GRUBENV_SIZE 1024

#define starts_with(s, p) (strncmp(s, p, strlen(p)) == 0)

static void rewrite_grubenv() {
	printf("> Rewriting grubenv\n");

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
	printf("> Rebooting\n");
	system("/c/Windows/system32/shutdown /r /t 0");
}

static void shutdown() {
	printf("> Shutting down\n");
	system("/c/Windows/system32/shutdown /s /t 0");
}

static void execute_command_before(const char *cmd) {
	printf("> Executing command before\n");
	system(cmd);
}

#define ID_RAD1 500
#define ID_RAD2 501
#define ID_RAD3 502
#define ID_CHK  510

#define MARGIN                 8
#define OPTIONBUTTON_HEIGHT    10
#define OPTIONBUTTON_WIDTH    120
#define OPTIONBUTTONS_SPACING  10
#define PUSHBUTTON_HEIGHT     14
#define PUSHBUTTONS_SPACING    8

static void CenterDialog(HWND hwndDlg) {
	HWND hwndOwner = GetDesktopWindow();
	RECT rcOwner, rcDlg, rc;
	GetWindowRect(hwndOwner, &rcOwner);
	GetWindowRect(hwndDlg, &rcDlg);
	CopyRect(&rc, &rcOwner);

	OffsetRect(&rcDlg, -rcDlg.left, -rcDlg.top);
	OffsetRect(&rc, -rc.left, -rc.top);
	OffsetRect(&rc, -rcDlg.right, -rcDlg.bottom);

	SetWindowPos(hwndDlg,
		HWND_TOP,
		rcOwner.left + (rc.right / 2),
		rcOwner.top + (rc.bottom / 2),
		0, 0, // Ignores size arguments
		SWP_NOSIZE
	);
}

static BOOL CALLBACK DialogProc(
  _In_ HWND	  hwndDlg,
  _In_ UINT	  uMsg,
  _In_ WPARAM wParam,
  _In_ LPARAM lParam
) {
	switch (uMsg)
	{
		case WM_INITDIALOG:
			CenterDialog(hwndDlg);
			CheckDlgButton(hwndDlg, ID_RAD1, BST_CHECKED);
			CheckDlgButton(hwndDlg, ID_CHK, BST_CHECKED);
			return TRUE;

		case WM_COMMAND:
			switch (wParam) {
				case IDOK: {
					int ids[] = { ID_RAD1, ID_RAD2, ID_RAD3 };
					int N = sizeof(ids) / sizeof(ids[0]);
					INT_PTR nResult;
					for (int i = 0; i < N; i++) {
						int id = ids[i];
						if (IsDlgButtonChecked(hwndDlg, id)) {
							nResult = id;
							break;
						}
					}

					nResult |= IsDlgButtonChecked(hwndDlg, ID_CHK) << 16;

					EndDialog(hwndDlg, nResult);
					break;
				}

				case IDCANCEL:
					EndDialog(hwndDlg, IDCANCEL);
					break;
			}
			return FALSE;

		default:
			return FALSE;
	}
}

typedef struct {} *PTR;

#define DEFINE_PUT_FUNCTION(T) \
void put##T(PTR* p, T value) { \
	T** pp = (T**) p; \
	**pp = value; \
	(*pp)++; \
} \

DEFINE_PUT_FUNCTION(BYTE)
DEFINE_PUT_FUNCTION(WORD)
DEFINE_PUT_FUNCTION(DWORD)
DEFINE_PUT_FUNCTION(short)

#undef DEFINE_PUT_FUNCTION
#define put16bit putshort

static void putWCHARs(PTR *p, const char* chars) {
	LPWSTR* pp = (LPWSTR*)p;
	*pp += MultiByteToWideChar(CP_ACP, 0, chars, -1, *pp, 50);
}

static void alignAtDWORD(PTR *p) {
	unsigned int* ui = (unsigned int*)p;
	*ui = (*ui + sizeof(DWORD) - 1) / sizeof(DWORD) * sizeof(DWORD);
}

static void putButton(
	PTR *p, WORD id, const char *title, DWORD style,
	short x, short y, short cx, short cy
) {
	// https://docs.microsoft.com/pt-br/windows/desktop/dlgbox/dlgitemtemplateex
	alignAtDWORD(p);
	putDWORD(p, 0); // helpID
	putDWORD(p, 0); // exStyle
	putDWORD(p, WS_CHILD | WS_VISIBLE | style); // style
	putshort(p, x); // x
	putshort(p, y); // y
	putshort(p, cx); // cx
	putshort(p, cy); // cy
	putDWORD(p, id); // id
	put16bit(p, 0xFFFF); // windowClass[0]
	put16bit(p, 0x0080); // windowClass[1]
	putWCHARs(p, title); // title
	putWORD(p, 0); // extraCount
}

static void putRadioButton(PTR *p, WORD id, const char* title, short y) {
	putButton(
		p, id, title, BS_AUTORADIOBUTTON,
		MARGIN, y, OPTIONBUTTON_WIDTH, OPTIONBUTTON_HEIGHT
	);
}

static void putCheckBox(PTR *p, WORD id, const char *title, short y) {
	putButton(
		p, id, title, BS_AUTOCHECKBOX | WS_TABSTOP,
		MARGIN, y, OPTIONBUTTON_WIDTH, OPTIONBUTTON_HEIGHT
	);
}

static void putPushButton(PTR *p, WORD id, const char* title, DWORD style, short x, short y) {
	putButton(
		p, id, title, style | WS_TABSTOP,
		x, y, (OPTIONBUTTON_WIDTH - PUSHBUTTONS_SPACING) / 2, PUSHBUTTON_HEIGHT
	);
}

typedef enum { DoNothing, RebootLinux, RebootWindows, Shutdown } Action;
typedef struct {
	Action action;
	bool cmdBefore;
} Actions;

static Actions ask(const char *cmd_before, const char *lbl_cmd_before) {
	// https://docs.microsoft.com/pt-br/windows/desktop/dlgbox/dialog-boxes
	HGLOBAL hgbl = GlobalAlloc(GMEM_ZEROINIT, 1024);

	WORD cDlgItems = 5;
	short pushbuttons_top = MARGIN + 3 * OPTIONBUTTONS_SPACING + 8;
	if (cmd_before) {
		cDlgItems++;
		pushbuttons_top += OPTIONBUTTONS_SPACING + 3;
	}

	PTR p = GlobalLock(hgbl);

	// https://docs.microsoft.com/pt-br/windows/desktop/dlgbox/dlgtemplateex
	putWORD(&p, 1); // dlgVer
	putWORD(&p, 0xFFFF); // signature
	putDWORD(&p, 0); // helpID
	putDWORD(&p, 0); // exStyle
	putDWORD(&p, WS_POPUP | WS_BORDER | WS_SYSMENU | DS_MODALFRAME | WS_CAPTION | DS_SHELLFONT); // style
	putWORD(&p, cDlgItems); // cDlgItems
	putshort(&p, 0); // x
	putshort(&p, 0); // y
	putshort(&p, OPTIONBUTTON_WIDTH + 2*MARGIN); // cx
	putshort(&p, pushbuttons_top + PUSHBUTTON_HEIGHT + MARGIN); // cy
	put16bit(&p, 0x0000); // menu
	put16bit(&p, 0x0000); // windowClass
	putWCHARs(&p, "Sair do Windows"); // title[titleLen]
	// https://docs.microsoft.com/pt-br/windows/desktop/dlgbox/about-dialog-boxes#dialog-box-fonts
	putWORD(&p, 8); // pointsize
	putWORD(&p, FW_NORMAL); // weight
	putBYTE(&p, FALSE); // italic
	putBYTE(&p, OEM_CHARSET); // charset
	putWCHARs(&p, "MS Shell Dlg"); // typeface

	putRadioButton(&p, ID_RAD1, "Reiniciar no Linux", MARGIN + 0 * OPTIONBUTTONS_SPACING);
	putRadioButton(&p, ID_RAD2, "Reiniciar o Windows", MARGIN + 1 * OPTIONBUTTONS_SPACING);
	putRadioButton(&p, ID_RAD3, "Desligar o computador", MARGIN + 2 * OPTIONBUTTONS_SPACING);
	if (cmd_before) {
		char title[100] = "Antes: ";
		strcat(title, lbl_cmd_before);
		putCheckBox(&p, ID_CHK, title, MARGIN + 3 * OPTIONBUTTONS_SPACING + 3);
	}
	putPushButton(&p, IDOK, "OK", BS_DEFPUSHBUTTON, MARGIN, pushbuttons_top);
	putPushButton(&p, IDCANCEL, "Cancelar", 0, MARGIN + (OPTIONBUTTON_WIDTH - PUSHBUTTONS_SPACING) / 2 + PUSHBUTTONS_SPACING, pushbuttons_top);

	GlobalUnlock(hgbl);
	LRESULT ret = DialogBoxIndirect(
		NULL,
		(LPDLGTEMPLATE)hgbl,
		NULL,
		(DLGPROC)DialogProc
	);
	GlobalFree(hgbl);

	Actions actions;
	actions.cmdBefore = ret >> 16;
	switch (ret & 0x00FFFF) {
		case ID_RAD1: actions.action = RebootLinux; break;
		case ID_RAD2: actions.action = RebootWindows; break;
		case ID_RAD3: actions.action = Shutdown; break;
		default:      actions.action = DoNothing; break;
	}
	return actions;
}

int main(int argc, char **argv) {
	const char *command_before = NULL;
	const char *label_command_before = NULL;
	if (argc >= 3) {
		command_before = argv[1];
		label_command_before = argv[2];
	}

	Actions actions = ask(command_before, label_command_before);

	if (actions.cmdBefore) {
		execute_command_before(command_before);
	}

	switch (actions.action) {
		case RebootLinux:
			rewrite_grubenv();
			reboot();
			return 0;
		case RebootWindows:
			reboot();
			return 0;
		case Shutdown:
			shutdown();
			return 0;
		default:
			return 1;
	}
}
