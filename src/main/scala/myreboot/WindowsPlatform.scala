package myreboot

class WindowsPlatform extends Platform {

  override val name: String = "Windows"

  override val actions: Seq[Action] =
    Seq(
      Action("Desligar") {},
      Action("Reiniciar usando o monitor") {},
      Action("Reiniciar usando a TV") {},
      Action("Reiniciar no Linux") {},
    )
}
