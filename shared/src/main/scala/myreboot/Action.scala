package myreboot

trait Action {
  val label: String
  val icon: Icon
  def apply(): Unit
}

object Action {
  def apply(label: String, icon: Icon)(f: => Unit): Action = {
    val lbl = label
    val icn = icon
    new Action {
      override val label: String = lbl
      override val icon: Icon = icn
      override def apply(): Unit = f
    }
  }
}
