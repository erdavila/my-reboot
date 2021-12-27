import { app } from 'electron';
import * as fsPromises from 'fs/promises';
import { ConfigsWriter } from './configs';
import { execFile, OSProvider } from './os-provider';
import { Display, DISPLAYS, OperatingSystem, OPERATING_SYSTEMS } from './state';
import { WindowsCurrentDisplayHandling } from './win32-provider';

export async function linuxConfigure() {
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

  configurationDone();
}

export async function windowsConfigure(initialDisplay: Display | undefined) {
  if (initialDisplay === undefined) {
    console.log('Execute:');
    console.log('  my-reboot configure TELA');
    console.log(`onde TELA é a tela atual (${DISPLAYS.map(d => `"${d}"`).join(' ou ')}).`);
    console.log();
    console.log('Será testada a troca de telas. A configuração termina ao retornar para a tela inicial.');
    process.exit();
  }

  const osProvider = await OSProvider.get();
  const currentDisplayHandling = osProvider.currentDisplayHandling as WindowsCurrentDisplayHandling;

  const DISPLAY_SWITCH_ARGS = ["/internal", "/external"] as const;
  type DisplaySwitchArg = typeof DISPLAY_SWITCH_ARGS[number];

  const initialDisplayDeviceId = await currentDisplayHandling.getActiveDisplayDeviceId();
  const otherDisplay = initialDisplay === "tv" ? "monitor" : "tv";
  let initialDisplaySwitchArg: DisplaySwitchArg;

  let otherDisplayDeviceId: string;
  let otherDisplaySwitchArg: DisplaySwitchArg;

  console.log("Trocando de tela...");
  const switched = await currentDisplayHandling.executeDisplaySwitch(DISPLAY_SWITCH_ARGS[0], 5);
  if (switched) {
    initialDisplaySwitchArg = DISPLAY_SWITCH_ARGS[1];
    otherDisplayDeviceId = await currentDisplayHandling.getActiveDisplayDeviceId();
    otherDisplaySwitchArg = DISPLAY_SWITCH_ARGS[0];
  } else {
    initialDisplaySwitchArg = DISPLAY_SWITCH_ARGS[0];
    await currentDisplayHandling.executeDisplaySwitch(DISPLAY_SWITCH_ARGS[1], 5);
    otherDisplayDeviceId = await currentDisplayHandling.getActiveDisplayDeviceId();
    otherDisplaySwitchArg = DISPLAY_SWITCH_ARGS[1];
  }

  console.log("Voltando para a tela inicial...");
  await currentDisplayHandling.executeDisplaySwitch(initialDisplaySwitchArg, 5);

  const configsWriter = await ConfigsWriter.load(osProvider.stateDir);
  configsWriter.setDeviceId(initialDisplay, initialDisplayDeviceId);
  configsWriter.setDisplaySwitchArg(initialDisplay, initialDisplaySwitchArg);
  configsWriter.setDeviceId(otherDisplay, otherDisplayDeviceId);
  configsWriter.setDisplaySwitchArg(otherDisplay, otherDisplaySwitchArg);
  await configsWriter.save();

  console.log("Registrando troca de tela ao iniciar o Windows...");
  await execFile('REG', [
    'ADD', 'HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run',
    '/v', 'My Reboot Display Switch',
    '/d', `"${app.getPath('exe')}" -- switch:saved`,
    '/f'
  ]);

  configurationDone();
}

function configurationDone() {
  console.log("Configuração finalizada.");
  process.exit();
}
