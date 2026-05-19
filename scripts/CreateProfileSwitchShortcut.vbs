Dim MyRebootPath, DisplaySwitchPath

MyRebootPath = Wscript.Arguments(0)
Title = Wscript.Arguments(1)
DisplaySwitchPath = Wscript.Arguments(2)

Set WshShell = WScript.CreateObject("WScript.Shell")

Set oLink = WshShell.CreateShortcut(WshShell.ExpandEnvironmentStrings("%UserProfile%") + "\Desktop\" + Title + ".lnk")
oLink.TargetPath = MyRebootPath
oLink.Arguments = "switch:other"
oLink.Description = Title
oLink.WindowStyle = 7 'Minimized
oLink.IconLocation = DisplaySwitchPath + ",2"
oLink.Hotkey = "ALT+CTRL+X"
oLink.Save
