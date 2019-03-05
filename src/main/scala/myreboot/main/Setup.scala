package myreboot.main

import java.io.File
import myreboot._
import scala.annotation.tailrec
import scala.io.{Source, StdIn}
import scala.sys.process._

object Setup {

  def main(args: Array[String]): Unit = {
    val (entries, stateDir) = OS.which match {
      case Linux => (LinuxSetup.run, LinuxPlatform.StateDir)
      case Windows => (WindowsSetup.run, WindowsPlatform.StateDir)
    }

    val configsFile = new File(stateDir, Configs.FileName)
    val propsFile = PropertiesFile.load(configsFile)
    println(s"As seguintes propriedades serão salvas em ${configsFile.getPath}:")
    for ((key, value) <- entries) {
      println(s"  $key=$value")
      propsFile.set(key, value)
    }
    askToProceed()
    propsFile.save()

    println("PRONTO")
  }

  def askToProceed(): Unit = {
    StdIn.readLine("Pressione ENTER para prosseguir ")
    println()
  }
}

object LinuxSetup {
  def run: Seq[(String, String)] = {
    val grubCfgFile = new File("/boot/grub/grub.cfg")
    println(s"Lendo ${grubCfgFile.getPath}...")
    val menuentryLines = Source.fromFile(grubCfgFile).getLines().filter(_ startsWith "menuentry ").toList

    for (os <- OS.Values)
      yield {
        println("Detectando entrada com \"" + os.code + "\"...")
        val entryIndex = menuentryLines.indexWhere(_ contains os.code)
        assert(entryIndex >= 0, "Entrada não encontrada!")
        val entryNumber = 1 + entryIndex
        val key = os.code + "." + Configs.GrubEntrySubKey
        key -> entryNumber.toString
      }
  }
}

object WindowsSetup {
  def run: Seq[(String, String)] = {
    val Internal = "/internal"
    val External = "/external"

    val initialDisplay = askCurrentDisplay

    println(s"A seguir, é possível que a tela mude para ${other(initialDisplay).code}.")
    println("Se a tela atual desligar, responda a pergunta que aparecer na outra tela.")
    Setup.askToProceed()

    displaySwitch(Internal)
    val internalDisplay = askCurrentDisplay
    val Some(internalDeviceId) = WindowsPlatform.currentDeviceId()

    val externalDisplay = other(internalDisplay)
    val Some(externalDeviceId) = if (initialDisplay == internalDisplay) {
      println(s"Agora, a tela mudará por alguns segundos para ${other(internalDisplay).code}.")
      println(s"Aguarde a tela retornar para ${internalDisplay.code}")
      Setup.askToProceed()

      displaySwitch(External)
      val externalDeviceId = WindowsPlatform.currentDeviceId()

      displaySwitch(Internal)
      externalDeviceId

    } else {
      println(s"Agora, a tela retornará para ${externalDisplay.code}.")
      Setup.askToProceed()

      displaySwitch(External)
      val externalDeviceId = WindowsPlatform.currentDeviceId()

      externalDeviceId
    }

    assert(internalDeviceId != externalDeviceId, internalDeviceId)

    Seq(
      (internalDisplay.code + "." + Configs.DeviceIdSubKey) -> internalDeviceId,
      (internalDisplay.code + "." + Configs.DisplaySwitchArgSubKey) -> Internal,
      (externalDisplay.code + "." + Configs.DeviceIdSubKey) -> externalDeviceId,
      (externalDisplay.code + "." + Configs.DisplaySwitchArgSubKey) -> External,
    )
  }

  @tailrec
  private def askCurrentDisplay: Display = {
    val options = Display.Values.map('"' + _.code + '"').mkString(" ou ")
    val answer = StdIn.readLine(s"Qual é a tela atual? $options? ")

    Display.Values.find(_.code == answer) match {
      case Some(display) =>
        println()
        display
      case None =>
        askCurrentDisplay
    }
  }

  private def other(display: Display): Display =
    display match {
      case Monitor => TV
      case TV => Monitor
    }

  private def displaySwitch(arg: String): Unit = {
    Seq(WindowsPlatform.DisplaySwitchExe, arg).!!
    Thread.sleep(3000)
  }
}
