package myreboot.main

import myreboot.{Monitor, Platform, TV}
import myreboot.main.dialog.{Action, Icon}

object Dialog extends DialogBase {

  override protected def platformName: String = "Linux"

  override protected def actions: Seq[Action] = Seq(
    Action("Desligar", Icon.Shutdown) { Platform.shutdown() },
    Action("Reiniciar", Icon.Linux) { Platform.reboot() },
    Action("Reiniciar no Windows usando o monitor", Icon.Windows) { Platform.rebootToWindows(Monitor) },
    Action("Reiniciar no Windows usando a TV", Icon.Windows) { Platform.rebootToWindows(TV) },
  )
}
