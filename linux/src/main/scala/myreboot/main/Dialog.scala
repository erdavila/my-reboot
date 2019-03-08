package myreboot.main

import myreboot.{Action, LinuxPlatform, Monitor, TV}

object Dialog extends DialogBase {

  override protected def platformName: String = "Linux"

  override protected def actions: Seq[Action] = Seq(
    Action("Desligar") { LinuxPlatform.shutdown() },
    Action("Reiniciar") { LinuxPlatform.reboot() },
    Action("Reiniciar no Windows usando o monitor") { LinuxPlatform.rebootToWindows(Monitor) },
    Action("Reiniciar no Windows usando a TV") { LinuxPlatform.rebootToWindows(TV) },
  )
}
