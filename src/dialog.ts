import { BrowserWindow, Size } from "electron";
import * as path from "path";
import { OSProvider } from "./os-provider";

export function showDialog(osProvider: OSProvider, options: { width: number, filePrefix: string }) {
  const asset = (file: string) => path.join(__dirname, 'assets', file);

  const win = new BrowserWindow({
    width: options.width,
    height: 100,
    center: true,
    resizable: false,
    fullscreenable: false,
    icon: asset(osProvider.icon),
    ...(process.platform == 'win32' ? { type: 'toolbar' } : {}),
    show: false,
    webPreferences: {
      preload: path.join(__dirname, `${options.filePrefix}-preload.js`),
      enablePreferredSizeMode: true,
    },
  });

  win.loadFile(asset(`${options.filePrefix}.html`));
  win.removeMenu();
  // win.webContents.openDevTools({ mode: "undocked", activate: false });

  win.webContents.on('preferred-size-changed', (_event, preferredSize: Size) => {
    win.setBounds({ height: preferredSize.height + EXTRA_HEIGHT });
    win.center();
  });

  win.once('ready-to-show', () => {
    win.show();
  });
}

const EXTRA_HEIGHT = process.platform == 'win32' ? 29 : 0;
