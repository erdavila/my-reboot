package myreboot

trait WithCode {
  val code: String
}

object WithCode {
  trait Companion[A <: WithCode] {
    val Values: Seq[A]

    def byCode(code: String): Option[A] =
      Values.find(_.code == code)

    implicit val self: Companion[A] = this
  }
}
