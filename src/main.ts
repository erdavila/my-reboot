import { app, ipcMain, BrowserWindow, Event, Size } from 'electron';
import { OSProvider } from './os-provider';
import * as path from "path";

const provider = import(`./${process.platform}-provider`);

const createBasicWindow = (osProvider: OSProvider) => {
  ipcMain.handleOnce('get-button-labels', () => {
    return osProvider.buttons.map(x => x.label);
  });

  ipcMain.once('basic-mode-button-click', async (_event, index: number) => {
    await osProvider.buttons[index].clicked();
    app.quit();
  });

  const win = new BrowserWindow({
    width: 300,
    height: 100,
    center: true,
    resizable: false,
    fullscreenable: false,
    icon: osProvider.icon,
    // TODO: Consider on Windows: titleBarStyle
    webPreferences: {
      preload: path.join(__dirname, 'basic-preload.js'),
      enablePreferredSizeMode: true,
    },
  });

  win.loadFile('../basic.html');
  win.removeMenu();
  // win.webContents.openDevTools();
  win.webContents.on('preferred-size-changed', (_event: Event, preferredSize: Size) => {
    win.setBounds({ height: preferredSize.height });
  });
};


Promise.all([app.whenReady(), provider]).then(([_, provider]) => {
  createBasicWindow(provider.default);
});
