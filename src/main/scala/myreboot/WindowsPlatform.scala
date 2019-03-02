package myreboot

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

  // TODO: this should come from some configuration
  private val display2DisplaySwitchParameter: Map[Display, String] = Map(
    TV -> "/internal",
    Monitor -> "/external",
  )

  // TODO: this should come from some configuration
  private val deviceId2Display = Map(
    raw"""\\.\DISPLAY1\Monitor0""" -> TV,
    raw"""\\.\DISPLAY1\Monitor1""" -> Monitor,
  )

  private def shutdown(): Unit = {
    switchDisplay(Monitor)
    Seq("shutdown", "/sg", "/t", "0").!!
  }

  private def switchDisplay(display: Display): Unit =
    if (!currentDisplay().contains(display)) {
      Seq("DisplaySwitch.exe", display2DisplaySwitchParameter(display)).!!
    }

  private def currentDisplay(): Option[Display] = {
    val output = Seq("query-display").!!
    wrapString(output).lines.toList match {
      case List("Single", _, deviceId) => Some(deviceId2Display(deviceId))
      case _ => None
    }
  }
}
