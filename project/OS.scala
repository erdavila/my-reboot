sealed trait OS
case object Linux extends OS
case object Windows extends OS

object OS {
  lazy val which: OS = System.getProperty("os.name") match {
    case n if n startsWith "Linux" => Linux
    case n if n startsWith "Windows" => Windows
  }
}
