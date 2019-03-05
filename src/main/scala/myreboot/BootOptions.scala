package myreboot

import java.io.File

object BootOptions {
  def using(dir: File, configs: Configs): BootOptions = {
    new BootOptions(dir, configs: Configs)
  }
}

class BootOptions private(dir: File, configs: Configs) {

  private val OSKey = "saved_entry"
  private val WindowsDisplayKey = "windows.display"

  def setOS(os: OS): Unit = {
    val grubenv = Grubenv.load(new File(dir, "grubenv"))
    grubenv.set(OSKey, configs.osGrubEntry(os))
    grubenv.save()
  }

  def setWindowsDisplay(display: Display): Unit = {
    val propsFile = PropertiesFile.load(new File(dir, "my-reboot-options.properties"))
    propsFile.set(WindowsDisplayKey, display.code)
    propsFile.save()
  }
}
