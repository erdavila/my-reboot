import { app, ipcMain, BrowserWindow, Event, Size } from 'electron';
import { OSProvider } from './os-provider';
import * as path from "path";

class ArgumentError extends Error {
  constructor(message: string, arg: string) {
    super(`${message}: ${arg}`);
    Object.setPrototypeOf(this, ArgumentError.prototype);
  }
}

process.on('uncaughtException', error => {
  if (error instanceof ArgumentError) {
    console.error(error.message);
    console.error();
    showUsage(console.error);
  } else if (error instanceof Error) {
    console.error(error.message);
    console.error(error.stack);
  } else {
    console.error("Exceção não tratada:", error);
  }

  app.quit();
  app.exit(1);
  process.exit(1);
});

handleArguments(process.argv.slice(2));

function handleArguments(args: string[]) {
  if (args.find(arg => arg == '-h' || arg == '--help')) {
    showUsage();
    process.exit();
  }

  const arg = args.shift();
  switch (arg) {
    case undefined:
      basicDialog();
      break;

    case "dialog": {
      const arg = args.shift();
      switch (arg) {
        case undefined:
          basicDialog()
          break;

        default:
          exceedingArgument(arg);
      }
      break;
    }

    default:
      unknownArgument(arg);
  }
}

function showUsage(out: typeof console.log = console.log) {
  out("Usos:");
  out("  my-reboot");
  out("  my-reboot dialog");
  out("    Exibe diálogo básico");
  out();
  out("  my-reboot -h|--help");
  out("    Exibe este conteúdo");
}

function exceedingArgument(arg: string): never {
  throw new ArgumentError("Argumento em excesso", arg);
}

function unknownArgument(arg: string): never {
  throw new ArgumentError("Argumento inesperado", arg);
}

function basicDialog() {
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
}
