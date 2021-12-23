import { OSProvider } from "./os-provider";
import { app, BrowserWindow, ipcMain, Size } from "electron";
import * as path from "path";
import { OperatingSystem, State, WindowsDisplay } from "./state";
import { Script, ScriptExecutor } from "./script";

export function showAdvancedDialog(osProvider: OSProvider) {
  ipcMain.handleOnce('get-state', async () => {
    const state = new State(osProvider.stateDir);
    const operatingSystem = await state.getOperatingSystem();
    const display = await state.getWindowsDisplay();
    const values: [OperatingSystem | undefined, WindowsDisplay | undefined] = [operatingSystem, display];
    return values;
  });

  ipcMain.once('execute-script', async (_event, script: Script) => {
    await ScriptExecutor.get().execute(script);
    app.quit();
  })

  const asset = (file: string) => path.join(__dirname, file);

  const win = new BrowserWindow({
    width: 340,
    height: 100,
    center: true,
    resizable: false,
    fullscreenable: false,
    icon: asset(osProvider.icon),
    // TODO: Consider on Windows: titleBarStyle
    webPreferences: {
      preload: path.join(__dirname, 'advanced-dialog-preload.js'),
      enablePreferredSizeMode: true,
    },
  });

  win.loadFile(asset('advanced-dialog.html'));
  win.removeMenu();
  // win.webContents.openDevTools();
  win.webContents.on('preferred-size-changed', (_event, preferredSize: Size) => {
    win.setBounds({ height: preferredSize.height });
    win.center();
  });
}
