import { Script } from "./script";
import * as childProcess from "child_process";
import * as util from "util";
import { Display } from "./state";

export interface PredefinedScript {
  readonly script: Script;
  readonly buttonLabel: string;
}

export interface CurrentDisplayHandling {
  get(): Promise<Display>;
  set(display: Display): Promise<void>;
}

export abstract class OSProvider {
  abstract readonly predefinedScripts: PredefinedScript[];

  abstract readonly stateDir: string;

  abstract readonly icon: string;

  abstract reboot(): Promise<void>;

  abstract shutdown(): Promise<void>;

  abstract readonly currentDisplayHandling: CurrentDisplayHandling | undefined;

  static async get(): Promise<OSProvider> {
    const osProvider = await import(`./${process.platform}-provider`);
    return osProvider.default;
  }
}

export const execFile = util.promisify(childProcess.execFile);
