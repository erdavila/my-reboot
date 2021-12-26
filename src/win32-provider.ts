import { execFile, OSProvider } from "./os-provider";
import { Display } from "./state";
import * as path from "path";
import { Configs } from "./configs";

class WindowsProvider extends OSProvider {
  override predefinedScripts = [
    {
      script: {
        nextBootOperatingSystem: 'linux' as const,
        rebootAction: 'reboot' as const,
      },
      buttonLabel: "Reiniciar no Linux",
    }
  ];

  override stateDir = 'C:\\grubenv.dir';

  override icon = 'icon.ico';

  override async reboot(): Promise<void> {
    await this.shutdownNow('/g');
  }

  override async shutdown(): Promise<void> {
    await this.shutdownNow('/sg');
  }

  private async shutdownNow(arg: string) {
    await execFile('shutdown', [arg, '/t', '0']);
  }

  override currentDisplay = {
    async get(): Promise<Display> {
      const result = await execFile(path.join(__dirname, "get_active_display_device_id.exe"));
      const deviceId = result.stdout.trimEnd();

      const configs = await Configs.load(windowsProvider.stateDir);
      return configs.getDisplayByDeviceId(deviceId);
    },

    async set(_display: Display): Promise<void> {
      throw new Error("Not implemented: WindowsProvider.currentDisplay.set()");
    },
  };
}

const windowsProvider = new WindowsProvider;

export default windowsProvider;
