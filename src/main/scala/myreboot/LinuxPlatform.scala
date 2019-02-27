package myreboot

class LinuxPlatform extends Platform {

  override val name: String = "Linux"

  override val actions: Seq[Action] =
    Seq(
      Action("Desligar") { shutdown() },
      Action("Reiniciar") { reboot() },
      Action("Reiniciar no Windows usando o monitor") {},
      Action("Reiniciar no Windows usando a TV") {},
    )

  private def shutdown(): Unit =
    execute("systemctl", "poweroff")

  private def reboot(): Unit =
    execute("systemctl", "reboot")

  private def execute(parts: String*): Unit = {
    val pb = new ProcessBuilder(parts: _*)
    val p = pb.start()
    p.waitFor()
  }
}
