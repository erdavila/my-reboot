import * as fsPromises from 'fs/promises';

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

    set(key: string, value: string): void {
        const index = this.lines.findIndex(s => s.startsWith(`${key}=`));
        const updatedLine = `${key}=${value}`;
        if (index >= 0) {
            this.lines[index] = updatedLine;
        } else {
            this.lines.push(updatedLine);
        }
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
