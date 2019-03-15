package myreboot.main

import myreboot.main.dialog.{Action, Icon}
import myreboot._
import myreboot.utils.Implicits._

object Dialog extends DialogBase {

  override protected def platformName: String = "Windows"

  override protected def actions: Seq[Action] = Seq(
    Action("Desligar", Icon.Shutdown) {
      Platform.shutdown()
    },

    Action("Reiniciar usando o monitor", Icon.Windows) {
      Platform.bootOptions.set(Windows, Monitor)
      Platform.reboot()
    },

    Action("Reiniciar usando a TV", Icon.Windows) {
      Platform.bootOptions.set(Windows, TV)
      Platform.reboot()
    },

    Action("Reiniciar no Linux", Icon.Linux) {
      Platform.bootOptions.set(Linux)
      Platform.reboot()
    },
  )
}
