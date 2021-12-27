import { execFile, OSProvider } from "./os-provider";
import { Script } from "./script";
import { Display } from "./state";

function rebootToWindowsWithDisplay(display: Display): Script {
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
    await execFile('systemctl', [command]);
  }

  override currentDisplayHandling = undefined;
}

export default new LinuxProvider();
