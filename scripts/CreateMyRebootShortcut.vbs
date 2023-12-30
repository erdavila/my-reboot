Dim MyRebootPath

MyRebootPath = Wscript.Arguments(0)

Set WshShell = WScript.CreateObject("WScript.Shell")

Set oLink = WshShell.CreateShortcut(WshShell.ExpandEnvironmentStrings("%UserProfile%") + "\Desktop\My Reboot.lnk")
oLink.TargetPath = MyRebootPath
oLink.Description = "My Reboot"
oLink.Save
