package myreboot

import java.io.File
import myreboot.utils.Executor.execute

object Platform extends OSPlatform {
  override val StateDir = new File("/boot/grub/grubenv.dir")

  override def reboot(): Unit =
    execute("systemctl", "reboot")

  override def shutdown(): Unit =
    execute("systemctl", "poweroff")
}
