import { onDOMContentLoaded } from "./dialog-preload";
import { ExecuteScriptMessage, GetPredefinedScripts } from "./messages";

onDOMContentLoaded({ advancedDialog: false }, () => {
  GetPredefinedScripts.invoke().then(predefinedScripts => {
    const advancedBlock = document.getElementById('footer');

    predefinedScripts.forEach((predefScript) => {
      const button = document.createElement('button');
      button.classList.add('script-button');
      button.append(document.createTextNode(predefScript.buttonLabel));
      button.addEventListener('click', () => {
        ExecuteScriptMessage.send(predefScript.script);
      });

      const div = document.createElement('div');
      div.appendChild(button);
      document.body.insertBefore(div, advancedBlock);
    });
  });
});
