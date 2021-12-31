import { OSProvider } from "./os-provider";
import { OperatingSystem, operatingSystemText, State, Display, displayText } from "./state";
import chalk = require("chalk");

export const NEXT_BOOT_OPERATING_SYSTEM_SENTENCE = "Sistema operacional a ser iniciado na próxima inicialização do computador";
export const NEXT_WINDOWS_BOOT_DISPLAY_SENTENCE = "Tela a ser usada na próxima inicialização do Windows";

const RebootActions = ["reboot", "shutdown"] as const;
type RebootAction = typeof RebootActions[number];
export const REBOOT_ACTIONS: ReadonlyArray<RebootAction> = RebootActions;
export interface Script {
    readonly nextBootOperatingSystem?: OperatingSystem | "unset";
    readonly nextWindowsBootDisplay?: Display | "unset";
    readonly switchToDisplay?: Display | "other" | "saved";
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
      state => state.setNextBootOperatingSystem,
      state => state.unsetNextBootOperatingSystem,
      os => `${NEXT_BOOT_OPERATING_SYSTEM_SENTENCE} foi atualizado para ${operatingSystemText(os)}.`,
    );

    await this.updateStateWith(
      script.nextWindowsBootDisplay,
      state => state.setNextWindowsBootDisplay,
      state => state.unsetNextWindowsBootDisplay,
      display => `${NEXT_WINDOWS_BOOT_DISPLAY_SENTENCE} foi atualizada para ${displayText(display)}.`,
    );

    if (script.switchToDisplay) {
      await this.switchToDisplay(script.switchToDisplay);
    }

    switch (script.rebootAction) {
      case "reboot":
        await this.doRebootAction(osProvider => osProvider.reboot, "Reiniciando...");
        break;
      case "shutdown":
        await this.doRebootAction(osProvider => osProvider.shutdown, "Desligando...");
        break;
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

  private async switchToDisplay(display: Display | 'other' | 'saved') {
    const osProvider = await OSProvider.get();
    if (osProvider.currentDisplayHandling === undefined) {
      throw new Error("O sistema operacional atual não suporta troca de tela");
    }

    async function doSwitch(display: Display): Promise<void> {
      console.log(`Trocando de tela para ${displayText(display)}...`);
      await osProvider.currentDisplayHandling?.set(display);
    }

    const currentDisplay = await osProvider.currentDisplayHandling.get();

    switch (display) {
      case 'other':
        switch(currentDisplay) {
          case 'monitor':
            await doSwitch('tv');
            break;
          case 'tv':
            await doSwitch('monitor');
            break;
        }
        break;
      case 'saved': {
        const state = await this.getState();
        const savedDisplay = await state.getNextWindowsBootDisplay();
        if (savedDisplay === undefined) {
          console.log(`A ${NEXT_WINDOWS_BOOT_DISPLAY_SENTENCE} é ${displayText(savedDisplay)}`);
        } else if (savedDisplay == currentDisplay) {
          console.log(`A ${NEXT_WINDOWS_BOOT_DISPLAY_SENTENCE} é ${displayText(savedDisplay)}, que já é a tela atual`);
        } else {
          await doSwitch(savedDisplay);
          await state.unsetNextWindowsBootDisplay();
        }
        break;
      }
      case 'monitor':
      case 'tv':
        if (display === currentDisplay) {
          console.log(`${displayText(display)} já é a tela atual.`);
        } else {
          await doSwitch(display);
        }
        break;
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
