import * as fsPromises from 'fs/promises';

export class Properties {
    private readonly map: Map<string, string>;
    private readonly path: string;

    static async load(path: string) {
        const map = new Map<string, string>();
        const content = await fsPromises.readFile(path, 'ascii');
        content
            .split(/\r?\n/)
            .filter(line => !line.startsWith('#'))
            .forEach(line => {
                const index = line.indexOf('=');
                const key = line.substring(0, index);
                const value = line.substring(index + 1);
                if (key.length > 0) {
                    map.set(key, value);
                }
            });

        return new Properties(map, path);
    }

    private constructor(map: Map<string, string>, path: string) {
        this.map = map;
        this.path = path;
    }

    get(key: string): string | undefined {
        return this.map.get(key);
    }

    set(key: string, value: string): void {
        this.map.set(key, value);
    }

    clear(key: string): void {
        this.map.delete(key);
    }

    async save(): Promise<void> {
        const lines: string[] = [];
        this.map.forEach((value, key) => {
            lines.push(`${key}=${value}\n`);
        });
        const content = lines.sort().join('');
        await fsPromises.writeFile(this.path, content);
    }
}
