package myreboot

import java.io.File
import scala.sys.process._

object Platform {
  val StateDir = new File("/boot/grub/grubenv.dir")

  private val configs = Configs.load(StateDir)

  private lazy val bootOptions: BootOptions = BootOptions.using(StateDir, configs)

  def shutdown(): Unit =
    Seq("systemctl", "poweroff").!!

  def reboot(os: Option[OS] = None, display: Option[Display] = None): Unit = {
    for (os <- os) {
      Platform.bootOptions.setOS(os)
    }

    for (display <- display) {
      Platform.bootOptions.setWindowsDisplay(display)
    }

    Seq("systemctl", "reboot").!!
  }
}
