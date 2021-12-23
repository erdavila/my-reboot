import { ipcMain } from "electron";
import { OSProvider } from "./os-provider";
import { showDialog } from "./dialog";

export function showBasicDialog(osProvider: OSProvider) {
  ipcMain.handleOnce('get-predefined-scripts', () =>
    osProvider.predefinedScripts
  );

  showDialog(osProvider, {
    width: 300,
    filePrefix: 'basic-dialog',
  });
}
