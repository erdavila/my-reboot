import * as fsPromises from 'fs/promises';
import { ConfigsWriter } from './configs';
import { OSProvider } from './os-provider';
import { OperatingSystem, OPERATING_SYSTEMS } from './state';

export async function configure(): Promise<void> {
  switch (process.platform) {
    case 'linux':
      await linuxConfigure();
      break;
    case 'win32':
      await windowsConfigure();
      break;
  }

  console.log("Configuração finalizada.");
  process.exit();
}

async function linuxConfigure() {
  const GRUB_CFG = '/boot/grub/grub.cfg';
  const GRUB_ENTRY_RE = /.*'([a-zA-Z0-9_-]+)'\s*\{.*/;

  console.log(`Lendo ${GRUB_CFG}...`);

  const content = await fsPromises.readFile(GRUB_CFG, 'ascii');
  const entries = content.split(/\r?\n/)
    .filter(line => line.startsWith('menuentry '))
    .flatMap<Partial<Record<OperatingSystem, string>>>(line => {
      const os = OPERATING_SYSTEMS.find(os => line.indexOf(os) >= 0);

      const execResult = GRUB_ENTRY_RE.exec(line);
      const grubEntry = execResult ? execResult[1] : undefined;

      return os !== undefined && grubEntry !== undefined
        ? [{ [os]: grubEntry }]
        : [];
    })
    .reduce((x, y) => {
      return {
        ...x,
        ...y,
      };
    });

  const osProvider = await OSProvider.get()
  const configsWriter = await ConfigsWriter.load(osProvider.stateDir);
  OPERATING_SYSTEMS.forEach(os => {
    const grubEntry = entries[os];
    if (grubEntry === undefined) {
      throw new Error(`Entrada não encontrada para ${os}!`);
    }

    configsWriter.setGrubEntry(os, grubEntry);
  });

  await configsWriter.save();
}

async function windowsConfigure() {
  throw new Error("Not implemented: windowsConfigure()");
}
