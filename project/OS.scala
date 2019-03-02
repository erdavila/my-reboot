import sbt._

sealed trait OS
case object Linux extends OS
case object Windows extends OS

object OS {
  lazy val which: OS = System.getProperty("os.name") match {
    case n if n startsWith "Linux" => Linux
    case n if n startsWith "Windows" => Windows
  }

  def linuxOnly[T](task: Def.Initialize[Task[T]]): Def.Initialize[Task[T]] =
    if (which == Linux) task
    else Def.task { scala.sys.error("Task available only on Linux!") }

  def windowsOnly[T](task: Def.Initialize[Task[T]]): Def.Initialize[Task[T]] =
    if (which == Windows) task
    else Def.task { scala.sys.error("Task available only on Windows!") }

  def select(f: OS => Def.Initialize[Task[Unit]]): Def.Initialize[Task[Unit]] =
    f(which)
}
