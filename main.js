const { app, BrowserWindow } = require('electron');

const createBasicWindow = () => {
  const win = new BrowserWindow({
    width: 300,
    height: 100,
    center: true,
    resizable: false,
    fullscreenable: false,
    icon: process.platform === 'windows' ? 'icon.ico' : 'icon.png',
    // Consider on Windows: titleBarStyle
    webPreferences: {
        enablePreferredSizeMode: true,
    },
  });

  win.loadFile('basic.html');
  win.removeMenu();
  win.webContents.openDevTools();
  win.webContents.on('preferred-size-changed', (event, preferredSize) => {
    console.log("Setting height:", preferredSize.height);
    win.setBounds({ height: preferredSize.height });
  });
};

app.whenReady().then(() => {
  createBasicWindow();
});
