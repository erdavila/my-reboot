import { ipcRenderer } from "electron";

window.addEventListener("DOMContentLoaded", () => {
    ipcRenderer.invoke('get-button-labels').then((buttonLabels: string[]) => {
        const advancedBlock = document.getElementById('footer');

        buttonLabels.forEach((label, index) => {
            const button = document.createElement('button');
            button.classList.add('script-button');
            button.append(document.createTextNode(label));
            button.addEventListener('click', () => {
                ipcRenderer.send('basic-mode-button-click', index);
            });

            const div = document.createElement('div');
            div.appendChild(button);
            document.body.insertBefore(div, advancedBlock);
        });
    });
});
