package myreboot.main

import myreboot._
import myreboot.main.dialog.{Action, Icon}
import myreboot.utils.Implicits._

object Dialog extends DialogBase {

  override protected def platformName: String = "Linux"

  override protected def actions: Seq[Action] = Seq(
    Action("Desligar", Icon.Shutdown) {
      Platform.shutdown()
    },

    Action("Reiniciar", Icon.Linux) {
      Platform.bootOptions.set(Linux)
      Platform.reboot()
    },

    Action("Reiniciar no Windows usando o monitor", Icon.Windows) {
      Platform.bootOptions.set(Windows, Monitor)
      Platform.reboot()
    },

    Action("Reiniciar no Windows usando a TV", Icon.Windows) {
      Platform.bootOptions.set(Windows, TV)
      Platform.reboot()
    },
  )
}
