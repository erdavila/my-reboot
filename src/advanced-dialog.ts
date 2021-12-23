import { OSProvider } from "./os-provider";
import { BrowserWindow, Size } from "electron";
import * as path from "path";

export function showAdvancedDialog(osProvider: OSProvider) {
  const asset = (file: string) => path.join(__dirname, file);

  const win = new BrowserWindow({
    width: 300,
    height: 300,
    center: true,
    resizable: false,
    fullscreenable: false,
    icon: asset(osProvider.icon),
    // TODO: Consider on Windows: titleBarStyle
    webPreferences: {
      // preload: path.join(__dirname, 'basic-dialog-preload.js'),
      enablePreferredSizeMode: true,
    },
  });

  win.loadFile(asset('advanced-dialog.html'));
  win.removeMenu();
  // win.webContents.openDevTools();
  win.webContents.on('preferred-size-changed', (_event, preferredSize: Size) => {
    win.setBounds({ height: preferredSize.height });
  });
}
