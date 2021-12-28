import { CloseDialogMessage, ReplaceDialogMessage } from "./messages";

export function preloadCommon(options: { advancedDialog: boolean }) {
  document.getElementById('switch-mode')?.addEventListener('click', () => {
    ReplaceDialogMessage.invoke({ advanced: !options.advancedDialog });
  });

  document.body.addEventListener('keydown', event => {
    if (event.key === 'Escape') {
      CloseDialogMessage.send();
    }
  });
}
