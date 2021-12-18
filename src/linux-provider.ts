import { OSProvider } from "./os-provider";
import { State, WindowsDisplay } from "./state";
import * as childProcess from "child_process";
import { promisify } from "util";

const execFile = promisify(childProcess.execFile);

class LinuxProvider extends OSProvider {
    override buttons = [
        {
            label: "Reiniciar no Windows usando o monitor",
            clicked: this.rebootToWindows('monitor'),
        },
        {
            label: "Reiniciar no Windows usando a TV",
            clicked: this.rebootToWindows('tv'),
        },
    ]

    override stateDir = '/boot/grub/grubenv.dir';

    override icon = 'icon.png';

    private rebootToWindows(display: WindowsDisplay) {
        return async () => {
            const state = new State(this.stateDir);
            await state.setOperatingSystem('windows');
            await state.setWindowsDisplay(display);
            this.reboot();
        };
    }

    override async reboot(): Promise<void> {
        await execFile('systemctl', ['reboot'])
    }
}

export default new LinuxProvider();
