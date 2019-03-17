package myreboot.main.reboot

import myreboot.{OSPlatform, WithCode}

sealed abstract class Action(val code: String) extends WithCode { def apply(p: OSPlatform): Unit }
case object Reboot extends Action("reboot") { override def apply(p: OSPlatform): Unit = p.reboot() }
case object Shutdown extends Action("shutdown") { override def apply(p: OSPlatform): Unit = p.shutdown() }

object Action extends WithCode.Companion[Action] {
  val Values: Seq[Action] = Seq(Reboot, Shutdown)
}
