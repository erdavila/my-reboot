import { Properties } from "./properties";
import { Display, DISPLAYS, OperatingSystem, OPERATING_SYSTEMS } from "./state";

type ConfigProviderOperatingSystem = 'Linux' | 'Windows';

export class ConfigurationError extends Error {
  constructor(message: string, configProviderOS: ConfigProviderOperatingSystem) {
    super(`${message}. Execute 'my-reboot configure' no ${configProviderOS}`);
    Object.setPrototypeOf(this, ConfigurationError.prototype);
  }
}

class ConfigHandler<T> {
  // object.attribute = value
  private readonly props: Properties;
  private readonly attribute: string;
  private readonly configProviderOS: ConfigProviderOperatingSystem;
  private readonly objects: ReadonlyArray<T>;

  constructor(props: Properties, attribute: string, objects: ReadonlyArray<T>, configProviderOS: ConfigProviderOperatingSystem) {
    this.props = props;
    this.attribute = attribute;
    this.objects = objects;
    this.configProviderOS = configProviderOS;
  }

  getValue(object: T): string {
    const key = `${object}.${this.attribute}`;
    const value = this.props.get(key);
    if (value !== undefined) {
      return value;
    } else {
      throw new ConfigurationError(`Configuração '${key}' não encontrada`, this.configProviderOS);
    }
  }

  getObjectByValue(value: string): T {
    const obj = this.objects.find(obj => this.getValue(obj) === value);
    if (obj !== undefined) {
      return obj;
    } else {
      throw new ConfigurationError(`Configuração com valor ${value} não encontrada`, this.configProviderOS);
    }
  }
}

export class Configs {
  private readonly grubEntryHandler: ConfigHandler<OperatingSystem>;
  private readonly deviceIdHandler: ConfigHandler<Display>;

  static async load(stateDir: string) {
    const props = await Properties.load(`${stateDir}/my-reboot-configs.properties`);
    return new Configs(props);
  }

  private constructor(props: Properties) {
    this.grubEntryHandler = new ConfigHandler<OperatingSystem>(props, 'grubEntry', OPERATING_SYSTEMS, 'Linux');
    this.deviceIdHandler = new ConfigHandler<Display>(props, 'deviceId', DISPLAYS, 'Windows');
  }

  getOperatingSystemByGrubEntry(grubEntry: string): OperatingSystem {
    return this.grubEntryHandler.getObjectByValue(grubEntry);
  }

  getGrubEntry(operatingSystem: OperatingSystem): string {
    return this.grubEntryHandler.getValue(operatingSystem);
  }

  getDisplayByDeviceId(deviceId: string): Display {
    return this.deviceIdHandler.getObjectByValue(deviceId);
  }

  getDeviceId(display: Display): string {
    return this.deviceIdHandler.getValue(display);
  }
}
