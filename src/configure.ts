import { app } from 'electron';
import * as fsPromises from 'node:fs/promises';
import * as path from "node:path";
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
  await saveConfigs(osProvider, writer => {
    OPERATING_SYSTEMS.forEach(os => {
      const grubEntry = entries[os];
      if (grubEntry === undefined) {
        throw new Error(`Entrada não encontrada para ${os}!`);
      }

      writer.setGrubEntry(os, grubEntry);
    });
  });

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

  await detectDisplays(initialDisplay);
  await setDisplaySwitchRunOnStartup();
  await createDisplaySwitchShortcuts();

  configurationDone();
}

async function detectDisplays(initialDisplay: Display) {
  const osProvider = await OSProvider.get();
  const currentDisplayHandling = osProvider.currentDisplayHandling as WindowsCurrentDisplayHandling;

  const DISPLAY_SWITCH_ARGS = ["/internal", "/external"] as const;
  type DisplaySwitchArg = typeof DISPLAY_SWITCH_ARGS[number];

  const initialDisplayDeviceId = await currentDisplayHandling.getActiveDisplayDeviceId();
  let initialDisplaySwitchArg: DisplaySwitchArg;
  const otherDisplay = initialDisplay === "tv" ? "monitor" : "tv";
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

  await saveConfigs(osProvider, writer => {
    writer.setDeviceId(initialDisplay, initialDisplayDeviceId);
    writer.setDisplaySwitchArg(initialDisplay, initialDisplaySwitchArg);
    writer.setDeviceId(otherDisplay, otherDisplayDeviceId);
    writer.setDisplaySwitchArg(otherDisplay, otherDisplaySwitchArg);
  });
}

async function setDisplaySwitchRunOnStartup() {
  console.log("Registrando troca de tela ao iniciar o Windows...");
  await execFile('REG', [
    'ADD', 'HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run',
    '/v', 'My Reboot Display Switch',
    '/d', `"${app.getPath('exe')}" -- switch:saved`,
    '/f',
  ]);
}

async function createDisplaySwitchShortcuts() {
  const script = `
    Set WshShell = WScript.CreateObject("WScript.Shell")
    Set oLink = WshShell.CreateShortcut(Wscript.Arguments(0) + "\\Trocar de Tela.lnk")
    oLink.TargetPath = "${app.getPath('exe')}"
    oLink.Arguments = "-- switch:other"
    oLink.Description = "Trocar de Tela"
    oLink.IconLocation = "${WindowsCurrentDisplayHandling.DISPLAY_SWITCH_PATH},2"
    oLink.Hotkey = "ALT+CTRL+X"
    oLink.Save
  `;

  const tempDir = await fsPromises.mkdtemp('my-reboot');
  const scriptPath = path.join(tempDir, 'CreateShortcut.vbs');
  try {
    await fsPromises.writeFile(scriptPath, script);

    const shortcuts = [
      ['na Área de Trabalho', app.getPath('desktop')],
      ['no menu Iniciar', path.join(app.getPath('appData'), 'Microsoft\\Windows\\Start Menu\\Programs\\My Reboot')],
    ] as const;

    for (const [where, shortcutPath] of shortcuts) {
      console.log(`Criando atalho para Troca de Tela ${where}...`);
      await execFile('cscript', [scriptPath, shortcutPath]);
    }
  } finally {
    await fsPromises.rm(tempDir, { recursive: true, force: true });
  }
}

async function saveConfigs(osProvider: OSProvider, set: (configsWriter: ConfigsWriter) => void) {
  const configsWriter = await ConfigsWriter.load(osProvider.stateDir);
  set(configsWriter);
  console.log("Salvando configurações...");
  await configsWriter.save();
}

function configurationDone() {
  console.log("Configuração finalizada.");
  process.exit();
}
