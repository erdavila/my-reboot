import { OSProvider } from "./os-provider";
import { State } from "./state";
import { showDialog } from "./dialog";
import { GetStateValuesMessage } from "./messages";

export function showAdvancedDialog(osProvider: OSProvider) {
  GetStateValuesMessage.handleOnce(async () => {
    const state = new State(osProvider.stateDir);
    return await state.getValues();
  });

  showDialog(osProvider, {
    width: 340,
    filePrefix: 'advanced-dialog',
  })
}
