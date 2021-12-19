import { app, ipcMain, BrowserWindow, Event, Size } from 'electron';
import { OSProvider } from './os-provider';
import { Script, ScriptExecutor } from './script';
import { State } from './state';
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

    case "show": {
      const arg = args.shift();
      if (arg !== undefined) {
        exceedingArgument(arg);
      }
      showState();
      break;
    }

    default:
      handleScriptArguments(arg, args);
  }
}

function showState() {
  OSProvider.get().then(async osProvider => {
    const state = new State(osProvider.stateDir);
    const os = await state.getOperatingSystem();
    const display = await state.getWindowsDisplay();
    // TODO: identify current Window display

    console.log("S.O. a ser iniciado na próxima inicialização do computador:");
    console.log(" ", os !== undefined ? os : "indefinido");
    console.log();
    console.log("Tela a ser usada na próxima inicialização do Windows:");
    console.log(" ", display !== undefined ? display : "indefinida");

    process.exit()
  })
}

function handleScriptArguments(arg: string | undefined, args: string[]) {
  let script: Script = {};

  const OS_PREFIX = 'os:'
  if (arg === 'windows' || arg === 'linux') {
    arg = `${OS_PREFIX}${arg}`;
  }
  if (arg?.startsWith(OS_PREFIX)) {
    const os = arg!.substring(OS_PREFIX.length);
    switch (os) {
      case 'windows':
      case 'linux':
      case 'unset':
        script = { ...script, nextBootOperatingSystem: os };
        arg = args.shift();
        break;
      default:
        throw new ArgumentError("Sistema operacional inválido", arg);
    }
  }

  const DISPLAY_PREFIX = 'display:'
  if (arg === 'monitor' || arg === 'tv') {
    arg = `${DISPLAY_PREFIX}${arg}`;
  }
  if (arg?.startsWith(DISPLAY_PREFIX)) {
    const display = arg.substring(DISPLAY_PREFIX.length);
    switch (display) {
      case 'monitor':
      case 'tv':
      case 'unset':
        script = { ...script, nextWindowsBootDisplay: display };
        arg = args.shift();
        break;
      default:
        throw new ArgumentError("Tela inválida", arg);
    }
  }

  switch (arg) {
    case 'reboot':
    case 'shutdown':
      script = { ...script, rebootAction: arg };
      arg = args.shift();
      break;
  }

  if (arg) {
    unknownArgument(arg);
  }

  ScriptExecutor.get().execute(script).then(() => {
    process.exit(0);
  });
}

function showUsage(out: typeof console.log = console.log) {
  out("Usos:");
  out("  my-reboot");
  out("  my-reboot dialog");
  out("    Exibe diálogo básico.");
  out();
  out("  my-reboot [SO] [TELA] [AÇÃO]");
  out("    SO poder ser:");
  out("      [os:]windows - Inicia Windows na próxima inicialização do computador.");
  out("      [os:]linux - Inicia Linux na próxima inicialização do computador.");
  out("      os:unset - Deixa o Grub decidir o S.O. na próxima inicialização do computador.");
  out();
  out("    TELA poder ser:");
  out("      [display:]monitor - Usa o monitor na próxima inicialização do Windows.");
  out("      [display:]tv - Usa a TV na próxima inicialização do Windows.");
  out("      display:unset - Deixa o Windows decidir a tela na próxima inicialização do Windows.");
  out();
  out("    AÇÃO poder ser:");
  out("      reboot - Reinicia o computador.");
  out("      shutdown - Desliga o computador.");
  out();
  out("  my-reboot show");
  out("    Exibe as opções atuais para inicialização.");
  out();
  out("  my-reboot -h|--help");
  out("    Exibe este conteúdo.");
}

function exceedingArgument(arg: string): never {
  throw new ArgumentError("Argumento em excesso", arg);
}

function unknownArgument(arg: string): never {
  throw new ArgumentError("Argumento inesperado", arg);
}

function basicDialog() {
  const createBasicWindow = (osProvider: OSProvider) => {
    ipcMain.handleOnce('get-button-labels', () => {
      return osProvider.buttons.map(x => x.label);
    });

    ipcMain.once('basic-mode-button-click', async (_event, index: number) => {
      const script = osProvider.buttons[index]!.script;
      await ScriptExecutor.get().execute(script);
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


  Promise.all([app.whenReady(), OSProvider.get()]).then(([_, provider]) => {
    createBasicWindow(provider);
  });
}
