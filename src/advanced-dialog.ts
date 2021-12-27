import { OSProvider } from "./os-provider";
import { State } from "./state";
import { showDialog } from "./dialog";
import { GetStateMessage } from "./messages";

export function showAdvancedDialog(osProvider: OSProvider) {
  GetStateMessage.receive(async () => {
    const state = new State(osProvider.stateDir);
    return await state.getValues();
  });

  showDialog(osProvider, {
    width: 340,
    filePrefix: 'advanced-dialog',
  })
}
