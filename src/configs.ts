import { Properties } from "./properties";

export class Configs {
    private readonly props: Properties;

    static async load(stateDir: string) {
        const props = await Properties.load(`${stateDir}/my-reboot-configs.properties`);
        return new Configs(props);
    }

    private constructor(props: Properties) {
        this.props = props;
    }

    getGrubEntry(operatingSystem: string): string {
        const key = `${operatingSystem}.grubEntry`
        const value = this.props.get(key);
        if (value) {
            return value;
        } else {
            throw new Error(`Configuração '${key}' Não encontrada. A configuração foi executada no Linux?`);
        }
    }
}
