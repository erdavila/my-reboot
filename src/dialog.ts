import { BrowserWindow, Size } from "electron";
import * as path from "path";
import { OSProvider } from "./os-provider";

export function showDialog(osProvider: OSProvider, options: { width: number, filePrefix: string }) {
  const asset = (file: string) => path.join(__dirname, file);

  const win = new BrowserWindow({
    width: options.width,
    height: 100,
    center: true,
    resizable: false,
    fullscreenable: false,
    icon: asset(osProvider.icon),
    type: 'toolbar',
    webPreferences: {
      preload: path.join(__dirname, `${options.filePrefix}-preload.js`),
      enablePreferredSizeMode: true,
    },
  });

  win.loadFile(asset(`${options.filePrefix}.html`));
  win.removeMenu();
  // win.webContents.openDevTools();
  win.webContents.on('preferred-size-changed', (_event, preferredSize: Size) => {
    win.setBounds({ height: preferredSize.height + EXTRA_HEIGHT });
    win.center();
  });
}

const EXTRA_HEIGHT = process.platform == 'win32' ? 29 : 0;
