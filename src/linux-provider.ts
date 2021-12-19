import { OSProvider } from "./os-provider";
import { Script } from "./script";
import { WindowsDisplay } from "./state";
import * as childProcess from "child_process";
import { promisify } from "util";

const execFile = promisify(childProcess.execFile);

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
        await execFile('systemctl', ['reboot'])
    }

    override shutdown(): Promise<void> {
        throw new Error("Method not implemented: LinuxProvider.shutdown()");
    }
}

export default new LinuxProvider();
