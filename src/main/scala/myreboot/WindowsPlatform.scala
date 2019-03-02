package myreboot

import java.io.File
import scala.sys.process._

class WindowsPlatform extends Platform {

  override val name: String = "Windows"

  override val actions: Seq[Action] =
    Seq(
      Action("Desligar") { shutdown() },
      Action("Reiniciar usando o monitor") {},
      Action("Reiniciar usando a TV") {},
      Action("Reiniciar no Linux") {},
    )

  private val StateDir = new File("C:\\grubenv.dir")

  private val configs = Configs.load(StateDir)

  private def shutdown(): Unit = {
    switchDisplay(Monitor)
    Seq("shutdown", "/sg", "/t", "0").!!
  }

  private def switchDisplay(display: Display): Unit =
    if (!currentDisplay().contains(display)) {
      Seq("DisplaySwitch.exe", configs.windowsDisplaySwitchArgs(display)).!!
    }

  private def currentDisplay(): Option[Display] = {
    val output = Seq("query-display").!!
    wrapString(output).lines.toList match {
      case List("Single", _, deviceId) => configs.displayByDeviceId(deviceId)
      case _ => None
    }
  }
}
