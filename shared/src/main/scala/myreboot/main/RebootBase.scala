package myreboot.main

import myreboot.main.reboot.ArgsProgram._
import myreboot.main.reboot.{Action, Reboot}
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
      action <- argOfType[Action]
      _ <- noMoreArgs
    } yield {
      platform.bootOptions.set(os, display)
      (action getOrElse Reboot)(platform)
    }

  private def showUsageAndAbort(msg: String): Nothing = {
    val allowedArgs = "Argumentos possíveis: " +
      Seq(OS, Display, Action).map { _.Values.map(_.code).mkString("[", "|", "]") }.mkString(" ")

    System.err.println(msg)
    System.err.println(allowedArgs)
    System.exit(1).asInstanceOf[Nothing]
  }
}
