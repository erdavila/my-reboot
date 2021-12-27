import { app, BrowserWindow } from 'electron';
import { OSProvider } from './os-provider';
import { NEXT_BOOT_OPERATING_SYSTEM_SENTENCE, NEXT_WINDOWS_BOOT_DISPLAY_SENTENCE, Script, ScriptExecutor } from './script';
import { operatingSystemText, State, displayText, DISPLAYS } from './state';
import { showBasicDialog } from './basic-dialog';
import { showAdvancedDialog } from './advanced-dialog';
import { ExecuteScriptMessage, ReplaceDialogMessage } from './messages';
import { linuxConfigure, windowsConfigure } from './configure';

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
      showDialog({ advanced: false });
      break;

    case "dialog": {
      const arg = args.shift();
      let advanced: boolean;

      switch (arg) {
        case undefined:
          advanced = false;
          break;
        case "-x": {
          advanced = true;
          noMoreArguments(args);
          break;
        }
        default:
          unknownArgument(arg);
      }

      showDialog({ advanced });
      break;
    }

    case "script": {
      const arg = args.shift();
      if (arg === undefined) {
        missingArgument("NÚMERO");
      } else if (!arg.match(/^\d+$/)) {
        invalidArgument(arg);
      } else {
        const num = parseInt(arg);
        noMoreArguments(args);
        OSProvider.get().then(async osProvider => {
          const script = osProvider.predefinedScripts[num - 1]?.script;
          if (script === undefined) {
            throw new ArgumentError(
              `Número inválido de script para o sistema operacional atual (mín: 1; máx: ${osProvider.predefinedScripts.length})`,
              num.toString(),
            );
          } else {
            await ScriptExecutor.get().execute(script);
            process.exit(0);
          }
        });
      }
      break;
    }

    case "show": {
      noMoreArguments(args);
      showState();
      break;
    }

    case "configure": {
      switch (process.platform) {
        case 'linux':
          noMoreArguments(args);
          linuxConfigure().then(() => process.exit());
          break;
        case 'win32': {
          const arg = args.shift();
          const initialDisplay = DISPLAYS.find(display => display == arg);
          if (initialDisplay === undefined  &&  arg !== undefined) {
            exceedingArgument(arg);
          }
          noMoreArguments(args);
          windowsConfigure(initialDisplay).then(() => process.exit());
          break;
        }
      }
      break;
    }

    default:
      handleScriptArguments(arg, args);
  }
}

function showState() {
  OSProvider.get().then(async osProvider => {
    const state = new State(osProvider.stateDir);
    const stateValues = await state.getValues();

    console.log(`${NEXT_BOOT_OPERATING_SYSTEM_SENTENCE}:`, operatingSystemText(stateValues.nextBootOperatingSystem));
    console.log(`${NEXT_WINDOWS_BOOT_DISPLAY_SENTENCE}:`, displayText(stateValues.nextWindowsBootDisplay));
    if (stateValues.currentDisplay) {
      console.log("Tela atual:", displayText(stateValues.currentDisplay));
    }

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
    const os = arg.substring(OS_PREFIX.length);
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

  const SWITCH_PREFIX = 'switch:';
  if (arg === 'switch') {
    arg = `${SWITCH_PREFIX}other`;
  }
  if (arg?.startsWith(SWITCH_PREFIX)) {
    const switchOption = arg.substring(SWITCH_PREFIX.length);
    switch (switchOption) {
      case 'other':
      case 'monitor':
      case 'tv':
      case 'saved':
        script = { ...script, switchToDisplay: switchOption };
        arg = args.shift();
        break;
      default:
        throw new ArgumentError("Parâmetro inválido", arg);
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
  out("  my-reboot dialog -x");
  out("    Exibe diálogo avançado.");
  out();
  out("  my-reboot [SO] [TELA] [TROCA-DE-TELA] [AÇÃO]");
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
  out("    TROCA-DE-TELA poder ser (somente no Windows):");
  out("      switch[:other] - Troca para a outra tela.");
  out("      switch:monitor - Troca para o monitor.");
  out("      switch:tv - Troca para a TV.");
  out("      switch:saved - Troca para a tela definida para ser usada na próxima inicialização do Windows.");
  out();
  out("    AÇÃO poder ser:");
  out("      reboot - Reinicia o computador.");
  out("      shutdown - Desliga o computador.");
  out();
  out("  my-reboot show");
  out("    Exibe as opções atuais para inicialização.");
  out();
  out("  my-reboot script NÚMERO");
  out("    Executa o script correspondente às ações disponíveis no diálogo básico do S.O. atual.");
  out();
  out("  my-reboot configure");
  out("    Configura. Deve ser executado no Linux e no Windows ao menos uma vez.");
  out();
  out("  my-reboot -h|--help");
  out("    Exibe este conteúdo.");
}

function noMoreArguments(args: string[]) {
  const arg = args.shift();
  if (arg !== undefined) {
    exceedingArgument(arg);
  }
}

function exceedingArgument(arg: string): never {
  throw new ArgumentError("Argumento em excesso", arg);
}

function unknownArgument(arg: string): never {
  throw new ArgumentError("Argumento inesperado", arg);
}

function missingArgument(arg: string): never {
  throw new ArgumentError("Argumento faltando", arg);
}

function invalidArgument(arg: string): never {
  throw new ArgumentError("Argumento inválido", arg);
}

function showDialog(options: { advanced: boolean }) {
  Promise.all([OSProvider.get(), app.whenReady()]).then(([osProvider]) => {
    function show(advanced: boolean) {
      if (advanced) {
        showAdvancedDialog(osProvider);
      } else {
        showBasicDialog(osProvider);
      }
    }

    ReplaceDialogMessage.receive(options => {
      BrowserWindow.getAllWindows().forEach(win => {
        win.close();
      });

      show(options.advanced);
    });

    ExecuteScriptMessage.receive(async script => {
      await ScriptExecutor.get().execute(script);
      app.quit();
    });

    show(options.advanced);
  });
}
