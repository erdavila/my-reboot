package myreboot

import java.io.File
import scala.sys.process._

object Platform extends OSPlatform {
  override val StateDir = new File("/boot/grub/grubenv.dir")

  override def reboot(): Unit =
    Seq("systemctl", "reboot").!!

  def shutdown(): Unit =
    Seq("systemctl", "poweroff").!!
}
