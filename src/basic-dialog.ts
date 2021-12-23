import { app, BrowserWindow, ipcMain, Size } from "electron";
import { OSProvider } from "./os-provider";
import { ScriptExecutor } from "./script";
import * as path from "path";

export function showBasicDialog(osProvider: OSProvider) {
  ipcMain.handleOnce('get-button-labels', () =>
    osProvider.predefinedScripts.map(ps => ps.buttonLabel)
  );

  ipcMain.once('basic-mode-button-click', async (_event, index: number) => {
    const script = osProvider.predefinedScripts[index]?.script;
    if (script === undefined) {
      throw new Error("Invalid index");
    }
    await ScriptExecutor.get().execute(script);
    app.quit();
  });

  const asset = (file: string) => path.join(__dirname, file);

  const win = new BrowserWindow({
    width: 300,
    height: 100,
    center: true,
    resizable: false,
    fullscreenable: false,
    icon: asset(osProvider.icon),
    // TODO: Consider on Windows: titleBarStyle
    webPreferences: {
      preload: path.join(__dirname, 'basic-dialog-preload.js'),
      enablePreferredSizeMode: true,
    },
  });

  win.loadFile(asset('basic-dialog.html'));
  win.removeMenu();
  // win.webContents.openDevTools();
  win.webContents.on('preferred-size-changed', (_event, preferredSize: Size) => {
    win.setBounds({ height: preferredSize.height });
    win.center();
  });
}
