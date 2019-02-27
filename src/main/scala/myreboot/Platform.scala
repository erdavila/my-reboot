package myreboot

trait Platform {
  val name: String
  val actions: Seq[Action]
}

object Platform {
  def apply(): Platform =
    System.getProperty("os.name") match {
      case n if n.startsWith("Linux") => new LinuxPlatform
    }
}
