package myreboot.main

import myreboot.{Action, Platform, Monitor, TV}

object Dialog extends DialogBase {

  override protected def platformName: String = "Linux"

  override protected def actions: Seq[Action] = Seq(
    Action("Desligar") { Platform.shutdown() },
    Action("Reiniciar") { Platform.reboot() },
    Action("Reiniciar no Windows usando o monitor") { Platform.rebootToWindows(Monitor) },
    Action("Reiniciar no Windows usando a TV") { Platform.rebootToWindows(TV) },
  )
}
