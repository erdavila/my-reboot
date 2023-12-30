import { OSProvider } from "./os-provider";
import { showDialog } from "./dialog";
import { GetPredefinedScripts } from "./messages";

export function showBasicDialog(osProvider: OSProvider) {
  GetPredefinedScripts.handleOnce(() =>
    osProvider.predefinedScripts
  );

  showDialog(osProvider, {
    width: 300,
    filePrefix: 'basic-dialog',
  });
}