package myreboot.main

import myreboot.{BootOptions, Display, Platform}

object SwitchDisplay {

  def main(args: Array[String]): Unit = {
    val display = args.headOption match {
      case Some("saved") => savedDisplay().getOrElse(scala.sys.error("Nenhuma tela salva!"))
      case Some(code) => Display.byCode(code).getOrElse(exitWithHelp(s"Tela desconhecida: $code!"))
      case None => Platform.currentDisplay().getOrElse(exitWithHelp("Não é possível identificar a tela atual!")).theOther
    }

    Platform.switchDisplay(display)
  }

  private def savedDisplay(): Option[Display] = {
    val bootOptions = BootOptions.using(Platform.StateDir, Platform.configs)
    bootOptions.getWindowsDisplay
  }

  private def exitWithHelp(message: String): Nothing = {
    System.err.println(message)
    System.err.println()
    System.err.println("Possíveis parâmetros:")
    System.err.println("  saved - Muda para a tela que foi salva nas opções")
    for (display <- Display.Values) {
      System.err.println(s"  ${display.code} - Muda a tela para ${display.code}")
    }
    System.err.println("  NENHUM PARÂMETRO - Muda para a outra tela")

    sys.exit(1)
  }
}
