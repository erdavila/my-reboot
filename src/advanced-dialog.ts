import { OSProvider } from "./os-provider";
import { ipcMain } from "electron";
import { OperatingSystem, State, Display } from "./state";
import { showDialog } from "./dialog";

export function showAdvancedDialog(osProvider: OSProvider) {
  ipcMain.handleOnce('get-state', async () => {
    const state = new State(osProvider.stateDir);
    const operatingSystem = await state.getOperatingSystem();
    const display = await state.getWindowsDisplay();
    const isSwitchDisplaySupported = osProvider.currentDisplayHandling !== undefined;
    const values: [OperatingSystem | undefined, Display | undefined, boolean] = [operatingSystem, display, isSwitchDisplaySupported];
    return values;
  });

  showDialog(osProvider, {
    width: 340,
    filePrefix: 'advanced-dialog',
  })
}
