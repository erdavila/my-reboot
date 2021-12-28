import { ipcMain, ipcRenderer } from "electron";
import { PredefinedScript } from "./os-provider";
import { Script } from "./script";
import { StateValues } from "./state";

class MessageWithoutResponse<Args extends unknown[]> {
  private readonly channel: string;

  constructor(channel: string) {
    this.channel = channel;
  }

  send(...args: Args) {
    ipcRenderer.send(this.channel, args);
  }

  on(callback: (...args: Args) => Promise<void>) {
    this.receive(false, callback);
  }

  once(callback: (...args: Args) => Promise<void>) {
    this.receive(true, callback);
  }

  private receive(once: boolean, callback: (...args: Args) => Promise<void>) {
    const receive = once ? ipcMain.once : ipcMain.on;
    receive.call(ipcMain, this.channel, async (_event, args: Args) =>
      await callback(...args)
    );
  }
}

class MessageWithResponse<Args extends unknown[], Response> {
  private readonly channel: string;

  constructor(channel: string) {
    this.channel = channel;
  }

  invoke(...args: Args): Promise<Response> {
    return ipcRenderer.invoke(this.channel, args)
  }

  handle(callback: (...args: Args) => (Response | Promise<Response>)) {
    this.receive(false, callback);
  }

  handleOnce(callback: (...args: Args) => (Response | Promise<Response>)) {
    this.receive(true, callback);
  }

  private receive(once: boolean, callback: (...args: Args) => (Response | Promise<Response>)) {
    const receive = once ? ipcMain.handleOnce : ipcMain.handle;
    receive.call(ipcMain, this.channel, (_event, args) =>
      callback(...args)
    );
  }
}

export const CloseDialogMessage = new MessageWithoutResponse<[]>('close-dialog');
export const ExecuteScriptMessage = new MessageWithoutResponse<[script: Script]>('execute-script');
export const GetPredefinedScripts = new MessageWithResponse<[], PredefinedScript[]>('get-predefined-scripts');
export const GetStateValuesMessage = new MessageWithResponse<[], StateValues>('get-state-values');
export const ReplaceDialogMessage = new MessageWithResponse<[options: { advanced: boolean }], void>('replace-dialog');
