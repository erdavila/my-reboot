import { ipcRenderer } from "electron";

window.addEventListener("DOMContentLoaded", () => {
    ipcRenderer.invoke('get-button-labels').then((buttonLabels: string[]) => {
        const advancedBlock = document.getElementById('advanced-mode-link-block');

        buttonLabels.forEach((label, index) => {
            const button = document.createElement('button');
            button.append(document.createTextNode(label));
            button.addEventListener('click', () => {
                ipcRenderer.send('basic-mode-button-click', index);
            });

            const block = document.createElement('p');
            block.appendChild(button);
            document.body.insertBefore(block, advancedBlock);
        });
    });
});