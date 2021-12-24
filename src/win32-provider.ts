import { OSProvider } from "./os-provider";

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
    await this.execFile('shutdown', [arg, '/t', '0']);
  }
}

export default new WindowsProvider();
