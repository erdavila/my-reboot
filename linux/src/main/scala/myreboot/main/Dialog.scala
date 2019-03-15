package myreboot.main

import myreboot._
import myreboot.main.dialog.{Action, Icon}
import scala.language.implicitConversions

object Dialog extends DialogBase {

  override protected def platformName: String = "Linux"

  override protected def actions: Seq[Action] = Seq(
    Action("Desligar", Icon.Shutdown) { Platform.shutdown() },
    Action("Reiniciar", Icon.Linux) { Platform.reboot(Linux) },
    Action("Reiniciar no Windows usando o monitor", Icon.Windows) { Platform.reboot(Windows, Monitor) },
    Action("Reiniciar no Windows usando a TV", Icon.Windows) { Platform.reboot(Windows, TV) },
  )

  private implicit def toOption[A](a: A): Option[A] =
    Some(a)
}
