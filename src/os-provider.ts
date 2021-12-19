import { Script } from "./script";

export interface Button {
    readonly label: string;
    readonly script: Script;
}

export abstract class OSProvider {
    abstract readonly buttons: Button[];

    abstract readonly stateDir: string;

    abstract readonly icon: string;

    abstract reboot(): Promise<void>;

    abstract shutdown(): Promise<void>;

    static async get(): Promise<OSProvider> {
        const osProvider = await import(`./${process.platform}-provider`);
        return osProvider.default;
    }
};
