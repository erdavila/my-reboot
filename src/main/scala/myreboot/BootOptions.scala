package myreboot

object BootOptions {
  def load(path: String, configs: Configs): BootOptions =
    new BootOptions(path, Grubenv.load(path), configs: Configs)
}

class BootOptions private(path: String, grubenv: Grubenv, configs: Configs) {

  private val OSKey = "saved_entry"
  private val WindowsDisplayKey = "windows_display"

  def setOS(os: OS): Unit =
    grubenv.set(OSKey, configs.osGrubEntry(os))

  def setWindowsDisplay(display: Display): Unit =
    grubenv.set(WindowsDisplayKey, display.code)

  def save(): Unit =
    grubenv.save(path)
}
