package myreboot.utils

import scala.language.implicitConversions

object Implicits {
  implicit def toOption[A](a: A): Option[A] =
    Some(a)
}
