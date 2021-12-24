import { Script } from "./script";
import * as childProcess from "child_process";
import * as util from "util";

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

    protected execFile = util.promisify(childProcess.execFile);

    static async get(): Promise<OSProvider> {
      const osProvider = await import(`./${process.platform}-provider`);
      return osProvider.default;
    }
}
