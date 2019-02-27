package myreboot

class LinuxPlatform extends Platform {

  override val name: String = "Linux"

  override val actions: Seq[Action] =
    Seq(
      Action("Desligar") {},
      Action("Reiniciar") {},
      Action("Reiniciar no Windows usando o monitor") {},
      Action("Reiniciar no Windows usando a TV") {},
    )
}
