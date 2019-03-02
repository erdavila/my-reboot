package myreboot

import java.io.File

class LinuxPlatform extends Platform {

  override val name: String = "Linux"

  override val actions: Seq[Action] =
    Seq(
      Action("Desligar") { shutdown() },
      Action("Reiniciar") { reboot() },
      Action("Reiniciar no Windows usando o monitor") { rebootToWindows(Monitor) },
      Action("Reiniciar no Windows usando a TV") { rebootToWindows(TV) },
    )

  private val StateDir = new File("/boot/grub/grubenv.dir")
  private val OptionsPath = new File(StateDir, "grubenv")

  private val configs = Configs.load(StateDir)

  private def shutdown(): Unit =
    execute("systemctl", "poweroff")

  private def rebootToWindows(display: Display): Unit = {
    val bootOptions = BootOptions.load(OptionsPath.toString, configs)
    bootOptions.setOS(Windows)
    bootOptions.setWindowsDisplay(display)
    bootOptions.save()

    reboot()
  }

  private def reboot(): Unit =
    execute("systemctl", "reboot")

  private def execute(parts: String*): Unit = {
    val pb = new ProcessBuilder(parts: _*)
    val p = pb.start()
    p.waitFor()
  }
}
