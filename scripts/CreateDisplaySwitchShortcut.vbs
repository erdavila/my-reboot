Dim MyRebootPath, DisplaySwitchPath

MyRebootPath = Wscript.Arguments(0)
DisplaySwitchPath = Wscript.Arguments(1)

Set WshShell = WScript.CreateObject("WScript.Shell")

Set oLink = WshShell.CreateShortcut(WshShell.ExpandEnvironmentStrings("%UserProfile%") + "\Desktop\Trocar de Tela.lnk")
oLink.TargetPath = MyRebootPath
oLink.Arguments = "switch:other"
oLink.Description = "Trocar de Tela"
oLink.IconLocation = DisplaySwitchPath + ",2"
oLink.Hotkey = "ALT+CTRL+X"
oLink.Save
