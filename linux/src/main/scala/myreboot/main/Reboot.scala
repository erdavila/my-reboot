package myreboot.main

import myreboot.{Display, OS, Platform, WithCode}

object Reboot {
  private case class Selections(os: Option[OS], display: Option[Display])

  def main(args: Array[String]): Unit = {
    val selections = args.foldLeft(Selections(None, None)) { (selections, arg) =>
      arg match {
        case OS.Code(os) =>
          validate(selections.os, os)
          selections.copy(os = Some(os))

        case Display.Code(d) =>
          validate(selections.display, d)
          selections.copy(display = Some(d))

        case _ =>
          showUsageAndAbort(s"O que é $arg?!")
      }
    }

    Platform.reboot(selections.os, selections.display)
  }

  private def validate[A <: WithCode](previousSelection: Option[A], newSelection: A): Unit =
    previousSelection match {
      case Some(sel) if sel != newSelection => showUsageAndAbort(s"${newSelection.code} ou ${sel.code}?!")
      case _ =>
    }

  private def showUsageAndAbort(msg: String): Nothing = {
    val args = s"Argumentos possíveis: [${OS.Values.map(_.code).mkString("|")}] [${Display.Values.map(_.code).mkString("|")}]"

    System.err.println(msg)
    System.err.println(args)
    System.exit(1).asInstanceOf[Nothing]
  }
}
