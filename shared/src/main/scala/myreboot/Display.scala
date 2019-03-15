package myreboot

sealed abstract class Display(val code: String) extends WithCode { def theOther: Display }
case object Monitor extends Display("monitor") { override def theOther: Display = TV }
case object TV extends Display("tv") { override def theOther: Display = Monitor }

object Display extends WithCode.Companion[Display] {
  val Values: Seq[Display] = Seq(Monitor, TV)
}
