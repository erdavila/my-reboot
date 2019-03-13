package myreboot

sealed abstract class OS(val code: String) extends WithCode
case object Linux extends OS("linux")
case object Windows extends OS("windows")

object OS {
  val Values: Seq[OS] = Seq(Linux, Windows)
}
