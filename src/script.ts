import { OSProvider } from "./os-provider";
import { OperatingSystem, State, WindowsDisplay } from "./state";

export interface Script {
    readonly nextBootOperatingSystem?: OperatingSystem | "unset";
    readonly nextWindowsBootDisplay?: WindowsDisplay | "unset";
    readonly rebootAction?: "reboot" | "shutdown";
}

export class ScriptExecutor {
    private state: State | undefined = undefined;
    private osProvider: OSProvider | undefined = undefined;

    static get(): ScriptExecutor {
        return new ScriptExecutor();
    }

    async execute(script: Script) {
        await this.updateStateWith(
            script.nextBootOperatingSystem,
            state => state.setOperatingSystem,
            state => state.unsetOperatingSystem,
            "Sistema operacional a ser iniciado na próxima inicialização do computador foi atualizado.",
        );

        await this.updateStateWith(
            script.nextWindowsBootDisplay,
            state => state.setWindowsDisplay,
            state => state.unsetWindowsDisplay,
            "Tela a ser usada na próxima inicialização do Windows foi atualizada.",
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
        message: string,
    ): Promise<void> {
        if (value) {
            const state = await this.getState();
            if (value === "unset") {
                await unset(state).call(state);
            } else {
                await set(state).call(state, value);
            }
            console.log(message);
        }
    }

    private async getState(): Promise<State> {
        if (!this.state) {
            const osProvider = await this.getOSProvider();
            this.state = new State(osProvider.stateDir);
        }
        return this.state;
    };

    private async doRebootAction(
        action: (osProvider: OSProvider) => () => Promise<void>,
        message: string,
    ): Promise<void> {
        const osProvider = await this.getOSProvider();
        console.log(message);
        await action(osProvider).call(osProvider);
    }

    private async getOSProvider(): Promise<OSProvider> {
        if (!this.osProvider) {
            this.osProvider = await OSProvider.get();
        }
        return this.osProvider;
    }
}
