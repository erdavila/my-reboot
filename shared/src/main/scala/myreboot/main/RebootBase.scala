package myreboot.main

import myreboot.main.reboot.ArgsProgram._
import myreboot.{Display, OS, OSPlatform}

trait RebootBase {

  protected val platform: OSPlatform

  def main(args: Array[String]): Unit = {
    val result = program.run(args.toVector)

    for (msg <- result.left) {
      showUsageAndAbort(msg)
    }
  }

  private def program =
    for {
      os <- argOfType[OS]
      display <- argOfType[Display]
      _ <- noMoreArgs
    } yield {
      platform.bootOptions.set(os, display)
      platform.reboot()
    }

  private def showUsageAndAbort(msg: String): Nothing = {
    val args = s"Argumentos possíveis: [${OS.Values.map(_.code).mkString("|")}] [${Display.Values.map(_.code).mkString("|")}]"

    System.err.println(msg)
    System.err.println(args)
    System.exit(1).asInstanceOf[Nothing]
  }
}
