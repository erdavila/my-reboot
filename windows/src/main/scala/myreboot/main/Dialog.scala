package myreboot.main

import myreboot.main.dialog.{Action, Icon}
import myreboot.Platform

object Dialog extends DialogBase {

  override protected def platformName: String = "Windows"

  override protected def actions: Seq[Action] = Seq(
    Action("Desligar", Icon.Shutdown) { Platform.shutdown() },
    Action("Reiniciar usando o monitor", Icon.Windows) {},
    Action("Reiniciar usando a TV", Icon.Windows) {},
    Action("Reiniciar no Linux", Icon.Linux) {},
  )
}
