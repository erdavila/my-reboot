package myreboot

import java.io.File
import scala.sys.process._

object Platform {
  val StateDir = new File("/boot/grub/grubenv.dir")

  private val configs = Configs.load(StateDir)

  def shutdown(): Unit =
    Seq("systemctl", "poweroff").!!

  def rebootToWindows(display: Display): Unit = {
    val bootOptions = BootOptions.using(StateDir, configs)
    bootOptions.setOS(Windows)
    bootOptions.setWindowsDisplay(display)

    reboot()
  }

  def reboot(): Unit =
    Seq("systemctl", "reboot").!!
}
