import { OSProvider } from "./os-provider";
import { Script } from "./script";
import { WindowsDisplay } from "./state";
import * as childProcess from "child_process";
import * as util from "util";

function rebootToWindowsWithDisplay(display: WindowsDisplay): Script {
    return {
        nextBootOperatingSystem: 'windows',
        nextWindowsBootDisplay: display,
        rebootAction: 'reboot',
    }
}

class LinuxProvider extends OSProvider {
    override buttons = [
        {
            label: "Reiniciar no Windows usando o monitor",
            script: rebootToWindowsWithDisplay('monitor'),
        },
        {
            label: "Reiniciar no Windows usando a TV",
            script: rebootToWindowsWithDisplay('tv'),
        },
    ]

    override stateDir = '/boot/grub/grubenv.dir';

    override icon = 'icon.png';

    override async reboot(): Promise<void> {
        await this.systemctl('reboot');
    }

    override async shutdown(): Promise<void> {
        await this.systemctl('poweroff');
    }

    private async systemctl(command: string) {
        const execFile = util.promisify(childProcess.execFile);
        await execFile('systemctl', [command]);
    }
}

export default new LinuxProvider();
