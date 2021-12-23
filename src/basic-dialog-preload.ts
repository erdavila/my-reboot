import { ipcRenderer } from "electron";
import { PredefinedScript } from "./os-provider";

window.addEventListener("DOMContentLoaded", () => {
    ipcRenderer.invoke('get-predefined-scripts').then((predefinedScripts: PredefinedScript[]) => {
        const advancedBlock = document.getElementById('footer');

        predefinedScripts.forEach((predefScript) => {
            const button = document.createElement('button');
            button.classList.add('script-button');
            button.append(document.createTextNode(predefScript.buttonLabel));
            button.addEventListener('click', () => {
                ipcRenderer.send('execute-script', predefScript.script);
            });

            const div = document.createElement('div');
            div.appendChild(button);
            document.body.insertBefore(div, advancedBlock);
        });
    });
});
