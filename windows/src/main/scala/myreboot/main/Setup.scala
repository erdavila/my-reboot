package myreboot.main

import myreboot.utils.Executor.execute
import myreboot.{Configs, Display, Platform}
import scala.annotation.tailrec
import scala.io.StdIn

object Setup extends SetupBase {

  private val Internal = "/internal"
  private val External = "/external"

  def main(args: Array[String]): Unit = {
    val initialDisplay = askCurrentDisplay

    println(s"A seguir, é possível que a tela mude para ${initialDisplay.theOther.code}.")
    println("Se a tela atual desligar, responda a pergunta que aparecer na outra tela.")
    askToProceed()

    displaySwitch(Internal)
    val internalDisplay = askCurrentDisplay
    val Some(internalDeviceId) = Platform.currentDeviceId()

    val externalDisplay = internalDisplay.theOther
    val Some(externalDeviceId) = if (initialDisplay == internalDisplay) {
      println(s"Agora, a tela mudará por alguns segundos para ${internalDisplay.theOther.code}.")
      println(s"Aguarde a tela retornar para ${internalDisplay.code}")
      askToProceed()

      displaySwitch(External)
      val externalDeviceId = Platform.currentDeviceId()

      displaySwitch(Internal)
      externalDeviceId

    } else {
      println(s"Agora, a tela retornará para ${externalDisplay.code}.")
      askToProceed()

      displaySwitch(External)
      val externalDeviceId = Platform.currentDeviceId()

      externalDeviceId
    }

    assert(internalDeviceId != externalDeviceId, internalDeviceId)

    val entries = Seq(
      (internalDisplay.code + "." + Configs.DeviceIdSubKey) -> internalDeviceId,
      (internalDisplay.code + "." + Configs.DisplaySwitchArgSubKey) -> Internal,
      (externalDisplay.code + "." + Configs.DeviceIdSubKey) -> externalDeviceId,
      (externalDisplay.code + "." + Configs.DisplaySwitchArgSubKey) -> External,
    )
    saveConfigs(entries, Platform.StateDir)
  }

  @tailrec
  private def askCurrentDisplay: Display = {
    val options = Display.Values.map('"' + _.code + '"').mkString(" ou ")
    val answer = StdIn.readLine(s"Qual é a tela atual? $options? ")

    Display.byCode(answer) match {
      case Some(display) =>
        println()
        display
      case None =>
        askCurrentDisplay
    }
  }

  private def displaySwitch(arg: String): Unit = {
    execute(Platform.DisplaySwitchExe, arg)
    Thread.sleep(3000)
  }
}
