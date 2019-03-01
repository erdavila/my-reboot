package myreboot

import scala.io.Source

object BootOptions {
  def load(path: String): BootOptions =
    new BootOptions(path, Grubenv.load(path))
}

class BootOptions private(path: String, grubenv: Grubenv) {

  private val OSKey = "saved_entry"
  private val WindowsDisplayKey = "windows_display"

  def setOS(os: OS): Unit =
    os match {
      case Linux => ???
      case Windows => grubenv.set(OSKey, windowsEntryNumber().toString)
    }

  private def windowsEntryNumber(): Int = {
    val Some(entryIndex) = Source.fromFile("/boot/grub/grub.cfg")
      .getLines()
      .filter(_ startsWith "menuentry ")
      .zipWithIndex
      .collectFirst {
        case (line, index) if line contains "Windows Boot Manager" => index
      }
    entryIndex + 1
  }

  def setWindowsDisplay(display: Display): Unit = {
    val displayString = display match {
      case Monitor => "monitor"
      case TV => "tv"
    }
    grubenv.set(WindowsDisplayKey, displayString)
  }

  def save(): Unit =
    grubenv.save(path)
}
