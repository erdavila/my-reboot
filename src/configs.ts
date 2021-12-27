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
    const key = this.keyFor(object);
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

  setValue(object: T, value: string) {
    const key = this.keyFor(object);
    this.props.set(key, value);
  }

  private keyFor(object: T) {
    return `${object}.${this.attribute}`;
  }
}

export class Configs {
  protected readonly grubEntryHandler: ConfigHandler<OperatingSystem>;
  protected readonly deviceIdHandler: ConfigHandler<Display>;
  protected readonly displaySwitchArgHandler: ConfigHandler<Display>;

  protected static readonly FILE = 'my-reboot-configs.properties';

  static async load(stateDir: string): Promise<Configs> {
    return await ConfigsWriter.load(stateDir);
  }

  protected constructor(props: Properties) {
    this.grubEntryHandler = new ConfigHandler<OperatingSystem>(props, 'grubEntry', OPERATING_SYSTEMS, 'Linux');
    this.deviceIdHandler = new ConfigHandler<Display>(props, 'deviceId', DISPLAYS, 'Windows');
    this.displaySwitchArgHandler = new ConfigHandler<Display>(props, 'displaySwitchArg', DISPLAYS, 'Windows');
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

  getDisplaySwitchArg(display: Display): string {
    return this.displaySwitchArgHandler.getValue(display);
  }
}

export class ConfigsWriter extends Configs {
  static override async load(stateDir: string): Promise<ConfigsWriter> {
    const props = await Properties.load(`${stateDir}/${Configs.FILE}`);
    return new ConfigsWriter(props);
  }

  private readonly props: Properties;

  private constructor(props: Properties) {
    super(props);
    this.props = props;
  }

  setGrubEntry(operatingSystem: OperatingSystem, value: string) {
    this.grubEntryHandler.setValue(operatingSystem, value);
  }

  setDeviceId(display: Display, value: string) {
    this.deviceIdHandler.setValue(display, value);
  }

  setDisplaySwitchArg(display: Display, value: string) {
    this.displaySwitchArgHandler.setValue(display, value);
  }

  async save() {
    await this.props.save();
  }
}
