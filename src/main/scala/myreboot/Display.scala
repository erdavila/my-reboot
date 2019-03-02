package myreboot

sealed abstract class Display(val code: String) extends WithCode
case object Monitor extends Display("monitor")
case object TV extends Display("tv")

object Display {
  val Values: Seq[Display] = Seq(Monitor, TV)
}
