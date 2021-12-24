import chalk = require("chalk");
import { Configs } from "./configs";
import { Grubenv } from "./grubenv";
import { Properties } from "./properties";

const OperatingSystems = ['windows', 'linux'] as const;
export type OperatingSystem = typeof OperatingSystems[number];
export const OPERATING_SYSTEMS: ReadonlyArray<OperatingSystem> = OperatingSystems;
export function operatingSystemText(operatingSystem: OperatingSystem | undefined) {
  switch (operatingSystem) {
    case "linux": return chalk.green.bold("Linux");
    case "windows": return chalk.blueBright.bold("Windows");
    case undefined: return chalk.red.bold("indefinido");
  }
}

const WindowsDisplays = ['monitor', 'tv'] as const;
export type WindowsDisplay = typeof WindowsDisplays[number];
export const WINDOWS_DISPLAYS: ReadonlyArray<WindowsDisplay> = WindowsDisplays;
export function windowsDisplayText(windowsDisplay: WindowsDisplay | undefined) {
  switch (windowsDisplay) {
    case "monitor": return chalk.green.bold("monitor");
    case "tv": return chalk.blueBright.bold("TV");
    case undefined: return chalk.red.bold("indefinida");
  }
}

const GRUB_ENTRY = 'saved_entry';
const WINDOWS_DISPLAY_KEY = 'windows.display';

export class State {
  private readonly stateDir: string;

  constructor(stateDir: string) {
    this.stateDir = stateDir;
  }

  async getOperatingSystem(): Promise<OperatingSystem | undefined> {
    const grubEntry = await this.withGrubenv(grubenv => grubenv.get(GRUB_ENTRY));
    if (grubEntry) {
      const configs = await Configs.load(this.stateDir);
      return configs.getOperatingSystemByGrubEntry(grubEntry);
    } else {
      return undefined;
    }
  }

  async setOperatingSystem(operatingSystem: OperatingSystem): Promise<void> {
    const grubEntry = await this.getGrubEntryConfigFor(operatingSystem);
    await this.changeGrubenv(grubenv => grubenv.set(GRUB_ENTRY, grubEntry));
  }

  async unsetOperatingSystem(): Promise<void>  {
    await this.changeGrubenv(grubenv => grubenv.clear(GRUB_ENTRY));
  }

  private async getGrubEntryConfigFor(operatingSystem: OperatingSystem): Promise<string> {
    const configs = await Configs.load(this.stateDir);
    return configs.getGrubEntry(operatingSystem);
  }

  private async changeGrubenv<R>(operation: (grubenv: Grubenv) => R | Promise<R>): Promise<R> {
    return await this.withGrubenv(async (grubenv) => {
      const op = operation(grubenv);
      const result = op instanceof Promise ? await op : op;
      await grubenv.save();
      return result;
    });
  }

  private async withGrubenv<R>(operation: (grubenv: Grubenv) => R): Promise<R> {
    const grubenv = await Grubenv.load(this.stateDir);
    const result = operation(grubenv);
    return result;
  }

  async getWindowsDisplay(): Promise<WindowsDisplay | undefined> {
    return await this.withOptions((options) => options.get(WINDOWS_DISPLAY_KEY) as WindowsDisplay | undefined);
  }

  async setWindowsDisplay(display: WindowsDisplay): Promise<void> {
    await this.changeOptions(options => options.set(WINDOWS_DISPLAY_KEY, display));
  }

  async unsetWindowsDisplay(): Promise<void> {
    await this.changeOptions(options => options.clear(WINDOWS_DISPLAY_KEY));
  }

  private async changeOptions<R>(operation: (options: Properties) => R | Promise<R>): Promise<R> {
    return await this.withOptions(async (options) => {
      const op = operation(options);
      const result = op instanceof Promise ? await op : op;
      await options.save();
      return result;
    });
  }

  private async withOptions<R>(operation: (options: Properties) => R | Promise<R>): Promise<R> {
    const options = await Properties.load(`${this.stateDir}/my-reboot-options.properties`);
    const result = operation(options);
    return result;
  }
}
