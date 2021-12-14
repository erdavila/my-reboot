import { app, BrowserWindow, Event, Size } from 'electron';

const createBasicWindow = () => {
  const win = new BrowserWindow({
    width: 300,
    height: 100,
    center: true,
    resizable: false,
    fullscreenable: false,
    icon: process.platform === 'win32' ? 'icon.ico' : 'icon.png',
    // Consider on Windows: titleBarStyle
    webPreferences: {
        enablePreferredSizeMode: true,
    },
  });

  win.loadFile('../basic.html');
  win.removeMenu();
  win.webContents.openDevTools();
  win.webContents.on('preferred-size-changed', (event: Event, preferredSize: Size) => {
    win.setBounds({ height: preferredSize.height });
  });
};

app.whenReady().then(() => {
  createBasicWindow();
});
