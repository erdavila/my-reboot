import { ipcMain, ipcRenderer } from "electron";
import { PredefinedScript } from "./os-provider";
import { Script } from "./script";
import { StateValues } from "./state";

export const CloseDialogMessage = {
  CHANNEL: 'close-dialog',

  send() {
    return ipcRenderer.send(this.CHANNEL);
  },

  receive(callback: () => Promise<void>) {
    ipcMain.once(this.CHANNEL, async () =>
      await callback()
    );
  },
}

export const ExecuteScriptMessage = {
  CHANNEL: 'execute-script',

  send(script: Script) {
    return ipcRenderer.send(this.CHANNEL, script);
  },

  receive(callback: (script: Script) => Promise<void>) {
    ipcMain.once(this.CHANNEL, async (_event, script: Script) =>
      await callback(script)
    );
  },
};

export const GetPredefinedScripts = {
  CHANNEL: 'get-predefined-scripts',

  send(): Promise<PredefinedScript[]> {
    return ipcRenderer.invoke(this.CHANNEL);
  },

  receive(callback: () => PredefinedScript[]) {
    ipcMain.handleOnce(this.CHANNEL, callback);
  },
}

export const GetStateMessage = {
  CHANNEL: 'get-state',

  send(): Promise<StateValues> {
    return ipcRenderer.invoke(this.CHANNEL);
  },

  receive(callback: () => Promise<StateValues>) {
    ipcMain.handleOnce(this.CHANNEL, callback);
  },
};

export const ReplaceDialogMessage = {
  CHANNEL: 'replace-dialog',

  send(options: { advanced: boolean }) {
    ipcRenderer.invoke(this.CHANNEL, options);
  },

  receive(callback: (options: { advanced: boolean }) => void) {
    ipcMain.handle(this.CHANNEL, (_event, options: { advanced: boolean }) =>
      callback(options)
    );
  },
};
