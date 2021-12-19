import { Configs } from "./configs";
import { Grubenv } from "./grubenv";
import { Properties } from "./properties";

export type OperatingSystem = 'windows' | 'linux';

export type WindowsDisplay = 'monitor' | 'tv';

export class State {
    private readonly stateDir: string;

    constructor(stateDir: string) {
        this.stateDir = stateDir;
    }

    async setOperatingSystem(operatingSystem: OperatingSystem): Promise<void> {
        const grubEntry = await this.getGrubEntryConfigFor(operatingSystem);
        await this.setGrubenvEntry(grubEntry);
    }

    async unsetOperatingSystem() {
        throw new Error("Method not implemented: State.unsetOperatingSystem()");
    }

    private async getGrubEntryConfigFor(operatingSystem: OperatingSystem): Promise<string> {
        const configs = await Configs.load(this.stateDir);
        return configs.getGrubEntry(operatingSystem);
    }

    private async setGrubenvEntry(grubEntry: string): Promise<void> {
        const grubenv = await Grubenv.load(this.stateDir);
        grubenv.set('saved_entry', grubEntry);
        await grubenv.save();
    }

    async setWindowsDisplay(display: WindowsDisplay): Promise<void> {
        const options = await Properties.load(`${this.stateDir}/my-reboot-options.properties`);
        options.set('windows.display', display);
        await options.save();
    }

    async unsetWindowsDisplay() {
        throw new Error("Method not implemented: State.unsetWindowsDisplay.");
    }
}
