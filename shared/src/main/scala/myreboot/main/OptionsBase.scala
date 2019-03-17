package myreboot.main

import myreboot.{BootOptions, WithCode}
import myreboot.{Display, OS, OSPlatform}
import myreboot.main.ArgsProgram._

trait OptionsBase {

  protected val platform: OSPlatform

  def main(args: Array[String]): Unit = {
    val (program, programArgs) = args.toVector match {
      case v@Vector() => (showProgram, v)
      case Vector("show", showArgs@_*) => (showProgram, showArgs)
      case Vector("set", setArgs@_*) => (setProgram, setArgs)
      case Vector("unset", unsetArgs@_*) => (unsetProgram, unsetArgs)
      case Vector(cmd, _@_*) => showUsageAndAbort(s"Comando desconhecido: $cmd")
    }

    val result = program.run(programArgs.toVector)

    for (msg <- result.left) {
      showUsageAndAbort(msg)
    }
  }

  private def showProgram = {
    def show(label: String)(f: BootOptions => Option[WithCode]): Unit = {
      val valueString = f(platform.bootOptions).fold("<unset>")(_.code)
      println(s"$label: $valueString")
    }

    for {
      _ <- noMoreArgs
    } yield {
      show("OS")(_.getOS)
      show("Display")(_.getWindowsDisplay)
    }
  }

  private def setProgram =
    for {
      os <- argOfType[OS]
      display <- argOfType[Display]
      _ <- noMoreArgs
    } yield {
      platform.bootOptions.set(os, display)
    }

  private def unsetProgram =
    for {
      os <- hasArg("os")
      display <- hasArg("display")
      _ <- noMoreArgs
    } yield {
      if (os) {
        platform.bootOptions.unsetOS()
      }
      if (display) {
        platform.bootOptions.unsetWindowsDisplay()
      }
    }

  private def showUsageAndAbort(msg: String): Nothing = {
    System.err.println(msg)
    System.err.println("Argumentos possíveis:")
    System.err.println("  [show]")
    System.err.println("  set " + Seq(OS, Display).map { _.Values.map(_.code).mkString("[", "|", "]") }.mkString(" "))
    System.err.println("  unset [os] [display]")
    System.exit(1).asInstanceOf[Nothing]
  }
}
