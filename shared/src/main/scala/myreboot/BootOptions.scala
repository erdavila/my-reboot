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

  def set(os: Option[OS] = None, display: Option[Display] = None): Unit = {
    for (os <- os) {
      setOS(os)
    }

    for (display <- display) {
      setWindowsDisplay(display)
    }
  }

  def getOS: Option[OS] =
    for {
      entry <- loadGrubenv().get(OSKey)
      os <- configs.osByEntry(entry)
    } yield os

  private def setOS(os: OS): Unit = {
    val grubenv = loadGrubenv()
    grubenv.set(OSKey, configs.osGrubEntry(os))
    grubenv.save()
  }

  def unsetOS(): Unit = {
    val grubenv = loadGrubenv()
    grubenv.unset(OSKey)
    grubenv.save()
  }

  private def loadGrubenv() =
    Grubenv.load(new File(dir, "grubenv"))

  def getWindowsDisplay: Option[Display] =
    for {
      code <- loadPropertiesFile().get(WindowsDisplayKey)
      display <- Display.byCode(code)
    } yield display

  private def setWindowsDisplay(display: Display): Unit = {
    val propsFile = loadPropertiesFile()
    propsFile.set(WindowsDisplayKey, display.code)
    propsFile.save()
  }

  def unsetWindowsDisplay(): Unit = {
    val propsFile = loadPropertiesFile()
    propsFile.unset(WindowsDisplayKey)
    propsFile.save()
  }

  private def loadPropertiesFile() =
    PropertiesFile.load(new File(dir, "my-reboot-options.properties"))
}
