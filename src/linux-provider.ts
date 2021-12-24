import { OSProvider } from "./os-provider";
import { Script } from "./script";
import { WindowsDisplay } from "./state";

function rebootToWindowsWithDisplay(display: WindowsDisplay): Script {
    return {
        nextBootOperatingSystem: 'windows',
        nextWindowsBootDisplay: display,
        rebootAction: 'reboot',
    }
}

class LinuxProvider extends OSProvider {
    override predefinedScripts = [
        {
            script: rebootToWindowsWithDisplay('monitor'),
            buttonLabel: "Reiniciar no Windows usando o monitor",
        },
        {
            script: rebootToWindowsWithDisplay('tv'),
            buttonLabel: "Reiniciar no Windows usando a TV",
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
        await this.execFile('systemctl', [command]);
    }
}

export default new LinuxProvider();
