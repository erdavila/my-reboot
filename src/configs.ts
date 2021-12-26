import { Properties } from "./properties";
import { Display, DISPLAYS, OperatingSystem, OPERATING_SYSTEMS } from "./state";

export class ConfigurationError extends Error {
  constructor(message: string, providerOperatingSystem: 'Linux' | 'Windows') {
    super(`${message}. Execute 'my-reboot configure' no ${providerOperatingSystem}`);
    Object.setPrototypeOf(this, ConfigurationError.prototype);
  }
}

export class Configs {
  private readonly props: Properties;

  static async load(stateDir: string) {
    const props = await Properties.load(`${stateDir}/my-reboot-configs.properties`);
    return new Configs(props);
  }

  private constructor(props: Properties) {
    this.props = props;
  }

  getOperatingSystemByGrubEntry(grubEntry: string): OperatingSystem {
    const os = OPERATING_SYSTEMS.find(os => this.getGrubEntry(os) === grubEntry);
    if (os !== undefined) {
      return os;
    } else {
      throw new ConfigurationError(`Configuração com valor ${grubEntry} não encontrada`, 'Linux');
    }
  }

  getGrubEntry(operatingSystem: OperatingSystem): string {
    const key = `${operatingSystem}.grubEntry`
    const grubEntry = this.props.get(key);
    if (grubEntry !== undefined) {
      return grubEntry;
    } else {
      throw new ConfigurationError(`Configuração '${key}' não encontrada`, 'Linux');
    }
  }

  getDisplayByDeviceId(deviceId: string): Display {
    const display = DISPLAYS.find(display => this.getDeviceId(display) == deviceId);
    if (display !== undefined) {
      return display;
    } else {
      throw new ConfigurationError(`Configuração com valor ${deviceId} não encontrada`, 'Windows');
    }
  }

  getDeviceId(display: Display): string {
    const key = `${display}.deviceId`;
    const deviceId = this.props.get(key);
    if (deviceId !== undefined) {
      return deviceId;
    } else {
      throw new ConfigurationError(`Configuração '${key}' não encontrada`, 'Windows');
    }
  }
}
