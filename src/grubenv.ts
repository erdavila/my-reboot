import * as fsPromises from 'node:fs/promises';

export class Grubenv {
  private readonly lines: string[];
  private readonly path: string;

  static async load(dirPath: string): Promise<Grubenv> {
    const path = `${dirPath}/grubenv`;
    const content = await fsPromises.readFile(path, 'ascii');
    const lines = content.split('\n').filter(s => s.match(/[^#]/));
    return new Grubenv(lines, path);
  }

  private constructor(lines: string[], path: string) {
    this.lines = lines;
    this.path = path;
  }

  get(key: string): string | undefined {
    return this.withKeyIndex(
      key,
      index => this.lines[index]?.substring(key.length + 1),
    );
  }

  set(key: string, value: string): void {
    const updatedLine = `${key}=${value}`;
    this.withKeyIndex<void>(
      key,
      index => this.lines[index] = updatedLine,
      () => this.lines.push(updatedLine),
    );
  }

  clear(key: string): void {
    this.withKeyIndex(
      key,
      index => this.lines.splice(index, 1),
    );
  }

  private withKeyIndex<T>(key: string, found: (index: number) => T): T | undefined;
  private withKeyIndex<T>(key: string, found: (index: number) => T, notFound: () => T): T;
  private withKeyIndex<T>(key: string, found: (index: number) => T, notFound?: () => T) {
    const index = this.lines.findIndex(s => s.startsWith(`${key}=`));
    return index >= 0
      ? found(index)
      : notFound
        ? notFound()
        : undefined;
  }

  async save() {
    const GRUBENV_CONTENT_LENGTH = 1024;

    const content = this.lines
      .map(s => `${s}\n`)
      .join('')
      .substring(0, GRUBENV_CONTENT_LENGTH)
      .padEnd(GRUBENV_CONTENT_LENGTH, '#');

    await fsPromises.writeFile(this.path, content, 'binary');
  }
}
