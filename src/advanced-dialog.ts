import { OSProvider } from "./os-provider";
import { ipcMain } from "electron";
import { State } from "./state";
import { showDialog } from "./dialog";

export function showAdvancedDialog(osProvider: OSProvider) {
  ipcMain.handleOnce('get-state', async () => {
    const state = new State(osProvider.stateDir);
    return await state.getValues();
  });

  showDialog(osProvider, {
    width: 340,
    filePrefix: 'advanced-dialog',
  })
}
