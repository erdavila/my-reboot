import { OSProvider } from "./os-provider";
import { OperatingSystem, operatingSystemText, State, WindowsDisplay, windowsDisplayText } from "./state";
import chalk = require("chalk");

export const NEXT_BOOT_OPERATING_SYSTEM_SENTENCE = "Sistema operacional a ser iniciado na próxima inicialização do computador";
export const NEXT_WINDOWS_BOOT_DISPLAY_SENTENCE = "Tela a ser usada na próxima inicialização do Windows";

const RebootActions = ["reboot", "shutdown"] as const;
type RebootAction = typeof RebootActions[number];
export const REBOOT_ACTIONS: ReadonlyArray<RebootAction> = RebootActions;
export interface Script {
    readonly nextBootOperatingSystem?: OperatingSystem | "unset";
    readonly nextWindowsBootDisplay?: WindowsDisplay | "unset";
    readonly rebootAction?: RebootAction;
}

export class ScriptExecutor {
  private state: State | undefined = undefined;

  static get(): ScriptExecutor {
    return new ScriptExecutor();
  }

  async execute(script: Script) {
    await this.updateStateWith(
      script.nextBootOperatingSystem,
      state => state.setOperatingSystem,
      state => state.unsetOperatingSystem,
      os => `${NEXT_BOOT_OPERATING_SYSTEM_SENTENCE} foi atualizado para ${operatingSystemText(os)}.`,
    );

    await this.updateStateWith(
      script.nextWindowsBootDisplay,
      state => state.setWindowsDisplay,
      state => state.unsetWindowsDisplay,
      display => `${NEXT_WINDOWS_BOOT_DISPLAY_SENTENCE} foi atualizada para ${windowsDisplayText(display)}.`,
    );

    if (script.rebootAction) {
      switch (script.rebootAction) {
        case "reboot":
          await this.doRebootAction(osProvider => osProvider.reboot, "Reiniciando...");
          break;
        case "shutdown":
          await this.doRebootAction(osProvider => osProvider.shutdown, "Desligando...");
          break;
      }
    }
  }

  private async updateStateWith<T>(
    value: T | "unset" | undefined,
    set: (state: State) => (value: T) => Promise<void>,
    unset: (state: State) => () => Promise<void>,
    message: (value: T | undefined) => string,
  ): Promise<void> {
    if (value) {
      const state = await this.getState();
      if (value === "unset") {
        await unset(state).call(state);
      } else {
        await set(state).call(state, value);
      }
      console.log(message(value === "unset" ? undefined : value));
    }
  }

  private async getState(): Promise<State> {
    if (!this.state) {
      const osProvider = await OSProvider.get();
      this.state = new State(osProvider.stateDir);
    }
    return this.state;
  }

  private async doRebootAction(
    action: (osProvider: OSProvider) => () => Promise<void>,
    message: string,
  ): Promise<void> {
    console.log(message);
    if (process.env["NO_REBOOT_ACTION"] === 'true') {
      console.log(chalk.yellow("...mas não de verdade!"), "😬");
    } else {
      const osProvider = await OSProvider.get();
      await action(osProvider).call(osProvider);
    }
  }
}
