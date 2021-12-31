import { CloseDialogMessage, ReplaceDialogMessage } from "./messages";

export function onDOMContentLoaded(options: { advancedDialog: boolean }, callback: () => void) {
  window.addEventListener("DOMContentLoaded", () => {
    callback();

    document.getElementById('switch-mode')?.addEventListener('click', () => {
      ReplaceDialogMessage.invoke({ advanced: !options.advancedDialog });
    });

    document.body.addEventListener('keydown', event => {
      if (event.key === 'Escape') {
        CloseDialogMessage.send();
      }
    });
  });
}
