package myreboot

import java.io.File
import scala.sys.process._
import windowsapi.scala.User32

object WindowsPlatform {
  val StateDir = new File("C:\\grubenv.dir")

  val DisplaySwitchExe = "DisplaySwitch.exe"

  def currentDeviceId(): Option[String] = {
    val activeDisplayDevices = for {
      adapter <- User32.enumDisplayDevices().toList
      displayDevice <- User32.enumDisplayDevices(adapter.name)
      if displayDevice.active
    } yield displayDevice

    activeDisplayDevices match {
      case List(displayDevice) => Some(displayDevice.id)
      case _ => None
    }
  }
}

class WindowsPlatform extends Platform {
  import WindowsPlatform._

  override val name: String = "Windows"

  override val actions: Seq[Action] =
    Seq(
      Action("Desligar") { shutdown() },
      Action("Reiniciar usando o monitor") {},
      Action("Reiniciar usando a TV") {},
      Action("Reiniciar no Linux") {},
    )

  private val configs = Configs.load(StateDir)

  private def shutdown(): Unit = {
    switchDisplay(Monitor)
    Seq("shutdown", "/sg", "/t", "0").!!
  }

  private def switchDisplay(display: Display): Unit =
    if (!currentDisplay().contains(display)) {
      Seq(DisplaySwitchExe, configs.windowsDisplaySwitchArgs(display)).!!
    }

  private def currentDisplay(): Option[Display] =
    for {
      deviceId <- currentDeviceId()
      display <- configs.displayByDeviceId(deviceId)
    } yield display
}
