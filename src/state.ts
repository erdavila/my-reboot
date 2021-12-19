import { Configs } from "./configs";
import { Grubenv } from "./grubenv";
import { Properties } from "./properties";

export type OperatingSystem = 'windows' | 'linux';

export type WindowsDisplay = 'monitor' | 'tv';

const GRUB_ENTRY = 'saved_entry';
const WINDOWS_DISPLAY_KEY = 'windows.display';

export class State {
    private readonly stateDir: string;

    constructor(stateDir: string) {
        this.stateDir = stateDir;
    }

    async setOperatingSystem(operatingSystem: OperatingSystem): Promise<void> {
        const grubEntry = await this.getGrubEntryConfigFor(operatingSystem);
        await this.withGrubenv(grubenv => grubenv.set(GRUB_ENTRY, grubEntry));
    }

    async unsetOperatingSystem(): Promise<void>  {
        await this.withGrubenv(grubenv => grubenv.clear(GRUB_ENTRY));
    }

    private async getGrubEntryConfigFor(operatingSystem: OperatingSystem): Promise<string> {
        const configs = await Configs.load(this.stateDir);
        return configs.getGrubEntry(operatingSystem);
    }

    private async withGrubenv<R>(operation: (grubenv: Grubenv) => R): Promise<R> {
        const grubenv = await Grubenv.load(this.stateDir);
        const result = operation(grubenv);
        await grubenv.save();
        return result;
    }

    async setWindowsDisplay(display: WindowsDisplay): Promise<void> {
        await this.withOptions(options => options.set(WINDOWS_DISPLAY_KEY, display));
    }

    async unsetWindowsDisplay(): Promise<void> {
        await this.withOptions(options => options.clear(WINDOWS_DISPLAY_KEY));
    }

    private async withOptions<R>(operation: (options: Properties) => R): Promise<R> {
        const options = await Properties.load(`${this.stateDir}/my-reboot-options.properties`);
        const result = operation(options);
        await options.save();
        return result;
    }
}
