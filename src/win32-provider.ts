import { execFile, CurrentDisplay, OSProvider } from "./os-provider";
import { Display, displayText } from "./state";
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

  override currentDisplay = new WindowsCurrentDisplay(this.stateDir);
}

class WindowsCurrentDisplay implements CurrentDisplay {
  private configs?: Configs = undefined;
  private readonly stateDir: string;

  constructor(stateDir: string) {
    this.stateDir = stateDir;
  }

  async get(): Promise<Display> {
    const result = await execFile(path.join(__dirname, "get_active_display_device_id.exe"));
    const deviceId = result.stdout.trimEnd();

    const configs = await this.getConfigs();
    return configs.getDisplayByDeviceId(deviceId);
  }

  async set(display: Display): Promise<void> {
    const WAIT_SECONDS = 10;
    const configs = await this.getConfigs();
    const displaySwitchArg = configs.getDisplaySwitchArg(display);
    await execFile("DisplaySwitch.exe", [displaySwitchArg]);
    for (let i = 0; i < WAIT_SECONDS; i++) {
      await this.sleep(1000);
      const currentDisplay = await this.get();
      if (currentDisplay === display) {
        return;
      }
    }

    console.error(`A tela não mudou para ${displayText(display)} em ${WAIT_SECONDS} segundos!`);
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
