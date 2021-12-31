import { execFile, CurrentDisplayHandling, OSProvider } from "./os-provider";
import { Display } from "./state";
import * as path from "node:path";
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

  override currentDisplayHandling = new WindowsCurrentDisplayHandling(this.stateDir);
}

export class WindowsCurrentDisplayHandling implements CurrentDisplayHandling {
  static readonly DISPLAY_SWITCH_PATH = 'C:\\Windows\\system32\\DisplaySwitch.exe';

  private configs?: Configs = undefined;
  private readonly stateDir: string;

  constructor(stateDir: string) {
    this.stateDir = stateDir;
  }

  async get(): Promise<Display> {
    const deviceId = await this.getActiveDisplayDeviceId();
    const configs = await this.getConfigs();
    return configs.getDisplayByDeviceId(deviceId);
  }

  async set(display: Display): Promise<void> {
    const WAIT_SECONDS = 10;
    const configs = await this.getConfigs();
    const displaySwitchArg = configs.getDisplaySwitchArg(display);
    await this.executeDisplaySwitch(displaySwitchArg, WAIT_SECONDS);
  }

  async executeDisplaySwitch(arg: string, waitSeconds: number): Promise<boolean> {
    const deviceIdBefore = await this.getActiveDisplayDeviceId();
    await execFile(WindowsCurrentDisplayHandling.DISPLAY_SWITCH_PATH, [arg]);
    for (let i = 0; i < waitSeconds; i++) {
      await this.sleep(1000);
      const deviceId = await this.getActiveDisplayDeviceId();
      if (deviceId !== deviceIdBefore) {
        return true;
      }
    }

    return false;
  }

  async getActiveDisplayDeviceId(): Promise<string> {
    const result = await execFile(path.join(__dirname, "get_active_display_device_id.exe"));
    return result.stdout.trimEnd();
  }

  private async sleep(ms: number) {
    return new Promise(resolve => setTimeout(resolve, ms) );
  }

  private async getConfigs() {
    if (this.configs === undefined) {
      this.configs = await Configs.load(this.stateDir);
    }
    return this.configs;
  }
}

export default new WindowsProvider;
