import { ipcRenderer } from "electron";
import { NEXT_BOOT_OPERATING_SYSTEM_SENTENCE, NEXT_WINDOWS_BOOT_DISPLAY_SENTENCE, REBOOT_ACTIONS, Script } from "./script";
import { OperatingSystem, OPERATING_SYSTEMS, WindowsDisplay, WINDOWS_DISPLAYS } from "./state";

window.addEventListener("DOMContentLoaded", () => {
  ipcRenderer.invoke('get-state').then(([os, display]: [OperatingSystem | undefined, WindowsDisplay | undefined]) => {
    function checkElement<T>(type: string, value: T | undefined) {
      const elementValue = value === undefined ? 'unset' : value
      const element = document.querySelector(`input[name=${type}][value=${elementValue}]`);
      if (element instanceof HTMLInputElement) {
        element.checked = true;
      }
    }

    checkElement('os', os);
    checkElement('display', display);
  });

  function setSentence(id: string, sentence: string) {
    const element = document.getElementById(id)
    if (element !== null) {
      element.innerText = sentence;
    }
  }

  setSentence('os-sentence', NEXT_BOOT_OPERATING_SYSTEM_SENTENCE);
  setSentence('display-sentence', NEXT_WINDOWS_BOOT_DISPLAY_SENTENCE);

  document.getElementById('ok-button')?.addEventListener('click', async () => {
    const possiblyUndefinedForm = document.forms[0];
    if (possiblyUndefinedForm === undefined) {
      throw new Error("Form not found");
    }
    const form = possiblyUndefinedForm;

    function getValue(field: string): string {
      return (form[field] as RadioNodeList).value;
    }

    function validate<T extends string>(value: string, validValues: ReadonlyArray<T>):T {
      const validValue = validValues.find(v => v == value);
      if (validValue === undefined) {
        throw new Error(`Invalid value: ${value}`)
      } else {
        return validValue;
      }
    }

    function mapToUndefined<T1 extends string, T2 extends T1>(value: T1, mapped: T2): Exclude<T1, T2> | undefined {
      return value === mapped ? undefined : (value as Exclude<T1, T2>);
    }

    function getAndValidate<T extends string>(field: string, validValues: ReadonlyArray<T>): T {
      const value = getValue(field);
      return validate(value, validValues);
    }

    function getAndValidateAndMapToUndefined<T1 extends string, T2 extends string>(field: string, validValues: ReadonlyArray<T1>, mapped: T2): Exclude<T1, T2> | undefined {
      const value = getAndValidate(field, [...validValues, mapped]);
      return mapToUndefined(value, mapped);
    }

    const nextBootOperatingSystem = getAndValidate('os', [...OPERATING_SYSTEMS, "unset"]);
    const nextWindowsBootDisplay = getAndValidate('display', [...WINDOWS_DISPLAYS, "unset"]);
    const rebootAction = getAndValidateAndMapToUndefined('action', REBOOT_ACTIONS, "none");

    const script: Script = {
      nextBootOperatingSystem,
      nextWindowsBootDisplay,
      ...(rebootAction !== undefined ? { rebootAction } : {}),
    };

    console.log("script:", script);

    ipcRenderer.send('execute-script', script);
  });
});
