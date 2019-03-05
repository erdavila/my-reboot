package myreboot

trait Platform {
  val name: String
  val actions: Seq[Action]
}

object Platform {
  def apply(): Platform =
    OS.which match {
      case Linux => new LinuxPlatform
      case Windows => new WindowsPlatform
    }
}
