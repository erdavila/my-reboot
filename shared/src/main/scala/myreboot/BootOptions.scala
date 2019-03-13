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

  def getWindowsDisplay: Option[Display] = {
    val propsFile = loadPropertiesFile()
    for {
      code <- propsFile.get(WindowsDisplayKey)
      display <- Display.byCode(code)
    } yield display
  }

  def setWindowsDisplay(display: Display): Unit = {
    val propsFile = loadPropertiesFile()
    propsFile.set(WindowsDisplayKey, display.code)
    propsFile.save()
  }

  private def loadPropertiesFile() =
    PropertiesFile.load(new File(dir, "my-reboot-options.properties"))
}
