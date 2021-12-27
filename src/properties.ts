import * as fsPromises from 'fs/promises';

export class Properties {
  private readonly map: Map<string, string>;
  private readonly path: string;

  static async load(path: string) {
    const map = new Map<string, string>();

    await fsPromises.readFile(path, 'ascii')
      .then(content => {
        content
          .split(/\r?\n/)
          .filter(line => !line.startsWith('#'))
          .forEach(line => {
            const index = line.indexOf('=');
            const key = line.substring(0, index);
            const escapedValue = line.substring(index + 1);
            if (key.length > 0) {
              const value = escapedValue.replaceAll('\\\\', '\\');
              map.set(key, value);
            }
          });
      })
      .catch(e => {
        if (e.code === 'ENOENT') {
          console.warn(`Arquivo ${path} não encontrado. Prosseguindo com conteúdo vazio.`);
        } else {
          console.error("Erro desconhecido:", e);
          process.exit(1);
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
      const escapedValue = value.replaceAll('\\', '\\\\')
      lines.push(`${key}=${escapedValue}\n`);
    });
    const content = lines.sort().join('');
    await fsPromises.writeFile(this.path, content);
  }
}
