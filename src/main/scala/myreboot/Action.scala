package myreboot

trait Action {
  val label: String
  def apply(): Unit
}

object Action {
  def apply(label: String)(f: => Unit): Action = {
    val lbl = label
    new Action {
      override val label: String = lbl
      override def apply(): Unit = f
    }
  }
}
