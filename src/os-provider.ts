import { Script } from "./script";

export interface PredefinedScript {
    readonly script: Script;
    readonly buttonLabel: string;
}

export abstract class OSProvider {
    abstract readonly predefinedScripts: PredefinedScript[];

    abstract readonly stateDir: string;

    abstract readonly icon: string;

    abstract reboot(): Promise<void>;

    abstract shutdown(): Promise<void>;

    static async get(): Promise<OSProvider> {
        const osProvider = await import(`./${process.platform}-provider`);
        return osProvider.default;
    }
}
