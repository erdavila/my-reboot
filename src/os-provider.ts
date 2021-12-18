export interface Button {
    clicked(): Promise<void>;
    readonly label: string;
}

export abstract class OSProvider {
    abstract readonly buttons: Button[];

    abstract readonly stateDir: string;

    abstract readonly icon: string;

    abstract reboot(): Promise<void>;
};
