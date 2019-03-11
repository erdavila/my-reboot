package myreboot.main

import myreboot.main.dialog.{Action, Icon}
import myreboot.{Monitor, Platform, TV}

object Dialog extends DialogBase {

  override protected def platformName: String = "Windows"

  override protected def actions: Seq[Action] = Seq(
    Action("Desligar", Icon.Shutdown) { Platform.shutdown() },
    Action("Reiniciar usando o monitor", Icon.Windows) { Platform.reboot(Monitor) },
    Action("Reiniciar usando a TV", Icon.Windows) { Platform.reboot(TV) },
    Action("Reiniciar no Linux", Icon.Linux) {},
  )
}
